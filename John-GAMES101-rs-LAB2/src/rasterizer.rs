pub use crate::aabb::Aabb;
use crate::{
    aabb::{fabs, fmax, fmin},
    triangle::Triangle,
};
use nalgebra::{Matrix4, Vector2, Vector3, Vector4};
use std::collections::HashMap;

#[allow(dead_code)]
pub enum Buffer {
    Color,
    Depth,
    Both,
}

#[allow(dead_code)]
pub enum Primitive {
    Line,
    Triangle,
}

#[derive(Default, Clone)]
pub struct Rasterizer {
    model: Matrix4<f64>,
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    pos_buf: HashMap<usize, Vec<Vector3<f64>>>,
    ind_buf: HashMap<usize, Vec<Vector3<usize>>>,
    col_buf: HashMap<usize, Vec<Vector3<f64>>>,

    frame_buf: Vec<Vector3<f64>>,
    depth_buf: Vec<f64>,
    /*  You may need to uncomment here to implement the MSAA method  */
    frame_sample: Vec<Vector3<f64>>,
    depth_sample: Vec<f64>,

    width: u64,
    height: u64,
    next_id: usize,
}

#[derive(Clone, Copy)]
pub struct PosBufId(usize);

#[derive(Clone, Copy)]
pub struct IndBufId(usize);

#[derive(Clone, Copy)]
pub struct ColBufId(usize);

impl Rasterizer {
    pub fn new(w: u64, h: u64) -> Self {
        let mut r = Rasterizer::default();
        r.width = w;
        r.height = h;
        r.frame_buf.resize((w * h * 4) as usize, Vector3::zeros());
        r.depth_buf.resize((w * h * 4) as usize, 0.0);
        r.frame_sample
            .resize((w * h * 4) as usize, Vector3::zeros());
        r.depth_sample.resize((w * h * 4) as usize, 0.0);
        r
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        //先行后列
        ((self.height - 1 - y as u64) * self.width + x as u64) as usize
    }

    fn get_sample_index(&self, x: usize, y: usize, x_offset: usize, y_offset: usize) -> usize {
        (self.height as usize * 2 - 1 - y * 2 - y_offset) * self.width as usize * 2
            + x * 2
            + x_offset
    }

    fn set_pixel(&mut self, point: &Vector3<f64>, color: &Vector3<f64>) {
        let ind = (self.height as f64 - 1.0 - point.y) * self.width as f64 + point.x;
        self.frame_buf[ind as usize] = *color;
    }

    fn set_sample_pixel(
        &mut self,
        x: usize,
        y: usize,
        x_offset: usize,
        y_offset: usize,
        color: &Vector3<f64>,
    ) {
        let index = self.get_sample_index(x, y, x_offset, y_offset) as usize;
        self.frame_sample[index] = *color;
    }

