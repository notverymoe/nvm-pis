// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::prelude::*;

use super::{SATShapeImpl, SATShape};

pub struct Mesh {
    pub origin: Vec3,
    pub points: Box<[Vec3]>,
    pub norms: Box<[(Vec3, [f32; 2])]>,
}

impl SATShapeImpl for Mesh {
    fn set_transform(&mut self, origin: Vec3, _rotation: Quat) {
        self.origin = origin;
    }

    fn get_transform(&self) -> (Vec3, Quat) {
        (self.origin, Quat::IDENTITY)
    }

    fn points(&self, consumer: &mut dyn FnMut(Vec3) -> bool) -> bool {
        for &point in self.points.iter() {
            if !consumer(point) { return false; }
        }
        true
    }

    fn project(&self, _other: &SATShape, axis: Vec3) -> [f32; 2] {
        self.points.iter().map(|v| axis.dot(*v)).fold([f32::MAX, f32::MIN], |p, c| [p[0].min(c), p[1].max(c)])
    }

    fn seperation_axes(&self, _other: &SATShape, consumer: &mut dyn FnMut(Vec3, [f32; 2]) -> bool) -> bool {
        for &(axis, cache) in self.norms.iter() {
            if !consumer(axis, cache) { return false; }
        }
        true
    }

    fn is_complex(&self) -> bool {
        false
    }
}