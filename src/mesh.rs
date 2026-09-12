use bytemuck::{Pod, Zeroable};
use wgpu;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Zeroable, Pod, Copy, Clone)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub num_vertices: u32,
}

impl Mesh {
    pub fn new(device: &wgpu::Device, vertices: &[Vertex]) -> Self {
        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("mesh vertex buffer"),
            contents: bytemuck::cast_slice(vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertex_buffer,
            num_vertices: vertices.len() as u32,
        }
    }

    pub fn draw<'a>(&'a self, render_pass: &mut wgpu::RenderPass<'a>) {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));

        render_pass.draw(0..self.num_vertices, 0..1);
    }
}

