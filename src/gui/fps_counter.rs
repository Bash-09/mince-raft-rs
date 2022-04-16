use egui::{Align2, Color32, Context, Frame, RichText, Vec2};

pub fn render(gui_ctx: &Context, fps: u32, delta: f32) {
    let col: Color32;

    if fps < 60 {
        col = Color32::RED;
    } else {
        col = Color32::GREEN;
    }

    egui::Window::new("FPS Counter")
        .title_bar(false)
        .resizable(false)
        .anchor(Align2::LEFT_TOP, Vec2::new(5.0, 5.0))
        .frame(Frame::none())
        .show(gui_ctx, |ui| {
            ui.label(
                RichText::new(format!("FPS:  {}", fps))
                    .color(col)
                    .strong()
                    .heading(),
            );
            ui.label(
                RichText::new(format!("TIME: {:.2}ms", delta * 1000.0))
                    .color(col)
                    .strong()
                    .heading(),
            );
        });
}
