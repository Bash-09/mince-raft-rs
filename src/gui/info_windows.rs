
use crate::server::Server;
use egui::Context;

pub mod entities_window;
pub mod server_info_window;

pub fn render(gui_ctx: &Context, server: &Server) {
    server_info_window::render(gui_ctx, server);
    entities_window::render(gui_ctx, server);
}
