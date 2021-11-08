use imgui::*;

use crate::client::chat::Chat;

pub struct ChatWindow {
    last_scroll_y: f32,
}

impl ChatWindow {
    pub fn new() -> ChatWindow {
        ChatWindow { last_scroll_y: 0.0 }
    }

    pub fn render(&mut self, ui: &Ui, chat: &mut Chat) {
        Window::new("Chat")
            .size([400.0, 200.0], Condition::FirstUseEver)
            .position([275.0, 25.0], Condition::FirstUseEver)
            .build(&ui, || {
                ChildWindow::new("Chat History")
                    .size([0.0, ui.frame_height() - 45.0])
                    .build(&ui, || {
                        for msg in chat.get_history().iter() {
                            ui.text(&msg.text);
                        }

                        // Keep scroll at end of window
                        if self.last_scroll_y == ui.scroll_y() {
                            ui.set_scroll_y(ui.scroll_max_y());
                        }
                        self.last_scroll_y = ui.scroll_max_y();
                    });

                // Text input area
                let text = chat.get_message_mut();
                text.reserve(std::cmp::max(255 - text.len(), 0));
                let input_text = ui
                    .input_text(format!(""), text)
                    .enter_returns_true(true);
                chat.send = input_text.build();


                ui.same_line_with_pos(ui.window_size()[0] - 70.0);

                if !chat.send {
                    chat.send = ui.button("Send!");
                }
            });
    }
}
