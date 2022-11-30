#![allow(dead_code)]

use egui::{Context, Id, Align2, Vec2};
use glium_app::{Timer, utils::persistent_window::PersistentWindow};

use crate::{Client, state::State};

use self::pause_menu::PauseAction;

pub mod entities_window;
pub mod fps_counter;
pub mod info_window;
pub mod main_menu;
pub mod options_window;
pub mod pause_menu;

pub fn render(gui_ctx: &Context, cli: &mut Client, t: &Timer) {
    match &mut cli.state.server {
        Some(s) => {
            if cli.state.settings.show_fps {
                fps_counter::render(gui_ctx, t.fps(), t.delta());
            }

            if s.is_paused() {
                entities_window::render(gui_ctx, s);
                info_window::render(gui_ctx, s);

                match pause_menu::render(gui_ctx, &mut cli.window_manager) {
                    PauseAction::Unpause => {
                        s.set_paused(false);
                    }
                    PauseAction::Disconnect => {
                        s.disconnect();
                        cli.state.server = None;
                    }
                    _ => {}
                }
            }
        }
        None => match main_menu::render(gui_ctx, cli) {
            Some(mut s) => {
                s.set_paused(false);
                cli.state.server = Some(s);
            }
            None => {}
        },
    }
}

pub fn disconnect_window(reason: Option<String>) -> PersistentWindow<State> {
    PersistentWindow::new(Box::new(
        move |id, gui_ctx, _| {
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
        },
    ))
}
