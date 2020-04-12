use std::sync::{Arc, Mutex};

use shaderc;
use winit::{
    dpi::LogicalSize,
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};
use zerocopy::AsBytes;

use crate::PixelData;

pub trait DisplayProgram: 'static + Sized {
    fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        image: Arc<Mutex<PixelData>>,
    ) -> (Self, Option<wgpu::CommandBuffer>);
    fn resize(
        &mut self,
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
    ) -> Option<wgpu::CommandBuffer>;
    fn update(&mut self, event: WindowEvent);
    fn render(
        &mut self,
        frame: &wgpu::SwapChainOutput,
        device: &wgpu::Device,
    ) -> wgpu::CommandBuffer;
}

pub struct RayTracingGUI {
    image: Arc<Mutex<PixelData>>,
    width: u32,
    height: u32,
    vertex_buf: wgpu::Buffer,
    index_buf: wgpu::Buffer,
    index_count: usize,
    texture: wgpu::Texture,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl RayTracingGUI {
    fn create_vertices() -> (Vec<Vertex>, Vec<u16>) {
        let vertices = vec![
            Vertex::new([-1.0, -1.0, 0.0], [0.0, 1.0]),
            Vertex::new([1.0, -1.0, 0.0], [1.0, 1.0]),
            Vertex::new([1.0, 1.0, 0.0], [1.0, 0.0]),
            Vertex::new([-1.0, 1.0, 0.0], [0.0, 0.0]),
        ];

        let indices = vec![0, 1, 3, 1, 2, 3];
        (vertices, indices)
    }
}

impl DisplayProgram for RayTracingGUI {
    fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &wgpu::Device,
        width: u32,
        height: u32,
        image: Arc<Mutex<PixelData>>,
    ) -> (Self, Option<wgpu::CommandBuffer>) {
        let vertex_size = std::mem::size_of::<Vertex>();
        let (vertex_data, index_data) = RayTracingGUI::create_vertices();

        let vertex_buf =
            device.create_buffer_with_data(vertex_data.as_bytes(), wgpu::BufferUsage::VERTEX);

        let index_buf =
            device.create_buffer_with_data(index_data.as_bytes(), wgpu::BufferUsage::INDEX);

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            bindings: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: false,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                },
            ],
            label: None,
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            bind_group_layouts: &[&bind_group_layout],
        });

        let texture_extent = wgpu::Extent3d {
            width,
            height,
            depth: 1,
        };

        let texture = device.create_texture(&wgpu::TextureDescriptor {
            size: texture_extent,
            array_layer_count: 1,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
            label: None,
        });

        let texture_view = texture.create_default_view();

        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            lod_min_clamp: -100.0,
            lod_max_clamp: 100.0,
            compare: wgpu::CompareFunction::Undefined,
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &bind_group_layout,
            bindings: &[
                wgpu::Binding {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::Binding {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
            ],
            label: None,
        });

        let vs_bytes = compile_shader(
            include_str!("../shaders/shader.vert"),
            "shader.vert",
            shaderc::ShaderKind::Vertex,
        );
        let fs_bytes = compile_shader(
            include_str!("../shaders/shader.frag"),
            "shader.frag",
            shaderc::ShaderKind::Fragment,
        );
        let vs_module = device.create_shader_module(&vs_bytes);
        let fs_module = device.create_shader_module(&fs_bytes);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            layout: &pipeline_layout,
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: vertex_size as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float4,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float2,
                            offset: 4 * 4, // TODO
                            shader_location: 1,
                        },
                    ],
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        let program = RayTracingGUI {
            image,
            width,
            height,
            vertex_buf,
            index_buf,
            index_count: index_data.len(),
            texture,
            bind_group,
            pipeline,
        };

        (program, None)
    }

    fn resize(
        &mut self,
        _: &wgpu::SwapChainDescriptor,
        _: &wgpu::Device,
    ) -> Option<wgpu::CommandBuffer> {
        None
    }

    fn update(&mut self, _: WindowEvent) {}

    fn render(
        &mut self,
        frame: &wgpu::SwapChainOutput,
        device: &wgpu::Device,
    ) -> wgpu::CommandBuffer {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let image_buf = self.image.lock().unwrap().as_image();
        let texels = image_buf.as_bytes();
        let texture_extent = wgpu::Extent3d {
            width: self.width,
            height: self.height,
            depth: 1,
        };

        let temp_buf = device.create_buffer_with_data(texels, wgpu::BufferUsage::COPY_SRC);
        encoder.copy_buffer_to_texture(
            wgpu::BufferCopyView {
                buffer: &temp_buf,
                offset: 0,
                bytes_per_row: 4 * self.width,
                rows_per_image: 0,
            },
            wgpu::TextureCopyView {
                texture: &self.texture,
                mip_level: 0,
                array_layer: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            texture_extent,
        );

        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    load_op: wgpu::LoadOp::Clear,
                    store_op: wgpu::StoreOp::Store,
                    clear_color: wgpu::Color::BLACK,
                }],
                depth_stencil_attachment: None,
            });
            rpass.set_pipeline(&self.pipeline);
            rpass.set_bind_group(0, &self.bind_group, &[]);
            rpass.set_index_buffer(&self.index_buf, 0, 0);
            rpass.set_vertex_buffer(0, &self.vertex_buf, 0, 0);
            rpass.draw_indexed(0..self.index_count as u32, 0, 0..1);
        }
        encoder.finish()
    }
}

