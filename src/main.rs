extern crate chrono;
extern crate egui;
extern crate glium;
extern crate glium_app;
extern crate lazy_static;
extern crate log;
extern crate quartz_nbt;

extern crate mcnetwork;

use std::{time::Instant, collections::HashMap, sync::mpsc::TryRecvError};

use crate::network::*;

mod network;

use egui::{FontData, FontDefinitions, FontFamily};
use egui_extras::RetainedImage;
use egui_winit::winit::{
    event::Event,
    window::{Icon, WindowBuilder},
};
use glam::Vec3;
use glium::glutin;
use log::{debug, error, info};

use glium_app::context::Context;
use glium_app::*;
use mcnetwork::packets::{encode, PlayerPositionAndRotation};
use settings::SETTINGS;
use state::State;

pub mod chat;
pub mod entities;
pub mod gui;
pub mod player;
pub mod renderer;
pub mod resources;
pub mod server;
pub mod settings;
pub mod state;
pub mod world;

use self::{renderer::Renderer, server::Server};

fn main() {
    env_logger::init();
    debug!("Starting logger");

    let wb = WindowBuilder::new()
        .with_title("Minceraft!")
        .with_resizable(true)
        .with_window_icon(Some(
            Icon::from_rgba(include_bytes!("../assets/img.bmp")[70..].to_vec(), 512, 512).unwrap(),
        ))
        .with_inner_size(glutin::dpi::PhysicalSize::new(1200i32, 700i32));

    let (ctx, el) = glium_app::create(wb);

    let client = Client::new(&ctx);

    glium_app::run_with_context(client, ctx, el);
}

pub struct Client {
    pub rend: Renderer,

    pub server: Option<Server>,

    pub outstanding_server_pings: HashMap<String, Server>,
    pub server_pings: HashMap<String, ServerStatus>,
    pub icon_handles: HashMap<String, RetainedImage>,

    pub state: State,

    period: f32,
    last_mod: f32,
}

impl Application for Client {
    fn init(&mut self, ctx: &mut Context) {
        let mut fonts = FontDefinitions::default();

        fonts.font_data.insert(
            "minecraft".to_string(),
            FontData::from_static(include_bytes!("../minecraft_font.ttf")),
        );

        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "minecraft".to_owned());

        fonts
            .families
            .get_mut(&FontFamily::Monospace)
            .unwrap()
            .insert(0, "minecraft".to_owned());

        ctx.gui.egui_ctx.set_fonts(fonts);

        let dims = ctx.dis.get_framebuffer_dimensions();
        let aspect = dims.0 as f32 / dims.1 as f32;
        self.rend.cam.set_aspect_ratio(aspect);


        let s = SETTINGS.read().expect("Failed to initialize settings.");
        info!("Saved servers: {}", s.saved_servers.len());

        std::thread::spawn(|| {
            let start = Instant::now();
            resources::BLOCKS.len();
            resources::ENTITIES.len();
            resources::MODELS.len();
            let dur = Instant::now() - start;

            info!("Loading assets took {}ms", dur.as_millis());
        });
    }

    fn update(&mut self, t: &glium_app::timer::Timer, ctx: &mut glium_app::context::Context) {
        let delta = t.delta();
        let time = t.absolute_time();

        // Runs some code only once every self.period seconds
        let modulus = time % self.period;
        if modulus < self.last_mod {
            match &self.server {
                Some(serv) => {
                    // Send player position update packets
                    if serv.get_player().id != 0 {
                        serv.send_packet(encode(PlayerPositionAndRotation {
                            x: (serv.get_player().get_position().x as f64),
                            feet_y: (serv.get_player().get_position().y as f64),
                            z: (serv.get_player().get_position().z as f64),
                            yaw: (serv.get_player().get_orientation().get_yaw() as f32),
                            pitch: (serv.get_player().get_orientation().get_head_pitch() as f32),
                            on_ground: (true),
                        }));
                    }
                }
                None => {}
            }
        }
        self.last_mod = modulus;

        // Runs some code while the server is valid
        match &mut self.server {
            Some(serv) => {
                // Update camera
                self.rend
                    .cam
                    .set_pos(serv.get_player().get_position().clone());
                self.rend.cam.translate(Vec3::new(0.0, 1.7, 0.0));
                self.rend
                    .cam
                    .set_rot(serv.get_player().get_orientation().get_rotations() * -1.0);

                serv.update(ctx, &mut self.state, delta);

                if serv.disconnect {
                    self.server = None;
                }
            }
            None => {
                let Client { outstanding_server_pings, server_pings, .. } = self;
                outstanding_server_pings.retain(|k, v| {

                    match v.network.recv.try_recv() {
                        Ok(NetworkCommand::ReceiveStatus(status)) => {
                            server_pings.insert(k.clone(), status);
                            return false;
                        },
                        Err(TryRecvError::Disconnected) => {
                            return false;
                        },
                        _ => {}
                    }

                    true
                });
            }
        }

        // *********************** RENDER ***************************8
        let Context {
            dis,
            gui,
            mouse,
            keyboard,
        } = ctx;

        let mut target = dis.draw();

        // Render world if it exists
        if let Some(s) = &self.server {
            self.rend.render_server(&mut target, s);
        }

        // GUI
        let _repaint = gui.run(&dis, |gui_ctx| {
            gui::render(gui_ctx, self, t);
        });
        gui.paint(dis, &mut target);

        ctx.set_mouse_grabbed(self.state.mouse_grabbed).ok();
        ctx.set_mouse_visible(self.state.mouse_visible);

        target.finish().unwrap();
    }

    fn close(&mut self) {

        match SETTINGS.read().expect("Failed to get settings to save").save("settings.json") {
            Ok(_) => {
                info!("Saved settings!");
            },
            Err(e) => {
                error!("Failed to save settings: {:?}", e);
            }
        }

        match &self.server {
            Some(serv) => {
                serv.send_command(NetworkCommand::Disconnect)
                    .expect("Failed to send disconnect command to network commander.");
            }
            None => {}
        }

        debug!("Closing App");
    }

    fn handle_event(&mut self, ctx: &mut Context, event: &Event<()>) {
        match event {
            Event::WindowEvent {
                window_id: _,
                event: glutin::event::WindowEvent::Focused(focused),
            } => {
                if let Some(server) = &mut self.server {
                    if !focused {
                        self.state.mouse_grabbed = false;
                        self.state.mouse_visible = true;
                        server.set_paused(true, &mut self.state);
                    }
                }
            }
            Event::WindowEvent {
                window_id: _,
                event: glutin::event::WindowEvent::Resized(new),
            } => {
                let aspect = new.width as f32 / new.height as f32;
                self.rend.cam.set_aspect_ratio(aspect);
            }
            _ => {}
        }
    }
}

impl Client {
    pub fn new(ctx: &Context) -> Client {
        Client {
            rend: Renderer::new(&ctx.dis),

            server: None,

            state: State::new(),

            server_pings: HashMap::new(),
            outstanding_server_pings: HashMap::new(),
            icon_handles: HashMap::new(),

            period: 0.05,
            last_mod: 0.0,
        }
    }
}
