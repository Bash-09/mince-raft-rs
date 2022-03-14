use egui::{Context, Align2, Rect, Vec2};

use crate::{settings::Settings, state::State};

pub enum PauseAction {
    Nothing,
    Disconnect,
    Unpause,
}

/// Returns if the player has chosen to disconnect from the server
pub fn render(gui_ctx: &Context, state: &mut State) -> PauseAction {

    let mut paused = true;

    let mut out = PauseAction::Nothing;

    egui::Window::new("Game Paused!")
    .anchor(Align2::CENTER_CENTER, Vec2::ZERO)
    .resizable(false)
    .collapsible(false)
    .open(&mut paused)
    .show(gui_ctx, |ui| {

        if ui.button("Settings").clicked() {
            state.options_visible = true;
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
