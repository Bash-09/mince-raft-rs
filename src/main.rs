extern crate glium;
extern crate chrono;
extern crate imgui;
extern crate imgui_glium_renderer;

mod timer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use timer::*;

mod app;
use app::*;

pub mod network;

pub mod io;

use glium::{*, glutin::{ContextBuilder, dpi::{PhysicalSize, Size}, window::WindowBuilder}};
use imgui::Context;
use imgui_glium_renderer::Renderer;

fn main() {

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = WindowBuilder::new()
        .with_title("Mince-Raft")
        .with_inner_size(PhysicalSize::new(1000, 600));
    let cb = ContextBuilder::new()
        .with_vsync(false);
    let display = Display::new(wb, cb, &event_loop).expect("Failed to open Display!");

    let mut imgui = Context::create();
    imgui.set_ini_filename(None);

    //Stuff to handle Imgui input
    let mut platform = WinitPlatform::init(&mut imgui);
    platform.attach_window(imgui.io_mut(), display.gl_window().window(), HiDpiMode::Default);

    let renderer = Renderer::init(&mut imgui, &display).expect("Failed to initialise renderer!");
    let gui = gui::Gui::new(imgui, renderer);



    let mut s: App = App::new(display, gui);
    let mut t = Timer::new();

    t.reset();
    event_loop.run(move |ev, _, control_flow| {

        // Imgui handle events
        platform.handle_event(s.gui.imgui.io_mut(), s.dis.gl_window().window(), &ev);

        use glutin::event::WindowEvent;

        // Handle our own events
        let mut events_cleared = false;
        use glutin::event::{Event::*, *};
        match &ev {
            glutin::event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => {
                    println!("Closing Application");
                    *control_flow = glutin::event_loop::ControlFlow::Exit;
                },
                WindowEvent::CursorMoved { device_id, position, .. } => {
                    s.mouse.update_pos((position.x as i32, position.y as i32));
                },
                WindowEvent::MouseInput { device_id , state , button, .. } => {
                    let mut mbutton: u16 = 0;
                    match button {
                        MouseButton::Left => {mbutton = 0;},
                        MouseButton::Middle => {mbutton = 1;},
                        MouseButton::Right => {mbutton = 2;},
                        MouseButton::Other(bnum) => {
                            if bnum > &(9 as u16) {return}
                            mbutton = *bnum;
                        }
                    }
                    let mut pressed = false;
                    if state == &ElementState::Pressed {
                        pressed = true;
                    }
                    if pressed {
                        s.mouse.press_button(mbutton as usize);
                    } else {
                        s.mouse.release_button(mbutton as usize);
                    }
                },
                WindowEvent::MouseWheel {device_id, delta, ..} => {
                    match delta {
                        MouseScrollDelta::LineDelta(y, x) => {
                            s.mouse.scroll((*x, *y));
                        }
                        _ => {}
                    }
                },
                WindowEvent::AxisMotion {device_id, axis, value} => {

                },
                WindowEvent::KeyboardInput { device_id, input, is_synthetic, ..} => match input {
                    KeyboardInput { scancode, state, virtual_keycode, .. } => {
                        match virtual_keycode {
                            None => {},
                            Some(key) => {
                                if state == &ElementState::Pressed {
                                    s.keyboard.press(*key);
                                } else {
                                    s.keyboard.release(*key);
                                }
                            }
                        }
                    }
                },
                WindowEvent::ReceivedCharacter(char) => {

                },
                _ => {
                    //println!("Unhandled event: {:?}", ev);
                },
            },
            MainEventsCleared => {
                events_cleared = true;
            },
            RedrawEventsCleared => {},
            NewEvents(cause) => {
                match cause {
                    StartCause::Init => {
                        s.init();
                    },
                    _ => {}
                }
            },
            _ => {
                //println!("Unhandled event: {:?}", ev);
            },
        }

        if !events_cleared {
            return;
        }

        // Update
        match t.go() {
            None => {},
            Some(_) => {
                s.update(&t);
                s.render();

                s.mouse.next_frame();
                s.keyboard.next_frame();
                s.gui.update(t.delta(), &s.mouse, &s.keyboard);
            }
        }
        
    });

}
