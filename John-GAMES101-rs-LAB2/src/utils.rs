use nalgebra::{Matrix4, Vector3};
use opencv::core::{Mat, MatTraitConst};
use opencv::imgproc::{cvt_color, COLOR_RGB2BGR};
use std::os::raw::c_void;

pub fn get_rotation(axis: Vector3<f64>, angle: f64) -> Matrix4<f64> {
    let rad = tran_ang_to_rad(angle);
    let mut arbitrary_rotation = Matrix4::identity() * rad.cos();
    let t3 = axis * axis.transpose() * (1.0 - rad.cos());
    let t4 = t3.to_homogeneous();
    arbitrary_rotation += t4;
    let tmp = Matrix4::new(
        0.0, -axis.z, axis.y, 0.0, axis.z, 0.0, -axis.x, 0.0, -axis.y, axis.x, 0.0, 0.0, 0.0, 0.0,
        0.0, 1.0,
    );
    arbitrary_rotation += tmp * rad.sin();
    arbitrary_rotation.m44 = 1.0;
    arbitrary_rotation
}

pub fn tran_ang_to_rad(a: f64) -> f64 {
    std::f64::consts::PI / 180.0 * a
}

pub(crate) fn get_view_matrix(eye_pos: Vector3<f64>) -> Matrix4<f64> {
    let mut view: Matrix4<f64> = Matrix4::identity();
    /*  implement what you've done in LAB1  */
    view.m14 = -eye_pos.x;
    view.m24 = -eye_pos.y;
    view.m34 = -eye_pos.z;
    view
}

pub(crate) fn get_model_matrix(rotation_angle: f64, str: String) -> Matrix4<f64> {
    // let mut model: Matrix4<f64> = Matrix4::identity();
    /*  implement what you've done in LAB1  */
    // let tmp = String::from("r");
    if str == String::from("r\n") {
        // println!("r");
        get_rotation(Vector3::new(1.0, 2.0, 3.0), rotation_angle)
    } else {
        let mut model: Matrix4<f64> = Matrix4::identity();
        /*  implement your code here  */
        let rad = tran_ang_to_rad(rotation_angle);
        model.m11 = rad.cos();
        model.m22 = rad.cos();
        model.m12 = -rad.sin();
        model.m21 = rad.sin();
        //matrix4_info(model);
        model
    }
}

pub(crate) fn get_projection_matrix(
    eye_fov: f64,
    aspect_ratio: f64,
    z_near: f64,
    z_far: f64,
) -> Matrix4<f64> {
    let mut m_trans: Matrix4<f64> = Matrix4::identity();
    m_trans.m34 = -(z_near + z_far) / 2.0;

    /*  implement your code here  */

    let t = z_near * (tran_ang_to_rad(eye_fov / 2.0).tan());
    let r = t * aspect_ratio;
    let l = -r;
    let b = -t;
    let mut m_scal = Matrix4::identity();
    m_scal.m11 = 2.0 / (r - l);
    m_scal.m22 = 2.0 / (t - b);
    m_scal.m33 = 2.0 / (z_near - z_far);

    let mut m_per = Matrix4::identity();
    m_per *= z_near;
    m_per.m44 = 0.0;
    m_per.m33 += z_far;
    m_per.m43 = 1.0;
    m_per.m34 = -z_far * z_near;
    m_scal * m_trans * m_per
}

pub(crate) fn frame_buffer2cv_mat(frame_buffer: &Vec<Vector3<f64>>) -> opencv::core::Mat {
    let mut image = unsafe {
        Mat::new_rows_cols_with_data(
            700,
            700,
            opencv::core::CV_64FC3,
            frame_buffer.as_ptr() as *mut c_void,
            opencv::core::Mat_AUTO_STEP,
        )
        .unwrap()
    };
    let mut img = Mat::copy(&image).unwrap();
    image
        .convert_to(&mut img, opencv::core::CV_8UC3, 1.0, 1.0)
        .expect("panic message");
    cvt_color(&img, &mut image, COLOR_RGB2BGR, 0).unwrap();
    image
}
