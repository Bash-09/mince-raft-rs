use std::ops::RangeInclusive;

use egui::{Context, ScrollArea};

use crate::{renderer::Renderer, settings::Settings, state::State};

pub fn render(gui_ctx: &Context, state: &mut State, settings: &mut Settings, rend: &mut Renderer) {
    egui::Window::new("Settings")
        .open(&mut state.options_visible)
        .show(gui_ctx, |ui| {
            ScrollArea::vertical().show(ui, |ui| {
                ui.collapsing("Window", |ui| {
                    ui.label("No settings here yet");
                });

                ui.collapsing("Camera", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("FOV");
                        let mut fov = rend.cam.get_fov();
                        if ui
                            .add(egui::Slider::new(
                                &mut fov,
                                RangeInclusive::new(60.0, 120.0),
                            ))
                            .changed()
                        {
                            rend.cam.set_fov(fov);
                        }
                    });
                });

                ui.collapsing("Input", |ui| {
                    ui.horizontal(|ui| {
                        ui.label("Mouse sensitivity");
                        ui.add(egui::Slider::new(
                            &mut settings.mouse_sensitivity,
                            RangeInclusive::new(0.1, 10.0),
                        ));
                    });
                });
            });
        });
}
