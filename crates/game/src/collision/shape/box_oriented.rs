// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::prelude::*;

use super::{SATShapeImpl, SATShape};

pub struct BoxOriented {
    pub origin: Vec3,
    pub size: Vec3,
    pub norms: [Vec3; 3],
}

impl SATShapeImpl for BoxOriented {
    fn set_transform(&mut self, origin: Vec3, rotation: Quat) {
        self.origin = origin;
        let rotation = Mat3::from_quat(rotation);
        self.norms = [rotation.col(0), rotation.col(1), rotation.col(2),];
    }

    fn get_transform(&self) -> (Vec3, Quat) {
        (
            self.origin,
            Quat::from_mat3(&Mat3::from_cols(self.norms[0], self.norms[1], self.norms[2]))
        )
    }

    fn points(&self, consumer: &mut dyn FnMut(Vec3) -> bool) -> bool {
        let x = self.norms[0] * self.size.x;
        let y = self.norms[1] * self.size.y;
        let z = self.norms[2] * self.size.z;

           consumer(self.origin - x - y - z)
        && consumer(self.origin - x - y + z)
        && consumer(self.origin - x + y - z)
        && consumer(self.origin - x + y + z)
        && consumer(self.origin + x - y - z)
        && consumer(self.origin + x - y + z)
        && consumer(self.origin + x + y - z)
        && consumer(self.origin + x + y + z)
    }
    
    fn project(&self, _other: &SATShape, axis: Vec3) -> [f32; 2] {
        let mut result = [f32::MAX, f32::MIN];
        self.points(&mut |v| {
            let v = axis.dot(v);
            result[0] = result[0].min(v);
            result[1] = result[1].max(v);
            true
        });
        result
    }

    fn seperation_axes(&self, _other: &SATShape, consumer: &mut dyn FnMut(Vec3, [f32; 2]) -> bool) -> bool {
        let origin_x = self.norms[0].dot(self.origin);
        let origin_y = self.norms[1].dot(self.origin);
        let origin_z = self.norms[2].dot(self.origin);

           consumer(self.norms[0], [origin_x - self.size.x, origin_x + self.size.x])
        && consumer(self.norms[1], [origin_y - self.size.y, origin_y + self.size.y])
        && consumer(self.norms[2], [origin_z - self.size.z, origin_z + self.size.z])
    }
    
    fn is_complex(&self) -> bool {
        false
    }
}