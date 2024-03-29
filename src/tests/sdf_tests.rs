use crate::{
    drawables::mandelbulb::Mandelbulb,
    scene::SceneObject,
    vec3::Vec3,
};

#[test]
fn test_mandelbulb_sdf() {
    let mandelbulb = Mandelbulb::new(Vec3::new(0.0, 0.0, 0.0), 12.0, 1_f32);
    let point = Vec3::new(0.0, -1.0004357, 0.0);
    println!("{:?}", point.vec);
    let step_one = mandelbulb.sdf(&point);
    println!("{}", step_one);
    // let direction = (*(mandelbulb.pos()) - point).normalize_new();
    // let scaled = direction.scale(step_one);
    // let new_pos = Vec3 {
    //     vec: (f32x4::from_array(point.vec) + f32x4::from_array(scaled)).to_array()
    // };
    // println!("{:?}", new_pos.vec);
    // let step_two = mandelbulb.sdf(&new_pos);
    // println!("{}", step_two)
}

#[test]
fn test_surface_normal() {
    
}
