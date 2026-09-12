use wgpu::{self, Device, Queue, RenderPipeline, Surface, SurfaceConfiguration, hal::SurfaceError};
use winit::{dpi::PhysicalSize, window::Window};
use anyhow;
use std::sync::Arc;
use crate::mesh;

pub struct Renderer {
    surface: Surface<'static>,
    pub device: Device,
    queue: Queue,
    surface_configuration: SurfaceConfiguration,
    render_pipeline: RenderPipeline,
    
}

impl Renderer {
    pub async fn new(window: Arc<Window>) -> anyhow::Result<Renderer> {
        let size = window.inner_size();

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            flags: Default::default(),
            memory_budget_thresholds: Default::default(),
            backend_options: Default::default(),
            display: None,

        });

        let surface = instance.create_surface(window.clone()).unwrap();

        let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
            apply_limit_buckets: true,
        })
        .await?;

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor {
                label: None,
                required_features: wgpu::Features::empty(),
                experimental_features: wgpu::ExperimentalFeatures::disabled(),
                required_limits: if cfg!(target_arch = "wasm32") {
                    wgpu::Limits::downlevel_webgl2_defaults()
                } else {
                    wgpu::Limits::default()
                },
                memory_hints: Default::default(),
                trace: wgpu::Trace::Off,
            })
            .await?;

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps.formats.iter()
            .find(|f| f.is_srgb())
            .copied()
            .unwrap_or(surface_caps.formats[0]);

        let surface_configuration = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
            color_space: wgpu::SurfaceColorSpace::Auto,
        };

        surface.configure(&device, &surface_configuration);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("shader"),
            source: wgpu::ShaderSource::Wgsl(include_str!("../shaders/shader.wgsl").into()),
        });

        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor { 
            label: Some("render pipeline layout"), 
            bind_group_layouts: &[], 
            immediate_size: 0, 
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor { 
            label: Some("render pipeline"), 
            layout: Some(&render_pipeline_layout), 
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: &[Some(wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<mesh::Vertex>() as u64,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float32x3,
                            offset: 12,
                            shader_location: 1,
                        }
                    ],
                })],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }, 
            primitive: wgpu::PrimitiveState::default(), 
            depth_stencil: None, 
            multisample: wgpu::MultisampleState::default(), 
            fragment: Some(wgpu::FragmentState { 
                module: &shader, 
                entry_point: Some("fs_main"), 
                compilation_options: wgpu::PipelineCompilationOptions::default(), 
                targets: &[Some(wgpu::ColorTargetState { 
                    format: surface_configuration.format, 
                    blend: Some(wgpu::BlendState::REPLACE), 
                    write_mask: wgpu::ColorWrites::ALL, 
                })], 
            }), 
            multiview_mask: None, 
            cache: None, 
        });

        Ok(Self {
            surface,
            device,
            queue,
            surface_configuration,
            render_pipeline,
        })
        
    }

    fn resize(&mut self, size: PhysicalSize<u32>) {
        todo!()
    }

pub async fn render(&mut self, mesh: &mesh::Mesh) -> Result<(), SurfaceError> {
        let frame = match self.surface.get_current_texture() {
            wgpu::CurrentSurfaceTexture::Success(frame) => frame,
            wgpu::CurrentSurfaceTexture::Lost => {
                panic!("Surface lost")
            }
            wgpu::CurrentSurfaceTexture::Outdated => {
                self.surface.configure(&self.device, &self.surface_configuration);
                return Ok(())
            }
            wgpu::CurrentSurfaceTexture::Timeout | wgpu::CurrentSurfaceTexture::Occluded => {
                return Ok(())
            }
            wgpu::CurrentSurfaceTexture::Validation => {
                panic!("Validation error")
            }
            wgpu::CurrentSurfaceTexture::Suboptimal(frame) => {

                frame
            }
        };

        let view = frame.texture.create_view(&Default::default());
        let mut encoder = self.device.create_command_encoder(&Default::default());
        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor { 
            label: Some("render pass"), 
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                depth_slice: None,
                resolve_target: None,
                ops: wgpu::Operations { 
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.1,
                        g: 0.2,
                        b: 0.3,
                        a: 1.0,
                    }), 
                    store: wgpu::StoreOp::Store, 
                }
                
            })], 
            depth_stencil_attachment: None, 
            timestamp_writes: None, 
            occlusion_query_set: None, 
            multiview_mask: None, 
        });

        render_pass.set_pipeline(&self.render_pipeline);
        mesh.draw(&mut render_pass);
        drop(render_pass);
        self.queue.submit(Some(encoder.finish()));
        self.queue.present(frame);
    Ok(())
}

}