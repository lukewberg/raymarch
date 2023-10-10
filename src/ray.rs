use std::simd::f32x4;

use crate::{scene::Scene, vec3::Vec3};

/// Marches the ray forward by the smallest distance returned by the scene's SceneObjects sdf functions.
pub fn march(scene: &Scene, origin: &Vec3, direction: &Vec3) -> u8 {
    // Get distances to all Objects in scene
    let tolerance: f32 = 0.0001;
    let mut closest = f32::INFINITY;
    let mut last_closest = f32::INFINITY;
    let mut position = origin.clone();

    while closest > tolerance {
        let num_scene_objects = scene.scene_objects.len();
        for i in 0..num_scene_objects {
            let distance = scene.scene_objects[i].sdf(&position);
            if distance < closest {
                last_closest = closest;
                closest = distance;
                position.vec = (f32x4::splat(distance) + f32x4::from_array(position.vec)).into();
            }

            // Not a hit, abort!
            if closest > last_closest {
                return 0;
            }
        }
    }
    1
}
