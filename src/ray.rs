use std::simd::f32x4;

use crate::{scene::Scene, vec3::Vec3};

/// Marches the ray forward by the smallest distance returned by the scene's SceneObjects sdf functions.
pub fn march(scene: &Scene, origin: &Vec3, direction: &Vec3) -> f32 {
    // Get distances to all Objects in scene
    let tolerance: f32 = 0.0003;
    let mut last_closest = f32::INFINITY;
    let mut position = (*origin).clone();
    let mut steps = 0;
    // let mut new_position: Option<[f32; 4]> = None;

    while last_closest > tolerance {
        let mut closest = f32::INFINITY;
        let mut i = 0;
        let num_scene_objects = scene.scene_objects.len();
        while i < num_scene_objects {
            let distance = { scene.scene_objects[i].sdf(&position) };

            if distance < closest {
                // last_closest = closest;
                closest = distance;
                // new_position =
                //     Some((f32x4::splat(distance) + f32x4::from_array(position.vec)).into());
            }

            // Not a hit, abort!
            // if closest > last_closest {
            //     return 0;
            // }

            if distance > 100_f32 {
                return 0_f32;
            }

            // if let Some(vec) = new_position {
            //     (position).vec = vec;
            // }
            i += 1;
        }
        last_closest = closest;
        // Scale direction by distance
        let scaled = direction.scale(closest);
        position.vec = (f32x4::from_array(scaled) + f32x4::from_array(position.vec)).to_array();
        steps += 1;
    }
    1_f32 - (steps as f32 / 75_f32)
}
