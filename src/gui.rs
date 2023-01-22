#![allow(dead_code)]

use egui::{Align2, Context, Id, Vec2};
use glium_app::{utils::persistent_window::PersistentWindow, Timer};

use crate::{server::InputState, state::State, Client};

use self::other_windows::fps_counter;

pub mod chat_windows;
pub mod info_windows;
pub mod other_windows;
pub mod pause_windows;

pub mod main_menu;

pub fn render(gui_ctx: &Context, cli: &mut Client, t: &Timer) {
    match &mut cli.state.server {
        Some(s) => {
            if cli.state.settings.show_fps {
                fps_counter::render(gui_ctx, t.fps(), t.delta());
            }

            s.render(gui_ctx, &mut cli.window_manager);
        }
        None => match main_menu::render(gui_ctx, cli) {
            Some(mut s) => {
                s.set_input_state(InputState::Playing);
                cli.state.server = Some(s);
            }
            None => {}
        },
    }
}

pub fn disconnect_window(reason: Option<String>) -> PersistentWindow<State> {
    PersistentWindow::new(Box::new(move |id, _, gui_ctx, _| {
        let mut open = true;

        egui::Window::new("Disconnected")
            .id(Id::new(id))
            .resizable(false)
            .collapsible(false)
            .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
            .show(gui_ctx, |ui| {
                let mut label = None;

                ui.horizontal(|ui| {
                    ui.add_space(15.0);
                    label = Some(ui.label(match &reason {
                        Some(r) => r,
                        None => "No reason Specified.",
                    }));
                });

                ui.horizontal(|ui| {
                    let size = label.unwrap().rect.width() / 2.0;
                    ui.add_space(size);
                    open = !ui.button("Ok").clicked();
                    ui.add_space(size);
                });
            });

        open
    }))
}
