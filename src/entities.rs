use std::ops::Mul;

use crate::{network::types::UUID, renderer::Vertex};

pub mod components;
use components::*;
use glam::Vec3;
use resources::entities::{EntityType, ENTITIES};

pub struct Entity {
    pub id: i32,
    pub uuid: UUID,

    pub entity_type: &'static EntityType,

    pub data: i32,

    pub pos: Vec3,
    pub last_pos: Vec3,
    pub vel: Vec3,
    pub ori: Orientation,
    pub ori_head: Orientation,

    pub on_ground: bool,
}

impl Entity {
    pub fn new(id: i32) -> Entity {
        Entity {
            id,
            uuid: UUID([0, 0]),

            entity_type: ENTITIES.get(&id).expect("Failed to get entity from ID"),
            data: 0,

            pos: Vec3::new(0.0, 0.0, 0.0),
            last_pos: Vec3::new(0.0, 0.0, 0.0),
            vel: Vec3::new(0.0, 0.0, 0.0),
            ori: Orientation::new(),
            ori_head: Orientation::new(),

            on_ground: true,
        }
    }

    pub fn new_with_values(
        id: i32,
        uuid: UUID,
        entity_type: i32,
        data: i32,
        px: f32,
        py: f32,
        pz: f32,
        yaw: f32,
        pitch: f32,
        head_pitch: f32,
        vx: f32,
        vy: f32,
        vz: f32,
    ) -> Entity {
        Entity {
            id,
            uuid,
            entity_type: ENTITIES
                .get(&entity_type)
                .expect(&format!("Failed to get entity from ID: {}", entity_type)),
            data,
            pos: Vec3::new(px, py, pz),
            last_pos: Vec3::new(px, py, pz),
            vel: Vec3::new(vx, vy, vz),
            ori: Orientation::new_with_values(yaw, pitch, 0.0, 0.0),
            ori_head: Orientation::new_with_values(0.0, head_pitch, -90.0, 90.0),
            on_ground: true,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_uuid(&self) -> UUID {
        self.uuid.clone()
    }

    pub fn get_type(&self) -> &'static EntityType {
        self.entity_type
    }



    pub fn update(&mut self, delta: f32) {
        let mut vel = self.vel;
        if self.on_ground {
            vel.y = 0.0;
        }

        self.pos += vel * delta;
    }

}

pub fn hitbox_model() -> Vec<Vertex> {
    vec![
        Vertex{position: [-0.5, 0.0, -0.5]},
        Vertex{position: [-0.5, 0.0, 0.5]},

        Vertex{position: [-0.5, 0.0, 0.5]},
        Vertex{position: [0.5, 0.0, 0.5]},

        Vertex{position: [0.5, 0.0, 0.5]},
        Vertex{position: [0.5, 0.0, -0.5]},

        Vertex{position: [0.5, 0.0, -0.5]},
        Vertex{position: [-0.5, 0.0, -0.5]},


        Vertex{position: [-0.5, 1.0, -0.5]},
        Vertex{position: [-0.5, 1.0, 0.5]},

        Vertex{position: [-0.5, 1.0, 0.5]},
        Vertex{position: [0.5, 1.0, 0.5]},

        Vertex{position: [0.5, 1.0, 0.5]},
        Vertex{position: [0.5, 1.0, -0.5]},

        Vertex{position: [0.5, 1.0, -0.5]},
        Vertex{position: [-0.5, 1.0, -0.5]},


        Vertex{position: [-0.5, 0.0, -0.5]},
        Vertex{position: [-0.5, 1.0, -0.5]},

        Vertex{position: [0.5, 0.0, -0.5]},
        Vertex{position: [0.5, 1.0, -0.5]},

        Vertex{position: [0.5, 0.0, 0.5]},
        Vertex{position: [0.5, 1.0, 0.5]},

        Vertex{position: [-0.5, 0.0, 0.5]},
        Vertex{position: [-0.5, 1.0, 0.5]},
        ]
}