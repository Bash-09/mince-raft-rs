#![allow(dead_code)]

use egui::Context;
use glium_app::Timer;

use crate::Client;

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
                entities_window::render(gui_ctx, &s);
                info_window::render(gui_ctx, &s);

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