pub async fn run_async<D: DisplayProgram>(width: u32, height: u32, image: Arc<Mutex<PixelData>>) {
    let event_loop = EventLoop::new();

    let (window, size, surface) = {
        let window = winit::window::WindowBuilder::new()
            .with_title("Ray Tracer")
            .with_inner_size(LogicalSize::new(width as f64, height as f64))
            .with_resizable(false)
            .build(&event_loop)
            .unwrap();
        let size = window.inner_size();
        let surface = wgpu::Surface::create(&window);
        (window, size, surface)
    };

    let adapter = wgpu::Adapter::request(
        &wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::Default,
            compatible_surface: Some(&surface),
        },
        wgpu::BackendBit::PRIMARY,
    )
    .await
    .unwrap();

    let (device, queue) = adapter
        .request_device(&wgpu::DeviceDescriptor {
            extensions: wgpu::Extensions {
                anisotropic_filtering: false,
            },
            limits: wgpu::Limits::default(),
        })
        .await;

    let mut sc_desc = wgpu::SwapChainDescriptor {
        usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
        format: wgpu::TextureFormat::Bgra8UnormSrgb,
        width: size.width,
        height: size.height,
        present_mode: wgpu::PresentMode::Mailbox,
    };

    let mut swap_chain = device.create_swap_chain(&surface, &sc_desc);

    let (mut program, init_cmd_buf) = D::init(&sc_desc, &device, width, height, image);
    if let Some(cmd_buf) = init_cmd_buf {
        queue.submit(&[cmd_buf]);
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            event::Event::MainEventsCleared => window.request_redraw(),
            event::Event::WindowEvent {
                event: WindowEvent::Resized(size),
                ..
            } => {
                sc_desc.width = size.width;
                sc_desc.height = size.height;
                swap_chain = device.create_swap_chain(&surface, &sc_desc);
                let command_buf = program.resize(&sc_desc, &device);
                if let Some(command_buf) = command_buf {
                    queue.submit(&[command_buf]);
                }
            }
            event::Event::WindowEvent { event, .. } => match event {
                WindowEvent::KeyboardInput {
                    input:
                        event::KeyboardInput {
                            virtual_keycode: Some(event::VirtualKeyCode::Escape),
                            state: event::ElementState::Pressed,
                            ..
                        },
                    ..
                }
                | WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                }
                _ => {
                    program.update(event);
                }
            },
            event::Event::RedrawRequested(_) => {
                let frame = swap_chain
                    .get_next_texture()
                    .expect("Timeout when acquiring next swap chain texture");
                let command_buf = program.render(&frame, &device);
                queue.submit(&[command_buf]);
            }
            _ => {}
        }
    })
}

pub fn run<D: DisplayProgram>(width: usize, height: usize, image: Arc<Mutex<PixelData>>) {
    futures::executor::block_on(run_async::<D>(width as u32, height as u32, image));
}

#[repr(C)]
#[derive(AsBytes)]
struct Vertex {
    pos: [f32; 4],
    texcoords: [f32; 2],
}

impl Vertex {
    fn new(coords: [f32; 3], texcoords: [f32; 2]) -> Vertex {
        Vertex {
            pos: [coords[0], coords[1], coords[2], 1.0],
            texcoords,
        }
    }
}

fn compile_shader(code: &str, path: &str, kind: shaderc::ShaderKind) -> Vec<u32> {
    let mut compiler = shaderc::Compiler::new().unwrap();
    let mut options = shaderc::CompileOptions::new().unwrap();
    options.set_warnings_as_errors();
    options.set_generate_debug_info();
    let binary_result = compiler
        .compile_into_spirv(code, kind, path, "main", Some(&options))
        .unwrap();
    log::warn!("{}", binary_result.get_warning_messages());
    wgpu::read_spirv(std::io::Cursor::new(binary_result.as_binary_u8())).unwrap()
}
