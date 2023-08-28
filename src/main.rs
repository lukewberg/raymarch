use std::time::Instant;

use raymarch::vec3::Vec3;

fn main() {
    let vec_a = Vec3::new(2.34623342, 5.2983742, 9.12387978);
    let vec_b = Vec3::new(7.348756, 6.289734, 3.903457);

    // let benchA = Instant::now();
    let mut i = 0;
    while i < 1_000_000_000 {
        // Vec3::multiply_vec3(&vecA, &vecB);
        // Vec3::multiply_vec3_simd(&vecA, &vecB);
        vec_a.multiply_vec3_scalar(1245.567231412334);
        vec_a.multiply_vec3_scalar_simd(1245.567231412334);
        i += 1;
    }
    // let aResult = benchA.elapsed();
    // println!("Bench A: {:.2?}", aResult);

    // let benchB = Instant::now();
    // for i in 0..2_000_000_000 {
    //     Vec3::multiply_vec3_simd(&vecA, &vecB);
    // }
    // let bResult = benchB.elapsed();
    // println!("Bench B: {:.2?}", bResult);
}
