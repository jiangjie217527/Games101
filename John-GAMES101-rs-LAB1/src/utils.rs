use std::os::raw::c_void;
use nalgebra::{Matrix4, Vector3};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{COLOR_RGB2BGR, cvt_color};

pub type V3d = Vector3<f64>;

pub fn tran_ang_to_rad(a:f64)->f64{
    std::f64::consts::PI/180.0*a
}

pub fn matrix4_info(m:Matrix4<f64>){
    for i in 0..4{
        for j in 0..4{
            print!("{},",m[(i,j)]);
        }
        println!("");
    }
    println!("----------");
}

pub(crate) fn get_view_matrix(eye_pos: V3d) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */

    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64) -> Matrix4<f64> {
    let mut model: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */
    let rad = tran_ang_to_rad(rotation_angle);
    model.m44 = 1.0;
    model.m33 = 1.0;
    model.m11 = rad.cos();
    model.m22 = rad.cos();
    model.m12 = -rad.sin();
    model.m21 = rad.sin();
    matrix4_info(model);
    model
}

pub(crate) fn get_projection_matrix(eye_fov: f64, aspect_ratio: f64, z_near: f64, z_far: f64) -> Matrix4<f64> {
    let mut projection: Matrix4<f64> = Matrix4::identity();
    /*  implement your code here  */

    projection
}

pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<V3d>) -> opencv::core::Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700, 700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        ).unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image.convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0).expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}