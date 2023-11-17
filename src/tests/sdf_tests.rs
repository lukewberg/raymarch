use crate::{drawables::mandelbulb::{self, Mandelbulb}, vec3::Vec3, scene::SceneObject};

#[test]
fn test_mandelbulb_sdf() {
    let mandelbulb = Mandelbulb::new(Vec3::new(0.0, 0.0, 0.0), 12.0);
    let point = Vec3::new(-10.0, 5.0, 0.0);
    let result = mandelbulb.sdf(&point);
    println!("{}", result);
}