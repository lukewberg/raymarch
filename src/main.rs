use std::time::Instant;

use raymarch::{camera::Camera, vec3::Vec3};

fn main() {
    // let vec_a = Vec3::new(2.34623342, 5.2983742, 9.12387978);
    // let vec_b = Vec3::new(7.348756, 6.289734, 3.903457);

    let benchA = Instant::now();
    let frame_buffer = [0u32; 100];
    // frame_buffer.split_array_mut();
    // let mut i = 0;
    // while i < 1_000_000_000 {
    //     Vec3::multiply_vec3(&vec_a, &vec_b);
    //     Vec3::multiply_vec3_simd(&vec_a, &vec_b);
    //     // vec_a.multiply_vec3_scalar(1245.567231412334);
    //     // vec_a.multiply_vec3_scalar_simd(1245.567231412334);
    //     i += 1;
    // }

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 90.0, (1920, 1080));
    let uv_coords = camera.calc_uv_simd();
    let sample_point = uv_coords[(100, 100)];

    // #[cfg(target_arch = "arm")]
    // use std::arch::is_arm_feature_detected;
    // #[cfg(target_arch = "arm")]
    // println("{}", is_arm_feature_detected!("neon"));

    println!("{:?}", sample_point);
    let aResult = benchA.elapsed();
    println!("Bench A: {:.2?}", aResult);

    // let benchB = Instant::now();
    // for i in 0..2_000_000_000 {
    //     Vec3::multiply_vec3_simd(&vecA, &vecB);
    // }
    // let bResult = benchB.elapsed();
    // println!("Bench B: {:.2?}", bResult);
}
