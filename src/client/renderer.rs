use glium::{Frame, Surface};

use super::{Client, server::Server};




pub struct Renderer {

}


impl Renderer {

    pub fn new() -> Renderer {

        Renderer {

        }
    }

    pub fn render_server(&mut self, target: &mut Frame, serv: &Server) {

        target.clear_color(0.5, 0.7, 0.8, 1.0);

        

    }

}