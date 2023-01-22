// use imgui::*;
//
// use crate::client::chat::Chat;
//
// pub struct ChatWindow {
//     last_scroll_y: f32,
// }
//
// impl ChatWindow {
//     pub fn new() -> ChatWindow {
//         ChatWindow { last_scroll_y: 0.0 }
//     }
//
//     pub fn render(&mut self, ui: &Ui, chat: &mut Chat) {
//         Window::new("Chat")
//             .size([400.0, 200.0], Condition::FirstUseEver)
//             .position([275.0, 25.0], Condition::FirstUseEver)
//             .build(&ui, || {
//                 ChildWindow::new("Chat History")
//                     .size([0.0, ui.frame_height() - 45.0])
//                     .build(&ui, || {
//                         for msg in chat.get_history().iter() {
//                             ui.text(&msg.text);
//                         }
//
//                         // Keep scroll at end of window
//                         if self.last_scroll_y == ui.scroll_y() {
//                             ui.set_scroll_y(ui.scroll_max_y());
//                         }
//                         self.last_scroll_y = ui.scroll_max_y();
//                     });
//
//                 // Text input area
//                 let text = chat.get_message_mut();
//                 text.reserve(std::cmp::max(255 - text.len(), 0));
//                 let input_text = ui
//                     .input_text(format!(""), text)
//                     .enter_returns_true(true);
//                 chat.send = input_text.build();
//
//
//                 ui.same_line_with_pos(ui.window_size()[0] - 70.0);
//
//                 if !chat.send {
//                     chat.send = ui.button("Send!");
//                 }
//             });
//     }
// }
const CHAT_TIME: i64 = 300;

use egui::{Align, Align2, Color32, Context, Frame, Layout, RichText, Stroke, Vec2};
use mcproto_rs::v1_16_3::PlayServerChatMessageSpec;

use crate::server::Server;

pub fn render_inactive(server: &Server, gui_ctx: &Context) {
    let messages: Vec<&(PlayServerChatMessageSpec, i64)> = server
        .get_chat()
        .get_history()
        .iter()
        .rev()
        .filter(|m| server.get_world_time() - m.1 < CHAT_TIME)
        .collect();

    if !messages.is_empty() {
        egui::Window::new("Chat_Active")
            .title_bar(false)
            .resizable(false)
            .anchor(Align2::LEFT_BOTTOM, Vec2::new(5.0, -50.0))
            .frame(Frame::none())
            .show(gui_ctx, |ui| {
                ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                    ui.add_space(ui.text_style_height(&egui::TextStyle::Body) + 9.0);
                    for message in messages {
                        if let Some(text) = message.0.message.to_traditional() {
                            ui.label(
                                RichText::new(text).color(Color32::WHITE).background_color(
                                    Color32::from_rgba_unmultiplied(0, 0, 0, 175),
                                ),
                            );
                        }
                    }
                });
            });
    }
}

pub fn render_active(server: &mut Server, gui_ctx: &Context) {
    egui::Window::new("Chat_Active")
        .title_bar(false)
        .resizable(false)
        .fixed_size(Vec2::new(500.0, 500.0))
        .anchor(Align2::LEFT_BOTTOM, Vec2::new(5.0, -50.0))
        .frame(Frame::none())
        .show(gui_ctx, |ui| {
            ui.with_layout(Layout::bottom_up(Align::LEFT), |ui| {
                ui.text_edit_singleline(server.get_chat_mut().get_current_message_mut())
                    .request_focus();
                ui.add_space(5.0);

                egui::ScrollArea::vertical().show(ui, |ui| {
                    for message in server.get_chat().get_history().iter().rev() {
                        if let Some(text) = message.0.message.to_traditional() {
                            ui.label(
                                RichText::new(text).color(Color32::WHITE).background_color(
                                    Color32::from_rgba_unmultiplied(0, 0, 0, 175),
                                ),
                            );
                        }
                    }
                });

                // let len = server.get_chat().get_history().len();
                // egui::ScrollArea::vertical().show_rows(
                //     ui,
                //     ui.text_style_height(&egui::TextStyle::Body),
                //     len,
                //     |ui, range| {
                //         for i in range {
                //             let message = &server.get_chat().get_history()[len - i - 1];
                //             if let Some(text) = message.0.message.to_traditional() {
                //                 ui.label(RichText::new(text).color(Color32::WHITE).background_color(Color32::from_rgba_unmultiplied(0, 0, 0, 175)));
                //             }
                //         }
                //     });
            });
        });
}
