use std::collections::HashMap;

use __core::any::Any;
use imgui::*;

use crate::app::client::{entities::{ENTITIES, Entity}, server::Server};


pub struct EntitiesWindow {
    last_scroll_y: f32,
}


impl EntitiesWindow {

    pub fn new() -> EntitiesWindow {
        EntitiesWindow {
            last_scroll_y: 0.0,
        }
    }

    pub fn render(&mut self, ui: &Ui, serv: &mut Server) {

        Window::new(im_str!("Entities"))
        .size([225.0, 300.0], Condition::FirstUseEver)
        .position([25.0, 25.0], Condition::FirstUseEver)
        .build(&ui, || {

            ui.text(im_str!("Entities: {}", serv.entities.len()));

            let mut ents: HashMap<i32, Vec<&Entity>> = HashMap::new();
            for(id, e) in &serv.entities {
                match ents.get_mut(&e.entity_type) {
                    Some(vec) => {
                        vec.push(e);
                    },
                    None => {
                        ents.insert(e.entity_type, vec![e]);
                    }
                }
            }
            let mut ents_vec: Vec<(&i32, &Vec<&Entity>)> = Vec::new();
            for(type_id, e) in ents.iter() {
                ents_vec.push((type_id, e));
            }
            ents_vec.sort_by(|(id1, _), (id2, _)| {
                id1.cmp(id2)
            });

            for(type_id, ent) in ents_vec {
                let name = ENTITIES[*type_id as usize].name;
                if CollapsingHeader::new(&im_str!("{} ({})", name, ent.len())).build(ui) {

                    for e in ent {

                        // if CollapsingHeader::new(&im_str!("")).build(ui) {
                        ui.text(im_str!("Pos: {:.2} / {:.2} / {:.2}", e.pos.get_x(), e.pos.get_y(), e.pos.get_z()));
                        // }

                    }

                }
            }

        });

    }


}