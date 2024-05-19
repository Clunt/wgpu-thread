use super::viewport::Viewport;
use std::sync::{Arc, Mutex};
use winit::{
    event::WindowEvent,
    event_loop::ActiveEventLoop,
    window::{Window, WindowId},
};

#[derive(Debug)]
enum ChildEvent {
    WindowEvent(WindowEvent),
}

pub struct Child {
    window: Arc<Window>,
    viewport: Arc<Mutex<Viewport>>,
    sender: Option<std::sync::mpsc::Sender<ChildEvent>>,
}

impl Child {
    pub fn new(event_loop: &ActiveEventLoop) -> Self {
        let window = Arc::new(
            event_loop
                .create_window(Window::default_attributes())
                .unwrap(),
        );
        let viewport = Arc::new(Mutex::new(Viewport::new(window.clone())));
        Self {
            window,
            viewport,
            sender: None,
        }
    }

    pub fn id(&self) -> WindowId {
        self.window.id()
    }

    pub fn run(&mut self) {
        println!("child run {:?}", self.id());
        let (sender, receiver) = std::sync::mpsc::channel();
        let viewport = self.viewport.clone();
        self.sender = Some(sender);

        std::thread::Builder::new()
            .spawn(move || {
                {
                    let mut viewport = viewport.lock().unwrap();
                    viewport.build();
                }

                while let Ok(event) = receiver.recv() {
                    match event {
                        ChildEvent::WindowEvent(window_event) => match window_event {
                            WindowEvent::Resized(size) => viewport.lock().unwrap().resize(size),
                            WindowEvent::RedrawRequested => viewport.lock().unwrap().paint(),
                            _ => {}
                        },
                    }
                }
            })
            .unwrap();
    }

    pub fn window_event(&mut self, event: WindowEvent) {
        if let Some(sender) = &self.sender {
            sender.send(ChildEvent::WindowEvent(event)).unwrap();
        }
    }
}
