// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::prelude::{Vec3, Quat, Mat3, Component};
use enum_dispatch::enum_dispatch;

mod sat;
pub use sat::*;

mod box_aligned;
pub use box_aligned::*;

mod box_oriented;
pub use box_oriented::*;

mod sphere;
pub use sphere::*;

mod mesh;
pub use mesh::*;

use super::CollisionShape;

#[derive(Component)]
#[enum_dispatch]
pub enum SATShape {
    BoxAligned,
    BoxOriented,
    Sphere,
    Mesh,
}

impl SATShape {
    pub fn new(shape: CollisionShape, origin: Vec3, orientation: Quat) -> Self {
        match shape {
            CollisionShape::BoxAligned(size)     => Self::new_box_aligned(origin, size),
            CollisionShape::BoxOriented(size)    => Self::new_box_oriented(origin, orientation, size),
            CollisionShape::Sphere(radius)       => Self::new_sphere(origin, radius),
            CollisionShape::Mesh {points, norms} => Self::new_mesh(origin, points, &norms),
        }
    }

    pub fn new_box_aligned(origin: Vec3, size: Vec3) -> Self {
        BoxAligned{origin, size}.into()
    }

    pub fn new_box_oriented(origin: Vec3, orientation: Quat, size: Vec3) -> Self {
        BoxOriented{origin, size, norms: {
            let rotation = Mat3::from_quat(orientation);
            [rotation.col(0), rotation.col(1), rotation.col(2)]
        }}.into()
    }

    pub fn new_sphere(origin: Vec3, radius: f32) -> Self {
        Sphere{origin, radius}.into()
    }

    pub fn new_mesh(origin: Vec3, points: Box<[Vec3]>, norms: &[Vec3]) -> Self {
        let norms = norms.iter()
            .map(|axis| (*axis, points.iter().map(|v| axis.dot(*v))
            .fold([f32::MAX, f32::MIN], |p, c| [p[0].min(c), p[1].max(c)]))).collect::<Vec<(Vec3, [f32;2])>>().into_boxed_slice();
        Mesh{ origin, points, norms }.into()
    }
}

impl SATShape {
    pub fn set_transform(&mut self, origin: Vec3, rotation: Quat) {
        SATShapeImpl::set_transform(self, origin, rotation)
    }

    pub fn get_transform(&self) -> (Vec3, Quat) {
        SATShapeImpl::get_transform(self)
    }

    pub fn points(&self, consumer: &mut dyn FnMut(Vec3) -> bool) -> bool {
        SATShapeImpl::points(self, consumer)
    }

    pub fn project(&self, other: &SATShape, axis: Vec3) -> [f32; 2] {
        SATShapeImpl::project(self, other, axis)
    }

    pub fn seperation_axes(&self, other: &SATShape, consumer: &mut dyn FnMut(Vec3, [f32; 2]) -> bool) -> bool {
        SATShapeImpl::seperation_axes(self, other, consumer)
    }

    pub fn is_complex(&self) -> bool{
        SATShapeImpl::is_complex(self)
    }
}
