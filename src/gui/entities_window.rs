use std::collections::HashMap;

use egui::{Context, Id};
use resources::entities::ENTITIES;

use crate::{server::Server, entities::Entity};


pub fn render(gui_ctx: &Context, server: &Server) {

    egui::Window::new(format!("Entities: {}", server.get_entities().len()))
    .id(Id::new("Entities"))
    .show(gui_ctx, |ui| {

        let mut ents: HashMap<i32, Vec<&Entity>> = HashMap::new();
        for (id, e) in server.get_entities() {
            match ents.get_mut(&e.entity_type.id) {
                Some(vec) => {
                    vec.push(e);
                }
                None => {
                    ents.insert(e.entity_type.id, vec![e]);
                }
            }
        }

        // Dump entities into a vector
        let mut ents_vec: Vec<(&i32, &Vec<&Entity>)> = Vec::new();
        for (type_id, e) in ents.iter() {
            ents_vec.push((type_id, e));
        }
        // Sort by entity id
        ents_vec.sort_by(|(id1, _), (id2, _)| id1.cmp(id2));

        // List each present type of entity under dropdown menus
        for (type_id, ent) in ents_vec {
            let name = ENTITIES.get(type_id).unwrap().name;

            egui::CollapsingHeader::new(format!("{} ({})", name, ent.len()))
            .id_source(Id::new(name))
            .show(ui, |ui| {
                for e in ent {
                    ui.label(format!(
                        "{:.2} / {:.2} / {:.2}",
                        e.pos.x,
                        e.pos.y,
                        e.pos.z
                    ));
                }
            });

        }

    });

}
