use crate::network::types::UUID;

pub mod components;
use components::*;

pub struct Entity {
    pub id: i32,
    pub uuid: UUID,

    pub entity_type: i32,

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

            entity_type: 0,
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
            entity_type,
            data,
            pos: Position::new_with_values(px, py, pz),
            vel: Velocity::new_with_values(vx, vy, vz),
            ori: Orientation::new_with_values(yaw, pitch),
            ori_head: Orientation::new_with_values(0.0, head_pitch),
            on_ground: true,
        }
    }

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_uuid(&self) -> UUID {
        self.uuid.clone()
    }

    pub fn get_type(&self) -> i32 {
        self.entity_type
    }
}

pub struct EntityType {
    pub name: &'static str,
    pub bb_xz: f32,
    pub bb_y: f32,
    pub id: &'static str,
}

pub const ENTITIES: [EntityType; 112] = [
    EntityType {
        name: "Area Effect Cloud",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Armor Stand",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Arrow",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Axolotl",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Bat",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Bee",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Blaze",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Boat",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Cat",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Cave Spider",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Chicken",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Cod",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Cow",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Creeper",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Dolphin",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Donkey",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Dragon Fireball",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Drowned",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Elder Gaurdian",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "End Crystal",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Ender Dragon",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Enderman",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Endermite",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Evoker",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Evoker Fangs",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Experience Orb",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Eye of Ender",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Falling Block",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Firework Rocket Entity",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Fox",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Ghast",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Giant",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Glow Item Frame",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Glow Squid",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Goat",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Guardian",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Hoglin",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Horse",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Husk",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Illusioner",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Iron Golem",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Item",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Item Frame",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Fireball",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Leash Knot",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Lightning Bolt",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Llama",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Llama Spit",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Magma Cube",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Marker",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    }, // ???
    EntityType {
        name: "Minecart",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Minecart Chest",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Minecart Command Block",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Minecart Furnace",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Minecart Hopper",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Minecart Spawner",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Minecart TNT",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Mule",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Mooshroom",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Ocelot",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Painting",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Panda",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Parrot",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Phantom",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Pig",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Piglin",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Piglin Brute",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Pillager",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Polar Bear",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Primed TNT",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Pufferfish",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Rabbit",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Ravager",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Salmon",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Sheep",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Shulker",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Shulker Bullet",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Silverfish",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Skeleton",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Skeleton Horse",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Slime",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Small Fireball",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Snow Golem",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Snowball",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Spectral Arrow",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Spider",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Squid",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Stray",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Strider",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Thrown Egg",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Thrown Ender Pearl",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Thrown Experience Bottle",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Thrown Potion",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Thrown Trident",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Trader Llama",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Tropical Fish",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Turtle",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Vex",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Vindicator",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Wandering Trader",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Witch",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Wither",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Wither Skeleton",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Wither Skull",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Wolf",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Zoglin",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Zombie",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Zombie Horse",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Zombie Villager",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Zombified Piglin",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Player",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
    EntityType {
        name: "Fishing Hook",
        bb_xz: 1.0,
        bb_y: 1.0,
        id: "minecraft:",
    },
];
