// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::prelude::*;
use enum_dispatch::enum_dispatch;

use super::SATShape;

#[enum_dispatch(SATShape)]
pub(super) trait SATShapeImpl {
    fn set_transform(&mut self, origin: Vec3, rotation: Quat);
    fn get_transform(&self) -> (Vec3, Quat);

    fn points(&self, consumer: &mut dyn FnMut(Vec3) -> bool) -> bool;
    fn project(&self, other: &SATShape, axis: Vec3) -> [f32; 2];
    fn seperation_axes(&self, other: &SATShape, consumer: &mut dyn FnMut(Vec3, [f32; 2]) -> bool) -> bool;
    fn is_complex(&self) -> bool;
}

pub fn sweep(
    result: [f32; 2], 
    motion_dir: Vec3,
    motion_dist: f32,
    axis: Vec3,
) -> [f32; 2] {
    match motion_dist * axis.dot(motion_dir) {
        v if v < 0.0 => [result[0]+v, result[1]  ],
        v            => [result[0],   result[1]+v],
    }
}

pub fn test_overlap(a: &SATShape, b: &SATShape) -> bool {
    let process = |[a_min, a_max]: [f32; 2], [b_min, b_max]: [f32; 2]| b_max >= a_min && b_min <= a_max;
       a.seperation_axes(b, &mut |axis, a_proj| process(a_proj,             b.project(a, axis))) 
    && b.seperation_axes(a, &mut |axis, b_proj| process(a.project(b, axis), b_proj            ))
}
