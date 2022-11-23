use std::collections::HashMap;

use egui_extras::RetainedImage;
use glium::Display;

use mcproto_rs::status;

use crate::{renderer::Renderer, server::Server, settings::Settings};

pub struct State {
    pub rend: Renderer,

    pub settings: Settings,
    pub server: Option<Server>,

    pub outstanding_server_pings: HashMap<String, Server>,
    pub server_pings: HashMap<String, status::StatusSpec>,
    pub icon_handles: HashMap<String, RetainedImage>,
}

impl State {
    pub fn new(dis: &Display) -> State {
        State {
            rend: Renderer::new(dis),

            settings: match Settings::load("settings.json") {
                Ok(s) => s,
                Err(e) => {
                    log::error!("Couldn't load settings: {:?}", e);
                    Settings::default()
                }
            },
            server: None,

            outstanding_server_pings: HashMap::new(),
            server_pings: HashMap::new(),
            icon_handles: HashMap::new(),
        }
    }
}
