use glium::{Display, Frame};
use imgui::{Context};
use imgui_glium_renderer::Renderer;
use imgui::*;

use crate::io::{keyboard::Keyboard, mouse::Mouse};

mod log_window;
use log_window::*;

mod chat_window;
use chat_window::*;

mod debug_window;
use debug_window::*;

mod entities_window;
use entities_window::*;

use super::{client::server::Server, logger::Logger};

pub struct Gui {
    pub imgui: Context,
    rend: Renderer,

    log: LogWindow,
    chat: ChatWindow,
    debug: DebugWindow,
    ents: EntitiesWindow,
}


impl Gui {

    pub fn new(imgui: Context, rend: Renderer) -> Gui {

        let mut gui = Gui {
            imgui,
            rend,

            log: LogWindow::new(),
            chat: ChatWindow::new(),
            debug: DebugWindow::new(),
            ents: EntitiesWindow::new(),
        };

        gui
    }

    pub fn get_ui(&mut self) -> Ui {
        self.imgui.frame()
    }





    pub fn render(&mut self, display: &Display, target: &mut Frame, log: &Logger, server: &mut Option<Server>) {

        let size = display.gl_window().window().inner_size();
        self.imgui.io_mut().display_size = [size.width as f32, size.height as f32];


        let ui = self.imgui.frame();

        match server {
            Some(serv) => {

                self.chat.render(&ui, &mut serv.chat);
                self.debug.render(&ui, serv);
                self.ents.render(&ui, serv);

            },
            None => {}
        }

        self.log.render(&ui, log);


        self.rend.render(target, ui.render()).unwrap();
    }





    pub fn update(&mut self, delta: f32, mouse: &Mouse, keyboard: &Keyboard) {
        self.imgui.io_mut().delta_time = delta;

        self.update_mouse(mouse);
    }





    pub fn update_mouse(&mut self, mouse: &Mouse) {
        let mut imgui = self.imgui.io_mut();

        imgui.mouse_pos = [mouse.get_pos().0 as f32, mouse.get_pos().1 as f32];

        imgui.mouse_down[0] = mouse.is_pressed(0);
        imgui.mouse_down[1] = mouse.is_pressed(1);
        imgui.mouse_down[2] = mouse.is_pressed(2);
        imgui.mouse_down[3] = mouse.is_pressed(3);
        imgui.mouse_down[4] = mouse.is_pressed(4);

        imgui.mouse_wheel = mouse.get_scroll().0;
        imgui.mouse_wheel_h = mouse.get_scroll().1;

    }




}