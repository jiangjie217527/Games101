// #![allow(warnings)]

mod aabb;
mod rasterizer;
mod shader;
mod texture;
mod triangle;
mod utils;

extern crate opencv;

use crate::rasterizer::{Buffer, Rasterizer};
use crate::shader::FragmentShaderPayload;
use crate::texture::Texture;
use nalgebra::Vector3;
use opencv::core::Vector;
use opencv::Result;
use std::env;
use utils::*;

/**
 * normal-> 默认
 * phong -> Blinn-Phong光照
 * texture -> 正常纹理
 * bump -> 凹凸材质
 * displacement ->
 */
fn main() -> Result<()> {
    // let obj_file = "./models/spot/cube.obj";  cube只能做0，1，2
    let obj_file = "./models/spot/spot_triangulated_good.obj";
    let triangles = load_triangles(&obj_file);
    println!("load successfully");

    let angle = 140.0;
    let mut r = Rasterizer::new(700, 700);
    let obj_path = "./models/spot/".to_owned();
    let mut filename = "output.png".to_owned();
    let texture_path = "hmap.jpg".to_owned();
    let mut tex = Texture::new(&(obj_path.clone() + &texture_path));
    let mut active_shader: fn(&FragmentShaderPayload) -> Vector3<f64> = normal_fragment_shader; // 默认为<normal shader>
    let ags: Vec<String> = env::args().collect();
    //输出文件名
    if ags.len() >= 2 {
        filename = ags[1].clone();
    }
    //渲染方式
    if ags.len() >= 3 {
        let (shader, t) = choose_shader_texture(&ags[2], &obj_path);
        active_shader = shader;
        if let Some(tx) = t {
            tex = tx;
        }
    }
    r.set_texture(tex);

    let eye_pos = Vector3::new(0.0, 0.0, 10.0);
    r.set_vertex_shader(vertex_shader);
    r.set_fragment_shader(active_shader);

    r.clear(Buffer::Both);
    r.set_model(get_model_matrix(angle));
    r.set_view(get_view_matrix(eye_pos));
    r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));

    r.draw(&triangles);

    let image = frame_buffer2cv_mat(r.frame_buffer());
    let v: Vector<i32> = Default::default();

    opencv::imgcodecs::imwrite(&filename, &image, &v).unwrap();
    Ok(())
}
