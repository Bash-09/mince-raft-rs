use crate::network::types::UUID;

pub mod components;
use components::*;
use resources::entities::{ENTITIES, EntityType};

pub struct Entity {
    pub id: i32,
    pub uuid: UUID,

    pub entity_type: &'static EntityType,

    pub data: i32,

    pub pos: Position,
    pub vel: Velocity,
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

            pos: Position::new(),
            vel: Velocity::new(),
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
        px: f64,
        py: f64,
        pz: f64,
        yaw: f64,
        pitch: f64,
        head_pitch: f64,
        vx: f64,
        vy: f64,
        vz: f64,
    ) -> Entity {
        Entity {
            id,
            uuid,
            entity_type: ENTITIES.get(&entity_type).expect(&format!("Failed to get entity from ID: {}", entity_type)),
            data,
            pos: Position::new_with_values(px, py, pz),
            vel: Velocity::new_with_values(vx, vy, vz),
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
}

