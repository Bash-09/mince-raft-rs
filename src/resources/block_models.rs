use std::{collections::HashMap, error::Error};

use glam::{Vec2, Vec3};
use simple_error::bail;

use super::BLOCK_MODELS_RAW;

#[derive(Clone)]
pub struct BlockModel {
    ambient_occlusion: bool,
    display: HashMap<String, Display>,
    textures: HashMap<String, String>,
    elements: Vec<Element>,
}

#[derive(Clone)]
struct Display {
    pub rotation: Vec3,
    pub translation: Vec3,
    pub scale: Vec3,
}

#[derive(Clone)]
struct Element {
    pub from: Vec3,
    pub to: Vec3,
    pub rotation: Rotation,
    pub shade: bool,
    pub faces: HashMap<String, Face>,
}

#[derive(Clone)]
enum RotationAxis {
    X,
    Y,
    Z,
}

#[derive(Clone)]
struct Rotation {
    pub origin: Vec3,
    pub axis: RotationAxis,
    pub angle: f32,
    pub rescale: bool,
}

#[derive(Clone)]
struct Face {
    pub uv: (Vec2, Vec2),
    pub texture: String,
    pub cullface: String,
    pub rotation: f32,
    pub tintindex: f32,
}

impl BlockModel {
    pub fn empty() -> BlockModel {
        BlockModel {
            ambient_occlusion: false,
            display: HashMap::new(),
            textures: HashMap::new(),
            elements: Vec::new(),
        }
    }

    pub fn parse(
        json: &serde_json::Value,
        cache: Option<&mut HashMap<String, BlockModel>>,
    ) -> Result<BlockModel, Box<dyn Error>> {
        let mut base = BlockModel::empty();

        // Load parent model
        if let Some(serde_json::Value::String(parent)) = json.get("parent") {
            if let Some(cache) = cache {
                // Parse parent if it isn't already parsed and add it to the cache
                if cache.get(parent).is_none() {
                    if let Some(parent_raw) = BLOCK_MODELS_RAW.get(parent) {
                        let parent_parsed = Self::parse(parent_raw, Some(cache))?;
                        cache.insert(parent.clone(), parent_parsed);
                    } else {
                        bail!("Missing parent: {}", parent);
                    }
                }
                base = cache.get(parent).unwrap().clone();
            }
        }

        // Ambient occlusion
        if let Some(serde_json::Value::Bool(ambient_occlusion)) = json.get("ambientocclusion") {
            base.ambient_occlusion = *ambient_occlusion;
        }

        // Display
        if let Some(serde_json::Value::Object(display)) = json.get("display") {
            for (location, display) in display {
                base.display
                    .insert(location.clone(), Display::parse(display)?);
            }
        }

        // Textures
        if let Some(serde_json::Value::Object(textures)) = json.get("textures") {
            for (key, tex) in textures {
                if !tex.is_string() {
                    bail!("Invalid texture: {:?}", tex);
                }
                base.textures
                    .insert(key.clone(), tex.as_str().unwrap().to_string());
            }
        }

        // Elements
        if let Some(serde_json::Value::Array(elements)) = json.get("elements") {
            base.elements.clear();

            for element in elements {
                base.elements.push(Element::parse(element)?);
            }
        }

        todo!()
    }
}

impl Display {
    pub fn parse(json: &serde_json::Value) -> Result<Display, Box<dyn Error>> {
        todo!();
    }
}

impl Element {
    pub fn parse(json: &serde_json::Value) -> Result<Element, Box<dyn Error>> {
        todo!();
    }
}

impl Rotation {
    pub fn parse(json: &serde_json::Value) -> Result<Rotation, Box<dyn Error>> {
        todo!();
    }
}

impl Face {
    pub fn parse(json: &serde_json::Value) -> Result<Face, Box<dyn Error>> {
        todo!();
    }
}
