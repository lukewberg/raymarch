use std::{fs::File, io::BufWriter, path::Path};

use crate::{camera::Camera, light::Light, ray, transformation::Transformable, vec3::Vec3};

pub struct Scene {
    pub camera: Camera,
    // pub lights: Vec<Light>,
    pub scene_objects: Vec<Box<dyn SceneObject>>,
}

impl Scene {
    pub fn new(camera: Camera, scene_objects: Vec<Box<dyn SceneObject>>) -> Scene {
        Scene {
            camera,
            scene_objects,
        }
    }

    pub fn render(&self) {
        // Take the camera's UV grid and construct rays
        let uv = self.camera.calc_uv_simd();
        let mut result_vec: Vec<u8> = Vec::with_capacity(uv.coords.len());
        for i in 0..uv.coords.len() {
            let current_coords = uv.coords[i];
            let mut direction = self.camera.uv_direction(current_coords.0, current_coords.1);
            direction.normalize();
            result_vec.push(ray::march(&self, &self.camera.origin, &direction));
        }
        let mut converted_results: Vec<[u8; 4]> = Vec::with_capacity(result_vec.len());
        for x in result_vec {
            if x == 1 {
                converted_results.push([0, 0, 0, 255]);
            } else {
                converted_results.push([255, 255, 255, 255]);
            }
        }
        Scene::flush_png(&(*(converted_results.concat())), 1920, 1080)
    }

    pub fn flush_png(data: &[u8], width: u32, height: u32) {
        let path = Path::new(r"./result.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);
        let mut encoder = png::Encoder::new(w, width, height);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
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

pub trait SceneObject: Transformable {
    fn pos(&self) -> &Vec3;
    fn sdf(&self, p: &Vec3) -> f32;
}
