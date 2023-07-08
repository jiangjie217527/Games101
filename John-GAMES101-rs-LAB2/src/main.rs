mod aabb;
mod rasterizer;
mod triangle;
mod utils;

extern crate opencv;
use crate::rasterizer::{Primitive, Rasterizer};
use nalgebra::Vector3;
use opencv::core::Vector;
use opencv::highgui::{imshow, wait_key};
use opencv::imgcodecs::imwrite;
use opencv::Result;
use std::io;
use std::time::Instant;
use utils::*;

fn main() -> Result<()> {
    let mut r = Rasterizer::new(700, 700);
    let eye_pos = Vector3::new(0.0, 0.0, 5.0);
    let pos = vec![
        Vector3::new(2.0, 0.0, -2.0),
        Vector3::new(0.0, 2.0, -2.0),
        Vector3::new(-2.0, 0.0, -2.0),
        Vector3::new(3.5, -1.0, -5.0),
        Vector3::new(2.5, 1.5, -5.0),
        Vector3::new(-1.0, 0.5, -5.0),
        Vector3::new(-3.5, -3.5, -6.0),
        Vector3::new(3.5, 1.5, -6.0),
        Vector3::new(-2.0, 2.5, -6.0),
    ];
    let ind = vec![
        Vector3::new(0, 1, 2),
        Vector3::new(3, 4, 5),
        Vector3::new(6, 7, 8),
    ];
    let cols = vec![
        Vector3::new(217.0, 238.0, 185.0),
        Vector3::new(217.0, 238.0, 185.0),
        Vector3::new(217.0, 238.0, 185.0),
        Vector3::new(185.0, 217.0, 238.0),
        Vector3::new(185.0, 217.0, 238.0),
        Vector3::new(185.0, 217.0, 238.0),
        Vector3::new(238.0, 185.0, 217.0),
        Vector3::new(238.0, 185.0, 217.0),
        Vector3::new(238.0, 185.0, 217.0),
    ];
    let pos_id = r.load_position(&pos);
    let ind_id = r.load_indices(&ind);
    let col_id = r.load_colors(&cols);
    let mut k = 0;
    let mut frame_count = 0;
    let mut str = String::new();
    println!("输入r并回车来允许旋转，其他任意输入禁止旋转");
    io::stdin().read_line(&mut str).expect("fail");
    let mut angle = 180.0;
    let command_line = false;
    while k != 27 {
        let now = Instant::now();
        r.clear(rasterizer::Buffer::Both);
        r.set_model(get_model_matrix(angle, str.clone()));
        r.set_view(get_view_matrix(eye_pos));
        r.set_projection(get_projection_matrix(45.0, 1.0, 0.1, 50.0));
        r.draw(pos_id, ind_id, col_id, Primitive::Triangle);

        let frame_buffer = r.frame_buffer();
        let image = frame_buffer2cv_mat(frame_buffer);
        if command_line {
            imwrite("output.png", &image, &Vector::default()).unwrap();
            return Ok(());
        }
        imshow("image", &image)?;
        println!(
            "FXAA(simple)：frame count: {},time consume:{}毫秒",
            frame_count,
            now.elapsed().as_millis()
        );
        frame_count += 1;
        k = wait_key(2000).unwrap();
        if k == 'a' as i32 {
            angle += 10.0;
        } else if k == 'd' as i32 {
            angle -= 10.0;
        }
    }

    Ok(())
}
