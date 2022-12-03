use egui::{Align2, Context, Vec2};
use glium_app::utils::persistent_window::PersistentWindowManager;

use crate::WindowManagerType;

use super::other_windows::options_window;

pub enum PauseAction {
    Nothing,
    Disconnect,
    Unpause,
}

/// Returns if the player has chosen to disconnect from the server
pub fn render(
    gui_ctx: &Context,
    wm: &mut PersistentWindowManager<WindowManagerType>,
) -> PauseAction {
    let mut paused = true;

    let mut out = PauseAction::Nothing;

    egui::Window::new("Game Paused!")
        .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
        .resizable(false)
        .collapsible(false)
        .open(&mut paused)
        .show(gui_ctx, |ui| {
            if ui.button("Settings").clicked() {
                wm.push(options_window::new_options_window());
            }

            if ui.button("Disconnect").clicked() {
                out = PauseAction::Disconnect;
            }
        });

    if !paused {
        out = PauseAction::Unpause;
    }

    out
}
