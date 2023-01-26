use std::{collections::HashMap, error::Error};

use glam::{Vec2, Vec3};
use simple_error::{bail, require_with};

use super::BLOCK_MODELS_RAW;

#[derive(Clone, Debug)]
pub struct BlockModel {
    ambient_occlusion: bool,
    display: HashMap<String, Display>,
    textures: HashMap<String, String>,
    elements: Vec<Element>,
}

#[derive(Clone, Debug)]
struct Display {
    pub rotation: Vec3,
    pub translation: Vec3,
    pub scale: Vec3,
}

#[derive(Clone, Debug)]
struct Element {
    pub from: Vec3,
    pub to: Vec3,
    pub rot: Option<Rotation>,
    pub shade: bool,
    pub faces: HashMap<String, Face>,
}

#[derive(Clone, Debug)]
enum RotationAxis {
    X,
    Y,
    Z,
}

#[derive(Clone, Debug)]
struct Rotation {
    pub origin: Vec3,
    pub axis: RotationAxis,
    pub angle: f32,
    pub rescale: bool,
}

#[derive(Clone, Debug)]
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

    pub fn block_block() -> BlockModel {
        BlockModel::empty()
    }

    pub fn block_thin_block() -> BlockModel {
        BlockModel::empty()
    }

    pub fn block_cube() -> BlockModel {
        BlockModel {
            ambient_occlusion: true,
            display: HashMap::new(),
            textures: HashMap::new(),
            elements: Vec::new(),
        }
    }

    pub fn block_cube_column() -> BlockModel {
        let mut base = Self::block_cube();
        base.textures
            .insert("particle".to_string(), "#side".to_string());
        base.textures.insert("down".to_string(), "#end".to_string());
        base.textures.insert("up".to_string(), "#end".to_string());
        base.textures
            .insert("north".to_string(), "#side".to_string());
        base.textures
            .insert("east".to_string(), "#side".to_string());
        base.textures
            .insert("south".to_string(), "#side".to_string());
        base.textures
            .insert("west".to_string(), "#side".to_string());
        base
    }

    pub fn parse(
        json: &serde_json::Value,
        cache: Option<&mut HashMap<String, BlockModel>>,
    ) -> Result<BlockModel, Box<dyn Error>> {
        let mut base = BlockModel::empty();

        // Load parent model
        if let Some(serde_json::Value::String(parent)) = json.get("parent") {
            match parent.as_str() {
                "block/block" => base = BlockModel::block_block(),
                "block/cube" => base = BlockModel::block_cube(),
                "block/thin_block" => base = BlockModel::block_thin_block(),
                "block/cube_column" => base = BlockModel::block_cube_column(),
                _ => {
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

        Ok(base)
    }
}

impl Display {
    pub fn empty() -> Display {
        Display {
            rotation: Vec3::ZERO,
            translation: Vec3::ZERO,
            scale: Vec3::ZERO,
        }
    }

    pub fn parse(json: &serde_json::Value) -> Result<Display, Box<dyn Error>> {
        let mut base = Self::empty();

        // Rotation
        if let Some(serde_json::Value::Array(rot)) = json.get("rotation") {
            if rot.len() != 3 {
                bail!("Incorrect number of arguments in Display rotation");
            }

            base.rotation.x =
                require_with!(rot.get(0).unwrap().as_f64(), "Wrong type for rotation.") as f32;
            base.rotation.y =
                require_with!(rot.get(1).unwrap().as_f64(), "Wrong type for rotation.") as f32;
            base.rotation.z =
                require_with!(rot.get(2).unwrap().as_f64(), "Wrong type for rotation.") as f32;
        }

        // Translation
        if let Some(serde_json::Value::Array(trans)) = json.get("translation") {
            if trans.len() != 3 {
                bail!("Incorrect number of arguments in Display translation");
            }

            base.translation.x = require_with!(
                trans.get(0).unwrap().as_f64(),
                "Wrong type for translation."
            ) as f32;
            base.translation.y = require_with!(
                trans.get(1).unwrap().as_f64(),
                "Wrong type for translation."
            ) as f32;
            base.translation.z = require_with!(
                trans.get(2).unwrap().as_f64(),
                "Wrong type for translation."
            ) as f32;
        }

        // Scale
        if let Some(serde_json::Value::Array(scale)) = json.get("scale") {
            if scale.len() != 3 {
                bail!("Incorrect number of arguments in Display scale");
            }

            base.scale.x =
                require_with!(scale.get(0).unwrap().as_f64(), "Wrong type for scale.") as f32;
            base.scale.y =
                require_with!(scale.get(1).unwrap().as_f64(), "Wrong type for scale.") as f32;
            base.scale.z =
                require_with!(scale.get(2).unwrap().as_f64(), "Wrong type for scale.") as f32;
        }

        Ok(base)
    }
}

impl Element {
    pub fn empty() -> Element {
        Element {
            from: Vec3::ZERO,
            to: Vec3::ZERO,
            rot: None,
            shade: false,
            faces: HashMap::new(),
        }
    }

    pub fn parse(json: &serde_json::Value) -> Result<Element, Box<dyn Error>> {
        let mut base = Self::empty();

        // From
        if let Some(serde_json::Value::Array(from)) = json.get("from") {
            if from.len() != 3 {
                bail!("Incorrect number of arguments in Element from");
            }

            base.from.x = require_with!(
                from.get(0).unwrap().as_f64(),
                "Wrong type for Element from."
            ) as f32;
            base.from.y = require_with!(
                from.get(1).unwrap().as_f64(),
                "Wrong type for Element from."
            ) as f32;
            base.from.z = require_with!(
                from.get(2).unwrap().as_f64(),
                "Wrong type for Element from."
            ) as f32;
        }

        //  To
        if let Some(serde_json::Value::Array(to)) = json.get("to") {
            if to.len() != 3 {
                bail!("Incorrect number of arguments in Element to");
            }

            base.to.x =
                require_with!(to.get(0).unwrap().as_f64(), "Wrong type for Element to.") as f32;
            base.to.y =
                require_with!(to.get(1).unwrap().as_f64(), "Wrong type for Element to.") as f32;
            base.to.z =
                require_with!(to.get(2).unwrap().as_f64(), "Wrong type for Element to.") as f32;
        }

        // Rotation
        if let Some(rotation) = json.get("rotation") {
            base.rot = Some(Rotation::parse(rotation)?);
        }

        // Shade
        if let Some(serde_json::Value::Bool(shade)) = json.get("shade") {
            base.shade = *shade;
        }

        // Faces
        if let Some(serde_json::Value::Object(faces)) = json.get("faces") {
            for (face, data) in faces {
                base.faces.insert(face.clone(), Face::parse(data)?);
            }
        }

        Ok(base)
    }
}

impl Rotation {
    pub fn empty() -> Rotation {
        Rotation {
            origin: Vec3::ZERO,
            axis: RotationAxis::X,
            angle: 0.0,
            rescale: false,
        }
    }

    pub fn parse(json: &serde_json::Value) -> Result<Rotation, Box<dyn Error>> {
        let mut base = Self::empty();

        //  Origin
        if let Some(serde_json::Value::Array(origin)) = json.get("origin") {
            if origin.len() != 3 {
                bail!("Incorrect number of arguments in Element origin");
            }

            base.origin.x = require_with!(
                origin.get(0).unwrap().as_f64(),
                "Wrong type for Element origin."
            ) as f32;
            base.origin.y = require_with!(
                origin.get(1).unwrap().as_f64(),
                "Wrong type for Element origin."
            ) as f32;
            base.origin.z = require_with!(
                origin.get(2).unwrap().as_f64(),
                "Wrong type for Element origin."
            ) as f32;
        }

        // Axis
        if let Some(serde_json::Value::String(axis)) = json.get("axis") {
            if axis == "x" {
                base.axis = RotationAxis::X;
            }
            if axis == "y" {
                base.axis = RotationAxis::Y;
            }
            if axis == "z" {
                base.axis = RotationAxis::Z;
            }
        }

        // Angle
        if let Some(serde_json::Value::Number(angle)) = json.get("angle") {
            base.angle = require_with!(angle.as_f64(), "Couldn't get angle of rotation.") as f32;
        }

        // Rescale
        if let Some(serde_json::Value::Bool(rescale)) = json.get("rescale") {
            base.rescale = *rescale;
        }

        Ok(base)
    }
}

impl Face {
    pub fn empty() -> Face {
        Face {
            uv: (Vec2::ZERO, Vec2::ONE),
            texture: String::from(""),
            cullface: String::from(""),
            rotation: 0.0,
            tintindex: 0.0,
        }
    }

    pub fn parse(json: &serde_json::Value) -> Result<Face, Box<dyn Error>> {
        let mut base = Self::empty();

        // UV
        if let Some(serde_json::Value::Array(uv)) = json.get("uv") {
            if uv.len() != 4 {
                bail!("UV coordinates didn't have 4 values.");
            }

            base.uv.0.x =
                require_with!(uv.get(0).unwrap().as_f64(), "Couldn't read UV coordinate") as f32;
            base.uv.0.y =
                require_with!(uv.get(1).unwrap().as_f64(), "Couldn't read UV coordinate") as f32;
            base.uv.1.x =
                require_with!(uv.get(2).unwrap().as_f64(), "Couldn't read UV coordinate") as f32;
            base.uv.1.y =
                require_with!(uv.get(3).unwrap().as_f64(), "Couldn't read UV coordinate") as f32;
        }

        // Texture
        if let Some(serde_json::Value::String(texture)) = json.get("texture") {
            base.texture = texture.clone();
        }

        // Cullface
        if let Some(serde_json::Value::String(cullface)) = json.get("cullface") {
            base.cullface = cullface.clone();
        }

        // Rotation
        if let Some(serde_json::Value::Number(rotation)) = json.get("rotation") {
            base.rotation =
                require_with!(rotation.as_f64(), "Couldn't read face rotation value") as f32;
        }

        // Tint Index
        if let Some(serde_json::Value::Number(tintindex)) = json.get("tintindex") {
            base.tintindex =
                require_with!(tintindex.as_f64(), "Couldn't read face tint index") as f32;
        }

        Ok(base)
    }
}
