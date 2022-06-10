use std::collections::HashMap;

use serde_json::Value;


#[derive(Debug)]
pub enum ModelError {
    NoParent,
    MissingElements,
    MissingModel,
    InvalidJsonValue,
}

pub struct Model {

}


impl Model {

    pub fn parse_model(data: &HashMap<String, Value>, key: &str) -> Result<Model, ModelError> {
        use ModelError::*;

        let model: Model;

        let map = data.get(key);
        if map.is_none() {return Err(MissingModel)}
        if map.unwrap().as_object().is_none() {return Err(InvalidJsonValue)}
        let map = map.unwrap().as_object().unwrap();

        let parent = data.get("parent");
        if let Some(Value::String(temp)) = parent {
            model = match Model::parse_model(data, temp) {
                Ok(model) => model,
                Err(_) => {
                    todo!()
                },
            }
        } else {
            model = Model {

            }
        }


        Ok(model)
    }

    fn parse_elements(data: &HashMap<String, Value>, key: &str) -> Result<Model, ModelError> {
        use ModelError::*;

        let map = data.get(key);
        if map.is_none() {return Err(MissingModel)}
        if map.unwrap().as_object().is_none() {return Err(InvalidJsonValue)}
        let map = map.unwrap().as_object().unwrap();


        todo!()
    }


}