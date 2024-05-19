use std::collections::HashMap;

use super::child::Child;
use winit::{
    application::ApplicationHandler,
    event::{KeyEvent, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::{self, KeyCode},
    window::WindowId,
};

#[derive(Default)]
pub struct App {
    children: HashMap<WindowId, Child>,
}

impl App {
    pub fn create_child(&mut self, event_loop: &ActiveEventLoop) {
        let mut child = Child::new(event_loop);
        child.run();

        self.children.insert(child.id(), child);
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_child(event_loop);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping {id:?}");
                // let _ = self.window.take(); // winit@0.30.1 will fixed
                self.children.remove(&id);

                if self.children.len() == 0 {
                    event_loop.exit();
                }
            }
            WindowEvent::KeyboardInput {
                event:
                    KeyEvent {
                        state: winit::event::ElementState::Pressed,
                        physical_key: keyboard::PhysicalKey::Code(KeyCode::KeyN),
                        ..
                    },
                ..
            } => {
                println!("press N 创建新窗口");
                self.create_child(event_loop);
            }
            event => {
                if let Some(child) = self.children.get_mut(&id) {
                    child.window_event(event);
                }
            }
        }
    }
}