    pub fn clear(&mut self, buff: Buffer) {
        match buff {
            Buffer::Color => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
            }
            Buffer::Depth => {
                self.depth_buf.fill(f64::MAX);
            }
            Buffer::Both => {
                self.frame_buf.fill(Vector3::new(0.0, 0.0, 0.0));
                self.depth_buf.fill(f64::MAX);
                self.frame_sample.fill(Vector3::zeros());
                self.depth_sample.fill(f64::MAX);
            }
        }
    }

    pub fn set_model(&mut self, model: Matrix4<f64>) {
        self.model = model;
    }

    pub fn set_view(&mut self, view: Matrix4<f64>) {
        self.view = view;
    }

    pub fn set_projection(&mut self, projection: Matrix4<f64>) {
        self.projection = projection;
    }

    fn get_next_id(&mut self) -> usize {
        let res = self.next_id;
        self.next_id += 1;
        res
    }

    pub fn load_position(&mut self, positions: &Vec<Vector3<f64>>) -> PosBufId {
        let id = self.get_next_id();
        self.pos_buf.insert(id, positions.clone());
        PosBufId(id)
    }

    pub fn load_indices(&mut self, indices: &Vec<Vector3<usize>>) -> IndBufId {
        let id = self.get_next_id();
        self.ind_buf.insert(id, indices.clone());
        IndBufId(id)
    }

    pub fn load_colors(&mut self, colors: &Vec<Vector3<f64>>) -> ColBufId {
        let id = self.get_next_id();
        self.col_buf.insert(id, colors.clone());
        ColBufId(id)
    }

    pub fn draw(
        &mut self,
        pos_buffer: PosBufId,
        ind_buffer: IndBufId,
        col_buffer: ColBufId,
        _typ: Primitive,
    ) {
        let command = false;

        let buf = &self.clone().pos_buf[&pos_buffer.0];
        let ind: &Vec<Vector3<usize>> = &self.clone().ind_buf[&ind_buffer.0];
        let col = &self.clone().col_buf[&col_buffer.0];

        let f1 = (50.0 - 0.1) / 2.0;
        let f2 = (50.0 + 0.1) / 2.0;
        //model 模型变换（旋转）
        //view  视图变换
        //projection 把模型顶点(旋转后的世界坐标下)投影成二维（获得光栅器上的坐标）
        let mvp = self.projection * self.view * self.model;

        for i in ind {
            let mut t = Triangle::new();
            let mut v = vec![
                mvp * to_vec4(buf[i[0]], Some(1.0)), // homogeneous coordinates
                mvp * to_vec4(buf[i[1]], Some(1.0)),
                mvp * to_vec4(buf[i[2]], Some(1.0)),
            ]; //根据上述对MVP(model,view,projection)的分析，得到的点其实是在二维平面上

            for vec in v.iter_mut() {
                *vec = *vec / vec.w;
            } //归一化

            for vert in v.iter_mut() {
                vert.x = 0.5 * self.width as f64 * (vert.x + 1.0);
                vert.y = 0.5 * self.height as f64 * (vert.y + 1.0); //[-1,1]换到[0,1]
                vert.z = vert.z * f1 + f2; //??
            }

            for j in 0..3 {
                t.set_vertex(j, v[j].xyz());
            }

            let col = col[i[0]];
            t.set_color(0, col[0], col[1], col[2]);
            //only the  color of the first vertex was need
            //反走样：先模糊再采样
            if command {
                //control the process
                self.msaa_rasterize_triangle(&t);
            } else {
                self.fxaa_rasterize_triangle(&t);
            }
        }
        if !command {
            self.fxaa_process();
        }
    }

    pub fn msaa_rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */
        //传入AABB
        let aabb = Aabb::new(t);
        //对于所有像素，判断是不是在三角形里面
        for x in aabb.minimum.x..=aabb.maximum.x {
            for y in aabb.minimum.y..=aabb.maximum.y {
                let mut flg = false; //check whether one of the sample will uodate the pixel
                for x_offset in 0..2 {
                    for y_offset in 0..2 {
                        let xx = x as f64 + 0.25 + x_offset as f64 * 0.5;
                        let yy = y as f64 + 0.25 + x_offset as f64 * 0.5;
                        if inside_triangle(xx as f64, yy as f64, &t.v) {
                            //计算重心
                            //为什么插值->顶点处的值
                            //使用重心坐标插值
                            //三角形的坐标系统
                            //(x,y) = a A+ B b + C c
                            //a+b+c = 1  内部非负
                            //default
                            let (c1, c2, c3) = compute_barycentric2d(x as f64, y as f64, &t.v);
                            let z_interpolated = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                            let index = self.get_sample_index(x, y, x_offset, y_offset) as usize;
                            //z-buffer
                            if z_interpolated < self.depth_buf[index] {
                                flg = true;
                                self.depth_buf[index] = z_interpolated; //这里不需要set
                            }
                        }
                    }
                }
                if flg {
                    let mut color = Vector3::zeros();
                    for x_offset in 0..2 {
                        for y_offset in 0..2 {
                            color += self.frame_sample
                                [self.get_sample_index(x, y, x_offset, y_offset)]
                                / 4.0;
                        }
                    }
                    self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &color);
                }
            }
        }
    }

    //计划写一个最简单的FXAA
    pub fn fxaa_rasterize_triangle(&mut self, t: &Triangle) {
        /*  implement your code here  */
        //传入AABB
        let aabb = Aabb::new(t);
        //对于所有像素，判断是不是在三角形里面
        for x in aabb.minimum.x..=aabb.maximum.x {
            for y in aabb.minimum.y..=aabb.maximum.y {
                //这里同正常的一样
                if inside_triangle(x as f64, y as f64, &t.v) {
                    let (c1, c2, c3) = compute_barycentric2d(x as f64, y as f64, &t.v);
                    let z_interpolated = c1 * t.v[0].z + c2 * t.v[1].z + c3 * t.v[2].z;
                    let index = self.get_index(x, y);
                    if z_interpolated < self.depth_buf[index] {
                        self.depth_buf[index] = z_interpolated;
                        self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &t.get_color());
                    }
                }
            }
        }
    }
    //参考https://zhuanlan.zhihu.com/p/373379681 FXAA算法演义
    pub fn fxaa_process(&mut self) {
        for x in 1..self.height - 1 {
            for y in 1..self.width - 1 {
                let luma_contrast = self.compute_luma_contrast(x, y);
                if luma_contrast[5] > 0.05 {
                    let mut delta: [f64; 4] = [0.0; 4];
                    for i in 0..4 {
                        delta[i] = luma_contrast[i] - luma_contrast[4];
                    }
                    //水平方向的平均值
                    let v = fabs(delta[0] + delta[1]);
                    //垂直方向的平均值
                    let h = fabs(delta[2] + delta[3]);

                    //确定法线
                    let mut norm = Vector2::zeros();
                    //水平方向是法线
                    if v > h {
                        norm.x = sign(fabs(delta[0]) - fabs(delta[1]));
                    } else {
                        norm.y = sign(fabs(delta[2]) - fabs(delta[3]));
                    }

                    let finalcolor = (self.frame_buf[self
                        .get_index((x as f64 + norm.x) as usize, (y as f64 + norm.y) as usize)]
                        + self.frame_buf[self.get_index(x as usize, y as usize)])
                        / 2.0;
                    self.set_pixel(&Vector3::new(x as f64, y as f64, 0.0), &finalcolor);
                }
            }
        }
    }

    //计算对比度
    /**
     * 0->上
     * 1->下
     * 2->右
     * 3->左
     */
    pub fn compute_luma_contrast(&self, x: u64, y: u64) -> [f64; 6] {
        let v: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        let mut tmp: [f64; 5] = [0.0; 5];
        tmp[4] = get_luma(self.frame_buf[self.get_index(x as usize, y as usize)]);
        let mut mx = tmp[4];
        let mut mn = tmp[4];
        for i in 0..4 {
            tmp[i] = get_luma(
                self.frame_buf
                    [self.get_index((x as i32 + v[i].0) as usize, (y as i32 + v[i].1) as usize)],
            );
            mx = fmax(mx, tmp[i]);
            mn = fmin(mn, tmp[i]);
        }
        [tmp[0], tmp[1], tmp[2], tmp[3], tmp[4], mx - mn]
    }

    pub fn frame_buffer(&self) -> &Vec<Vector3<f64>> {
        &self.frame_buf
    }
}
pub fn get_luma(c: Vector3<f64>) -> f64 {
    (c[0] + c[1] + c[2]) as f64 / 3.0
}
pub fn to_vec4(v3: Vector3<f64>, w: Option<f64>) -> Vector4<f64> {
    Vector4::new(v3.x, v3.y, v3.z, w.unwrap_or(1.0))
}

