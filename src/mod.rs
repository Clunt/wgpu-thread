use winit::event_loop::{ControlFlow, EventLoop};

mod app;
mod child;
mod viewport;

#[allow(unused)]
pub fn run() {
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    event_loop.set_control_flow(ControlFlow::Wait);

    let mut app = app::App::default();
    event_loop.run_app(&mut app).unwrap();
}
