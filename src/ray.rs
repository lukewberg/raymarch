use std::{f32::consts::PI, simd::f32x4, sync::Arc};

use crate::{
    light::Light, scene::{Scene, SceneObject}, vec3::Vec3
};

/// Marches the ray forward by the smallest distance returned by the scene's SceneObjects sdf functions.
pub fn march(scene: &Scene, origin: &Vec3, direction: &Vec3) -> RayResult {
    // Get distances to all Objects in scene
    let tolerance: f32 = 0.0003;
    let mut last_closest = f32::INFINITY;
    let mut position = (*origin).clone();
    let mut steps = 0;
    let mut object_index: usize = 0;
    // let mut new_position: Option<[f32; 4]> = None;

    while last_closest > tolerance {
        if last_closest < f32::INFINITY {
            let scaled = direction.scale(last_closest * 0.95);
            position.vec = (f32x4::from_array(position.vec) + f32x4::from_array(scaled)).to_array();
        }
        let (closest, index) = distance_to_closest(scene, &position);
        object_index = index;
        if closest > 100.0 {
            return RayResult {
                point: position,
                intensity: 0.0,
                light_result: [0, 0, 0, 0]
            };
        }
        last_closest = closest;
        // Scale direction by distance
        // let scaled = direction.scale(closest * 0.95);
        // position.vec = (f32x4::from_array(position.vec) + f32x4::from_array(scaled)).to_array();
        steps += 1;
    }

    let normal =
        calculate_surface_normal(&scene.scene_objects[object_index], &position, last_closest);
    // let normal2 = ((*(scene.scene_objects[object_index].pos()) - position) * -1.0).normalize_new();
    let light_direction = (scene.lights[0].pos - position).normalize_new();
    let light_diff = light_direction.dot_prod(&normal);
    let reflection_tup = (light_direction * -1_f32).reflect(&normal);
    let specular_light = Light::get_specular(&scene.lights[0], &scene.scene_objects[object_index], reflection_tup);
    let surface_color = scene.scene_objects[object_index].surface_color();
    // 1_f32 - (steps as f32 / 255_f32)
    RayResult {
        point: position,
        intensity: light_diff,
        light_result
    }
}

pub fn march_light(scene: &Scene, origin: &Vec3, direction: &Vec3, mode: LightMode) {
    let tolerance: f32 = 0.0003;
    let breakaway_distance: f32 = 1.0;
    let mut closest = f32::INFINITY;
    let mut position = (*origin).clone();
    let mut obj_index: usize = 0;
    let mut inflected = false;
    let mut obj_distances: Vec<f32> = Vec::with_capacity(scene.scene_objects.len());
    obj_distances.fill(f32::INFINITY);

    while closest < 100.0 {
        while !inflected {
            let (distance, index) = distance_to_closest(scene, &position);
            if distance < obj_distances[index] {}
            obj_distances[index] = distance;
        }
    }
}

pub fn distance_to_closest(scene: &Scene, p: &Vec3) -> (f32, usize) {
    let num_scene_objects = scene.scene_objects.len();
    let mut closest = f32::INFINITY;
    let mut closest_index = 0;
    let mut i = 0;
    while i < num_scene_objects {
        let distance = scene.scene_objects[i].sdf(p);
        if distance < closest {
            closest_index = i;
            closest = distance;
        }
        i += 1;
    }
    (closest, closest_index)
}

pub fn calculate_surface_normal(
    scene_object: &Box<dyn SceneObject>,
    point: &Vec3,
    distance: f32,
) -> Vec3 {
    let x1 = Vec3::new(point.vec[0] + 0.0003, point.vec[1], point.vec[2]);
    let y1 = Vec3::new(point.vec[0], point.vec[1] + 0.0003, point.vec[2]);
    let z1 = Vec3::new(point.vec[0], point.vec[1], point.vec[2] + 0.0003);

    let mut normal = Vec3::new(
        scene_object.sdf(&x1),
        scene_object.sdf(&y1),
        scene_object.sdf(&z1),
    ) - distance;
    normal.normalize();
    normal
}

pub struct RayResult {
    pub point: Vec3,
    pub intensity: f32,
    pub light_result: [u8; 4]
}

pub enum LightMode {
    ToLights,
    Reflections,
}

// pub fn calculate_normal()
