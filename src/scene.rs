use std::{fs::File, io::BufWriter, path::Path, thread};

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
        let uv = self.camera.calc_uv();
        let mut result_vec: Vec<f32> = Vec::with_capacity(uv.coords.len());
        for i in 0..uv.coords.len() {
            let (x, y) = self.camera.uv_to_screen(uv.coords[i]);
            let mut direction = self
                .camera
                .screen_to_world(x, y);
            direction.normalize();
            result_vec.push(ray::march(&self, &self.camera.origin, &direction));
        }
        let mut converted_results: Vec<[u8; 4]> = Vec::with_capacity(result_vec.len());
        for x in result_vec {
            let color_is = (255_f32*x) as u8;
            converted_results.push([color_is, color_is, color_is, 255]);
        }
        Scene::flush_png(&(*(converted_results.concat())), 2560, 1440)
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
