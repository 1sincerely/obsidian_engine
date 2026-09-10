use winit::{window::Window};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use log;

pub struct App {
    pub window: Option<Window>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = Window::default_attributes()
                .with_title("Obsidian Engine");

            let window = event_loop.create_window(window_attrs).unwrap();
            self.window = Some(window);
        }
    }
    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    )
    {
        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit()
            },

            WindowEvent::RedrawRequested => {
                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(size) => {
                log::info!("resized: {:?}", size)
            }
            _ => (),
        }
    }

    fn about_to_wait(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        
    }
}