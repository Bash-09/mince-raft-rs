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
    match &mut cli.server {
        Some(s) => {
            if cli.settings.show_fps {
                fps_counter::render(gui_ctx, t.fps(), t.delta());
            }

            if s.is_paused() {
                entities_window::render(gui_ctx, &s);
                info_window::render(gui_ctx, &s);

                match pause_menu::render(gui_ctx, &mut cli.state) {
                    PauseAction::Unpause => {
                        s.set_paused(false, &mut cli.state);
                    }
                    PauseAction::Disconnect => {
                        s.disconnect();
                        cli.server = None;
                    }
                    _ => {}
                }
            }
        }
        None => match main_menu::render(gui_ctx, &mut cli.settings) {
            Some(mut s) => {
                s.set_paused(false, &mut cli.state);
                cli.state.options_visible = false;
                cli.server = Some(s);
            }
            None => {}
        },
    }

    if cli.state.options_visible {
        options_window::render(gui_ctx, &mut cli.state, &mut cli.settings, &mut cli.rend);
    }
}
