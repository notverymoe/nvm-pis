// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::prelude::*;

use super::{SATShapeImpl, SATShape};

pub struct BoxAligned {
    pub origin: Vec3,
    pub size: Vec3,
}

impl SATShapeImpl for BoxAligned {
    fn set_transform(&mut self, origin: Vec3, _rotation: Quat) {
        self.origin = origin;
    }

    fn get_transform(&self) -> (Vec3, Quat) {
        (self.origin, Quat::IDENTITY)
    }

    fn points(&self, consumer: &mut dyn FnMut(Vec3) -> bool) -> bool {
           consumer(self.origin + Vec3::new(-self.size.x, -self.size.y, -self.size.z))
        && consumer(self.origin + Vec3::new(-self.size.x, -self.size.y,  self.size.z))
        && consumer(self.origin + Vec3::new(-self.size.x,  self.size.y, -self.size.z))
        && consumer(self.origin + Vec3::new(-self.size.x,  self.size.y,  self.size.z))
        && consumer(self.origin + Vec3::new( self.size.x, -self.size.y, -self.size.z))
        && consumer(self.origin + Vec3::new( self.size.x, -self.size.y,  self.size.z))
        && consumer(self.origin + Vec3::new( self.size.x,  self.size.y, -self.size.z))
        && consumer(self.origin + Vec3::new( self.size.x,  self.size.y,  self.size.z))
    }
    
    fn project(&self, _other: &SATShape, axis: Vec3) -> [f32; 2] {
        let origin = axis.dot(self.origin);
        let size   = self.size * axis;
        [
            origin - size.x - size.y - size.z,
            origin - size.x - size.y + size.z,
            origin - size.x + size.y - size.z,
            origin - size.x + size.y + size.z,
            origin + size.x - size.y - size.z,
            origin + size.x - size.y + size.z,
            origin + size.x + size.y - size.z,
            origin + size.x + size.y + size.z,
        ].iter().fold([f32::MAX, f32::MIN], |p, c| [p[0].min(*c), p[1].max(*c)])
    }

    fn seperation_axes(&self, _other: &SATShape, consumer: &mut dyn FnMut(Vec3, [f32; 2]) -> bool) -> bool {
           consumer(Vec3::X, [self.origin.x - self.size.x, self.origin.x + self.size.x])
        && consumer(Vec3::Y, [self.origin.y - self.size.y, self.origin.y + self.size.y])
        && consumer(Vec3::Z, [self.origin.z - self.size.z, self.origin.z + self.size.z])
    }
    
    fn is_complex(&self) -> bool {
        false
    }
}