fn inside_triangle(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> bool {
    /*  implement your code here  */
    //come from intro

    //The cross product tells you that a point is to the left or right of a line,
    //and if the point is on the same side of all three lines of the triangle, then
    //the point is inside the triangle.
    //v[(i + 1) %3] - v[i] 表示三角形某条边的向量
    let p = Vector3::new(x, y, 0.0); //采样点
    let side1 = (v[1] - v[0]).cross(&(p - v[0]));
    let side2 = (v[2] - v[1]).cross(&(p - v[1]));
    let side3 = (v[0] - v[2]).cross(&(p - v[2]));
    (side1.z < 0.0 && side2.z < 0.0 && side3.z < 0.0)
        || (side1.z > 0.0 && side2.z > 0.0 && side3.z > 0.0)
}

fn compute_barycentric2d(x: f64, y: f64, v: &[Vector3<f64>; 3]) -> (f64, f64, f64) {
    let c1 = (x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * y + v[1].x * v[2].y - v[2].x * v[1].y)
        / (v[0].x * (v[1].y - v[2].y) + (v[2].x - v[1].x) * v[0].y + v[1].x * v[2].y
            - v[2].x * v[1].y);
    let c2 = (x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * y + v[2].x * v[0].y - v[0].x * v[2].y)
        / (v[1].x * (v[2].y - v[0].y) + (v[0].x - v[2].x) * v[1].y + v[2].x * v[0].y
            - v[0].x * v[2].y);
    let c3 = (x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * y + v[0].x * v[1].y - v[1].x * v[0].y)
        / (v[2].x * (v[0].y - v[1].y) + (v[1].x - v[0].x) * v[2].y + v[0].x * v[1].y
            - v[1].x * v[0].y);
    (c1, c2, c3)
}

pub fn sign(x: f64) -> f64 {
    if x > 0.0 {
        1.0
    } else {
        -1.0
    }
}
