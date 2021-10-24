use imgui::*;

use crate::logger::*;

pub struct LogWindow {
    last_scroll_y: f32,
}


impl LogWindow {

    pub fn new() -> LogWindow {
        LogWindow {
            last_scroll_y: 0.0,
        }
    }

    pub fn render(&mut self, ui: &Ui, log: &Logger) {

        Window::new(im_str!("Log"))
            .size([950.0, 225.0], Condition::FirstUseEver)
            .position([25.0, 350.0], Condition::FirstUseEver)
            .build(&ui, || {

                let history = log.get_log();

                for l in history.iter() {
                    ui.text(l.to_string());
                }


                // Keep scroll at end of window
                if self.last_scroll_y == ui.scroll_y() {
                    ui.set_scroll_y(ui.scroll_max_y());
                }
                self.last_scroll_y = ui.scroll_max_y();

            });

    }


}