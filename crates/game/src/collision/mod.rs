// // Plants In Space // Copyright 2023 Natalie Baker // AGPLv3 // //

use bevy::{prelude::*, utils::HashMap};

mod shape;
pub use shape::*;

pub const CHUNK_SIZE: f32 = 5.0;

#[derive(Component, Copy, Clone)]
pub struct StaticCollider;

#[derive(Component, Clone)]
pub enum CollisionShape {
    BoxAligned(Vec3),
    BoxOriented(Vec3),
    Sphere(f32),
    Mesh{
        points: Box<[Vec3]>,
        norms:  Box<[Vec3]>,
    }
}

#[derive(Resource, Default)]
pub struct PhysicsWorldStatic {
    data:   HashMap<Entity, shape::SATShape>,
    chunks: HashMap<IVec3, Vec<Entity>>,
}

impl PhysicsWorldStatic {
    pub fn insert(&mut self, entity: Entity, shape: CollisionShape, origin: Vec3, orientation: Quat) {
        let shape = shape::SATShape::new(shape, origin, orientation);
        self.mark_extents(entity, &shape);
        self.data.insert(entity, shape);   
    }

    pub fn trace<'a>(&'a self, shape: &shape::SATShape, motion: Vec3, results: &mut Vec<(Entity, &'a shape::SATShape)>) {
        let [x_extents, y_extents, z_extents] = calculate_extents(shape, motion);
        for x in x_extents {
            for y in y_extents.clone() {
                for z in z_extents.clone() {
                    // OPT check cell against shape
                    let pos = IVec3::new(x, y, z);
                    if let Some(chunk) = self.chunks.get(&pos) {
                        for entity in chunk {
                            // OPT investigate hashset perf
                            // OPT investigate duplcate perf
                            if results.iter().any(|(e, _)| e == entity) { continue; }
                            results.push((*entity, self.data.get(entity).unwrap()));
                        }
                    }
                }
            }
        }
    }
}

impl PhysicsWorldStatic {
    fn mark_extents(&mut self, entity: Entity, shape: &shape::SATShape) {
        let [x_extents, y_extents, z_extents] = calculate_extents(shape, Vec3::ZERO);
        for x in x_extents {
            for y in y_extents.clone() {
                for z in z_extents.clone() {
                    self.mark_cell(IVec3::new(x, y, z), entity);
                }
            }
        }
    }

    fn mark_cell(&mut self, cell: IVec3, entity: Entity) {
        self.chunks.entry(cell).or_default().push(entity);
    }
}

fn calculate_extents(shape: &shape::SATShape, motion: Vec3) -> [std::ops::Range<i32>; 3] {
    let (origin, _) = shape.get_transform();
    let mut extents = [[origin.x, origin.x], [origin.y, origin.y], [origin.z, origin.z]];
    shape.points(&mut |v| {
        extents[0][0] = extents[0][0].min(v.x);
        extents[0][1] = extents[0][1].max(v.x);
        extents[1][0] = extents[1][0].min(v.y);
        extents[1][1] = extents[1][1].max(v.y);
        extents[2][0] = extents[2][0].min(v.z);
        extents[2][1] = extents[2][1].max(v.z);
        true
    });
    
    extents[0][0] = extents[0][0].min(extents[0][0] + motion.x);
    extents[0][1] = extents[0][1].max(extents[0][1] + motion.x);
    extents[1][0] = extents[1][0].min(extents[1][0] + motion.y);
    extents[1][1] = extents[1][1].max(extents[1][1] + motion.y);
    extents[2][0] = extents[2][0].min(extents[2][0] + motion.z);
    extents[2][1] = extents[2][1].max(extents[2][1] + motion.z);

    [
        (extents[0][0]/CHUNK_SIZE).floor() as i32..(extents[0][1]/CHUNK_SIZE).ceil() as i32,
        (extents[1][0]/CHUNK_SIZE).floor() as i32..(extents[1][1]/CHUNK_SIZE).ceil() as i32,
        (extents[2][0]/CHUNK_SIZE).floor() as i32..(extents[2][1]/CHUNK_SIZE).ceil() as i32,
    ]
}