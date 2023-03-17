// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::prelude::*;

use super::{SATShapeImpl, SATShape};

pub struct Sphere {
    pub origin: Vec3,
    pub radius: f32,
}

impl SATShapeImpl for Sphere {
    fn set_transform(&mut self, origin: Vec3, _rotation: Quat) {
        self.origin = origin;
    }

    fn get_transform(&self) -> (Vec3, Quat) {
        (self.origin, Quat::IDENTITY)
    }

    fn points(&self, consumer: &mut dyn FnMut(Vec3) -> bool) -> bool {
        consumer(self.origin)
    }
    
    fn project(&self, _other: &SATShape, axis: Vec3) -> [f32; 2] {
        let origin = axis.dot(self.origin);
        [origin-self.radius, origin+self.radius]
    }

    fn seperation_axes(&self, other: &SATShape, consumer: &mut dyn FnMut(Vec3, [f32; 2]) -> bool) -> bool {
        other.points(&mut |point| {
            let axis = (point - self.origin).normalize();
            consumer(axis, self.project(other, axis))
        })
    }
    
    fn is_complex(&self) -> bool {
        true
    }
}
