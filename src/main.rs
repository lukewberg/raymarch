#![feature(split_array)]
use clap::Parser;
use raymarch::{
    camera::Camera,
    cli::Cli,
    drawables::{cube::Cube, mandelbulb::Mandelbulb, sphere::Sphere},
    light::Light,
    scene::{RenderOptions, Scene, SceneObject},
    transformation::Orientation,
    unsafe_buffer::UnsafeBuffer,
    vec3::Vec3,
};
use std::{panic, sync::Arc, time::Instant};

fn main() {
    panic::set_hook(Box::new(|_| {
        // Your custom panic hook logic here.
    }));

    let args = Cli::parse();
    let bench_a = Instant::now();
    let num_cpus = std::thread::available_parallelism().unwrap();

    let result_buffer =
        UnsafeBuffer::<f32>::new((args.width * args.height) as usize, num_cpus.get());

    let camera = Camera::new(Vec3::new(0.0, -2.5, 0.0), 90.0, (args.width, args.height));

    let scene_objects: Vec<Box<dyn SceneObject>> = vec![
        // Box::new(Sphere::new(
        //     Vec3::new(-3_f32, 7_f32, 0_f32),
        //     1.5,
        //     Orientation::default(),
        // )),
        // Box::new(Sphere::new(
        //     Vec3::new(-7_f32, 5_f32, 0_f32),
        //     1.5,
        //     Orientation::default(),
        // )),
        // Box::new(Sphere::new(
        //     Vec3::new(0_f32, 0_f32, 0_f32),
        //     1.5,
        //     Orientation::default(),
        // )),
        // Box::new(Cube::new(
        //     Vec3::new(0_f32, 0_f32, 0_f32),
        //     Vec3::new(1_f32, 1_f32, 1_f32),
        // )),
        Box::new(Mandelbulb::new(Vec3::new(0_f32, 0_f32, 0_f32), 10.0)),
    ];

    let lights: Vec<Light> = vec![Light {
        color: [0, 0, 0, 0],
        intensity: 0.5,
        pos: Vec3::new(2.0, -2.0, 0.0),
    }];
    let scene = Arc::new(Scene::new(camera, scene_objects, lights));
    let render_options = RenderOptions {
        threaded: args.threaded,
        width: args.width,
        height: args.height,
        shared_buffer: result_buffer,
    };

    if args.threaded {
        Scene::render_parallel(scene, render_options);
    } else {
        scene.render(render_options);
    }

    let result_a = bench_a.elapsed();
    println!("Done in: {:.2?}", result_a);
}
