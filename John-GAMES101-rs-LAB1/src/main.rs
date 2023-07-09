mod rasterizer;
mod triangle;
mod utils;
extern crate opencv;
use crate::rasterizer::{Primitive, Rasterizer};
use nalgebra::Vector3;
use opencv::core::Vector;
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::imwrite;
use std::env;
use std::io;
use utils::*;

fn main() {
    let mut angle = 0.0;

    let mut command_line = false;
    let mut filename = "output.png";
    let argv: Vec<String> = env::args().collect();
    if argv.len() >= 2 {
        command_line = true;
        angle = argv[1].parse().unwrap();
        if argv.len() == 3 {
            filename = &argv[2];
        }
    }
    //raster
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);

    let pos = vec![
        Vector3::new(2.0, 0.0, -2.0),
        Vector3::new(0.0, 2.0, -2.0),
        Vector3::new(-2.0, 0.0, -2.0),
        Vector3::new(0.0, 0.0, 0.0),
        Vector3::new(1.0, 1.0, 0.0),
        Vector3::new(-1.0, 0.0, 0.0),
    ];
    let ind = vec![Vector3::new(0, 1, 2)];

    let pos_id = r.load_position(&pos); //存储了所有存在的点，即点缓存
    let ind_id = r.load_indices(&ind); //每个物体使用到的点

    let mut k = 0; //key value
                   // let mut frame_count = 0;

    //是否绕任意轴旋转
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("fail");
    //output file
    //move with key
    while k != 27 {
        r.clear(rasterizer::Buffer::Both);
        r.set_model(get_model_matrix(angle, str.clone()));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw_triangle(pos_id, ind_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer(); //framebuffer为图像
        let image = frame_buffer2cv_mat(frame_buffer);
        //output file
        if command_line {
            imwrite(filename, &image, &Vector::default()).unwrap();
            return;
        }
        imshow("image", &image).unwrap();

        k = wait_key(80).unwrap();
        //println!("frame count: {}", frame_count);
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        }
        // frame_count += 1;
    }
}
