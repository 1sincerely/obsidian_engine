mod window;
mod app;

use app::App;
use winit::event_loop::EventLoop;

fn main() {
    env_logger::init();
    let event_loop = EventLoop::new().unwrap();
    let mut app = App { window: None };

    event_loop.run_app(&mut app).unwrap();
}
