use nalgebra::Vector3;
use opencv::core::{MatTraitConst, VecN};
use opencv::imgcodecs::{imread, IMREAD_COLOR};

pub struct Texture {
    pub img_data: opencv::core::Mat,
    pub width: usize,
    pub height: usize,
}

impl Texture {
    pub fn new(name: &str) -> Self {
        let img_data = imread(name, IMREAD_COLOR).expect("Image reading error!");
        let width = img_data.cols() as usize;
        let height = img_data.rows() as usize;
        Texture {
            img_data,
            width,
            height,
        }
    }

    // pub fn get_color(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
    //     (u,v) = format_uv(u, v);

    //     let u_img = u * self.width as f64;
    //     let v_img = (1.0 - v) * self.height as f64;
    //     let color: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();

    //     Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
    // }

    pub fn get_color_bilinear(&self, mut u: f64, mut v: f64) -> Vector3<f64> {
        // 在此实现双线性插值函数, 并替换掉get_color
        (u, v) = format_uv(u, v);
        let u_img = u * self.width as f64;
        let v_img = (1.0 - v) * self.height as f64;
        let color00: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32).unwrap();
        let color01: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32 + 1, u_img as i32).unwrap();
        let color10: &VecN<u8, 3> = self.img_data.at_2d(v_img as i32, u_img as i32 + 1).unwrap();
        let color11: &VecN<u8, 3> = self
            .img_data
            .at_2d(v_img as i32 + 1, u_img as i32 + 1)
            .unwrap();

        let s = u_img - (u_img as i32) as f64;
        let tmp0 = Vector3::new(color00[0] as f64, color00[1] as f64, color00[2] as f64);
        let tmp1 = Vector3::new(color10[0] as f64, color10[1] as f64, color10[2] as f64);
        let u0 = lerp(s, &tmp0, &tmp1);
        let tmp2 = Vector3::new(color01[0] as f64, color01[1] as f64, color01[2] as f64);
        let tmp3 = Vector3::new(color11[0] as f64, color11[1] as f64, color11[2] as f64);
        let u1 = lerp(s, &tmp2, &tmp3);

        let t = v_img - (v_img as i32) as f64;
        let color = lerp(t, &u0, &u1);
        // the same as above method
        Vector3::new(color[2] as f64, color[1] as f64, color[0] as f64)
        // self.get_color(u, v)
    }
}

fn format_uv(mut u: f64, mut v: f64) -> (f64, f64) {
    if u < 0.0 {
        u = 0.0;
    }
    if u > 1.0 {
        u = 1.0;
    }
    if v < 0.0 {
        v = 0.0;
    }
    if v > 1.0 {
        v = 1.0;
    }
    (u, v)
}

fn lerp(x: f64, v0: &Vector3<f64>, v1: &Vector3<f64>) -> Vector3<f64> {
    v0 + x * (v1 - v0)
}
