use std::ops::RangeInclusive;

use egui::{Id, ScrollArea};
use glium_app::utils::persistent_window::PersistentWindow;

use crate::WindowManagerType;

pub fn new_options_window() -> PersistentWindow<WindowManagerType> {
    PersistentWindow::new(Box::new(move |id, _, gui_ctx, state| {
        let mut open = true;

        egui::Window::new("Settings")
            .id(Id::new(id))
            .open(&mut open)
            .show(gui_ctx, |ui| {
                ScrollArea::vertical().show(ui, |ui| {
                    ui.collapsing("Window", |ui| {
                        ui.label("No settings here yet");
                    });

                    ui.collapsing("Camera", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("FOV");
                            let mut fov = state.rend.cam.get_fov();
                            if ui
                                .add(egui::Slider::new(
                                    &mut fov,
                                    RangeInclusive::new(60.0, 120.0),
                                ))
                                .changed()
                            {
                                state.rend.cam.set_fov(fov);
                            }
                        });
                        ui.horizontal(|ui| {
                            ui.label("Gamma");
                            let mut fov = state.rend.cam.get_gamma();
                            if ui
                                .add(egui::Slider::new(&mut fov, RangeInclusive::new(0.1, 2.0)))
                                .changed()
                            {
                                state.rend.cam.set_gamma(fov);
                            }
                        });
                    });

                    ui.collapsing("Input", |ui| {
                        ui.horizontal(|ui| {
                            ui.label("Mouse sensitivity");
                            ui.add(egui::Slider::new(
                                &mut state.settings.mouse_sensitivity,
                                RangeInclusive::new(0.1, 10.0),
                            ));
                        });
                    });
                });
            });

        open
    }))
}
