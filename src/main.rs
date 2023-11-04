#![feature(split_array)]
use clap::Parser;
use raymarch::{
    camera::Camera,
    cli::Cli,
    drawables::sphere::Sphere,
    scene::{RenderOptions, Scene, SceneObject},
    transformation::Orientation,
    unsafe_buffer::UnsafeBuffer,
    vec3::Vec3,
};
use std::{sync::Arc, time::Instant};

fn main() {
    // let vec_a = Vec3::new(2.34623342, 5.2983742, 9.12387978);
    // let vec_b = Vec3::new(7.348756, 6.289734, 3.903457);

    let args = Cli::parse();
    let bench_a = Instant::now();
    // let mut _frame_buffer = [0u32; 100];
    let num_cpus = std::thread::available_parallelism().unwrap();
    // println!("{}", num_cpus);

    // _frame_buffer.split_array_mut::<4>();
    // let mut i = 0;
    // while i < 1_000_000_000 {
    //     Vec3::multiply_vec3(&vec_a, &vec_b);
    //     Vec3::multiply_vec3_simd(&vec_a, &vec_b);
    //     // vec_a.multiply_vec3_scalar(1245.567231412334);
    //     // vec_a.multiply_vec3_scalar_simd(1245.567231412334);
    //     i += 1;
    // }

    let result_buffer = Arc::new(UnsafeBuffer::<u8>::new(
        (args.width * args.height) as usize,
        num_cpus.get(),
    ));

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 120.0, (args.width, args.height));
    // let uv_coords = camera.calc_uv_simd();
    // let sample_point = uv_coords[(100, 100)];
    // println!("{:?}", sample_point);
    let scene_objects: Vec<Box<dyn SceneObject>> = vec![
        Box::new(Sphere::new(
            Vec3::new(-3_f32, 7_f32, 0_f32),
            1.5,
            Orientation::default(),
        )),
        Box::new(Sphere::new(
            Vec3::new(-7_f32, 5_f32, 0_f32),
            1.5,
            Orientation::default(),
        )),
        Box::new(Sphere::new(
            Vec3::new(1.5_f32, 7_f32, 1_f32),
            1.5,
            Orientation::default(),
        )),
    ];
    let mut scene = Scene::new(camera, scene_objects);
    scene.render(RenderOptions {
        threaded: args.threaded,
        width: args.width,
        height: args.height,
    });

    // Testing matrices
    // let mat3_a = Mat3::pitch(75_f32);
    // let _mul_result = mat3_a * Vec3::new(11.312, 451.78, 32.8);

    // #[cfg(target_arch = "arm")]
    // use std::arch::is_arm_feature_detected;
    // #[cfg(target_arch = "arm")]
    // println("{}", is_arm_feature_detected!("neon"));

    let result_a = bench_a.elapsed();
    println!("Done in: {:.2?}", result_a);

    // let benchB = Instant::now();
    // for i in 0..2_000_000_000 {
    //     Vec3::multiply_vec3_simd(&vecA, &vecB);
    // }
    // let bResult = benchB.elapsed();
    // println!("Bench B: {:.2?}", bResult);
}
