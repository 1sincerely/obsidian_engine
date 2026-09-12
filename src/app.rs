use winit::{window::Window};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use log;
use pollster;
use std::sync::Arc;

use crate::mesh::{Mesh, Vertex};
use crate::renderer::Renderer;

pub struct App {
    pub window: Option<Arc<Window>>,
    pub renderer: Option<Renderer>,
    pub mesh: Option<Mesh>,
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if self.window.is_none() {
            let window_attrs = Window::default_attributes()
                .with_title("Obsidian Engine");

            let window = Arc::new(event_loop.create_window(window_attrs).unwrap());
            self.window = Some(window.clone());
            let renderer = pollster::block_on(Renderer::new(window)).unwrap();
            let vertices = [ 
                Vertex {position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex {position: [-0.5, -0.5, 0.0], color: [0.0, 1.0, 0.0]},
                Vertex {position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]},
                Vertex {position: [0.0, 0.5, 0.0], color: [1.0, 0.0, 0.0]},
                Vertex {position: [0.5, -0.5, 0.0], color: [0.0, 0.0, 1.0]},
                Vertex {position: [0.7, -0.12, 0.0], color: [0.4, 0.2, 0.4]},
                ];
            let mesh = Mesh::new(&renderer.device, &vertices);
            self.renderer = Some(renderer);
            self.mesh = Some(mesh);
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
                let renderer = self.renderer.as_mut().unwrap();
                let mesh = self.mesh.as_ref().unwrap();
                pollster::block_on(renderer.render(mesh)).unwrap();

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