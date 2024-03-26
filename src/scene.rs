use std::{
    fs::File,
    io::BufWriter,
    path::Path,
    sync::Arc,
    thread::{self, JoinHandle},
};

use crate::{
    camera::Camera, light::Light, ray, transformation::Transformable, unsafe_buffer::UnsafeBuffer,
    vec3::Vec3,
};

pub struct Scene {
    pub camera: Camera,
    pub lights: Vec<Light>,
    pub scene_objects: Vec<Box<dyn SceneObject>>,
}

unsafe impl Sync for Scene {}

impl Scene {
    pub fn new(
        camera: Camera,
        scene_objects: Vec<Box<dyn SceneObject>>,
        lights: Vec<Light>,
    ) -> Scene {
        Scene {
            camera,
            scene_objects,
            lights,
        }
    }

    pub fn render(&self, options: RenderOptions) {
        if options.threaded {}
        // Take the camera's UV grid and construct rays
        let uv = self.camera.calc_uv();
        let mut result_vec: Vec<f32> = Vec::with_capacity(uv.coords.len());
        for i in 0..uv.coords.len() {
            let (x, y) = self.camera.uv_to_screen(uv.coords[i]);
            let screen_projection = self.camera.screen_to_world(x, y);
            let mut direction = screen_projection - self.camera.origin;
            direction.normalize();
            result_vec.push(ray::march(&self, &self.camera.origin, &direction).intensity);
        }
        let mut converted_results: Vec<[u8; 4]> = Vec::with_capacity(result_vec.len());
        for x in result_vec {
            let color_is = (255_f32 * x) as u8;
            converted_results.push([color_is, color_is, color_is, 255]);
        }
        Scene::flush_png(
            &(*(converted_results.concat())),
            options.width,
            options.height,
        )
    }

    pub fn render_parallel(arc_scene: Arc<Scene>, options: RenderOptions) {
        let uv = Arc::new(arc_scene.camera.calc_uv());
        let shared_result_buffer = Arc::new(options.shared_buffer);

        let mut handles = Vec::<JoinHandle<_>>::with_capacity(shared_result_buffer.threads);

        for id in 0..shared_result_buffer.threads {
            let result_buffer = shared_result_buffer.clone();
            let cloned_scene = arc_scene.clone();
            let cloned_uv = uv.clone();
            handles.push(thread::spawn(move || {
                for i in 0..cloned_uv.coords.len() {
                    if let Some(index) = result_buffer.get_valid_index(id, i) {
                        let (x, y) = cloned_scene.camera.uv_to_screen(cloned_uv.coords[index]);
                        let screen_projection = cloned_scene.camera.screen_to_world(x, y);
                        let mut direction = screen_projection - cloned_scene.camera.origin;
                        direction.normalize();
                        result_buffer.write(
                            index,
                            ray::march(&cloned_scene, &cloned_scene.camera.origin, &direction)
                                .intensity,
                            id,
                        );
                    } else {
                        return;
                    }
                }
            }))
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let mut converted_results: Vec<[u8; 4]> = Vec::with_capacity(shared_result_buffer.capacity);
        for i in 0..shared_result_buffer.capacity {
            let x = shared_result_buffer[i];
            let color_is = (255_f32 * x) as u8;
            converted_results.push([color_is, color_is, color_is, 255]);
        }
        Scene::flush_png(
            &(*(converted_results.concat())),
            options.width,
            options.height,
        )
    }

    pub fn flush_png(data: &[u8], width: u32, height: u32) {
        if !Path::exists(&Path::new(r"./output")) {
            std::fs::create_dir("./output").expect("Unable to create output directory!");
        }
        let path = Path::new(r"./output/result.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        // encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();
        // let data = [255, 0, 0, 255, 0, 0, 0, 255];
        writer.write_image_data(data).unwrap();
    }
}

pub struct RenderOptions {
    pub threaded: bool,
    pub width: u32,
    pub height: u32,
    pub shared_buffer: UnsafeBuffer<f32>,
}

pub trait SceneObject: Transformable + Send {
    fn pos(&self) -> &Vec3;
    fn sdf(&self, p: &Vec3) -> f32;
    fn specular_intensity(&self) -> f32;
    fn surface_color(&self) -> Vec3;
}
