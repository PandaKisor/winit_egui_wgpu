mod egui_tools;
mod camera;
mod vertex;

use crate::egui_tools::EguiRenderer;
use camera::Camera;
use vertex::Vertex;
use egui_wgpu::wgpu::{InstanceDescriptor, PowerPreference, RequestAdapterOptions, TextureFormat};
use egui_wgpu::{wgpu, ScreenDescriptor};
use glam::{Mat4, Vec3};
use std::collections::HashMap;
use std::sync::Arc;
use winit::dpi::PhysicalSize;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::keyboard::{Key, ModifiersState, NamedKey};
use wgpu::util::DeviceExt;

// Rendering styles enum
enum RenderingStyle {
    Polygon,
    Cube,
}

pub async fn run() {
    let event_loop = EventLoop::new().unwrap();

    let builder = winit::window::WindowBuilder::new().with_title("Voxxele");
    let window = builder.build(&event_loop).unwrap();
    let window = Arc::new(window);
    let initial_width = 1360;
    let initial_height = 768;
    window.request_inner_size(PhysicalSize::new(initial_width, initial_height));

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 2.0), Vec3::ZERO, 0.1);

    // Create the wgpu instance and surface
    let instance = egui_wgpu::wgpu::Instance::new(InstanceDescriptor::default());
    let surface = instance
        .create_surface(window.clone())
        .expect("Failed to create surface!");

    let power_pref = PowerPreference::default();
    let adapter = instance
        .request_adapter(&RequestAdapterOptions {
            power_preference: power_pref,
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        })
        .await
        .expect("Failed to find an appropriate adapter");

    let features = wgpu::Features::empty();
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: None,
                required_features: features,
                required_limits: Default::default(),
            },
            None,
        )
        .await
        .expect("Failed to create device");

    let swapchain_capabilities = surface.get_capabilities(&adapter);
    let selected_format = TextureFormat::Bgra8UnormSrgb;
    let swapchain_format = swapchain_capabilities
        .formats
        .iter()
        .find(|d| **d == selected_format)
        .expect("failed to select proper surface texture format!");

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: *swapchain_format,
        width: initial_width,
        height: initial_height,
        present_mode: wgpu::PresentMode::AutoVsync,
        desired_maximum_frame_latency: 0,
        alpha_mode: swapchain_capabilities.alpha_modes[0],
        view_formats: vec![],
    };

    surface.configure(&device, &config);

    // Load shaders
    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Main Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("shader.wgsl").into()),
    });
    
    let challenge_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Challenge Shader"),
        source: wgpu::ShaderSource::Wgsl(include_str!("challenge_shader.wgsl").into()),
    });
    
    // Pipeline compilation options
    let mut constants = HashMap::new();
    constants.insert("MY_CONSTANT".to_string(), 1.0); // Example constant value, replace as needed

    let compilation_options = wgpu::PipelineCompilationOptions {
        constants: &constants, // Pipeline-overridable constants
        zero_initialize_workgroup_memory: true, // Set based on your requirements
    };

    // Group vertex and fragment state creation together
    let vertex_state_main = wgpu::VertexState {
        module: &shader,
        entry_point: "vs_main",
        buffers: &[Vertex::desc()], // Use the Vertex description
        compilation_options: compilation_options.clone(), // Added compilation options
    };

    let fragment_state_main = wgpu::FragmentState {
        module: &shader,
        entry_point: "fs_main", // Entry point in your fragment shader
        targets: &[Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: compilation_options.clone(),
    };

    let vertex_state_challenge = wgpu::VertexState {
        module: &challenge_shader,
        entry_point: "vs_main",
        buffers: &[Vertex::desc()], // Use the Vertex description
        compilation_options: compilation_options.clone(),
    };

    let fragment_state_challenge = wgpu::FragmentState {
        module: &challenge_shader,
        entry_point: "fs_main", // Entry point in your fragment shader
        targets: &[Some(wgpu::ColorTargetState {
            format: config.format,
            blend: Some(wgpu::BlendState {
                color: wgpu::BlendComponent::REPLACE,
                alpha: wgpu::BlendComponent::REPLACE,
            }),
            write_mask: wgpu::ColorWrites::ALL,
        })],
        compilation_options: compilation_options.clone(),
    };

    // Create render pipeline layout
    let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Render Pipeline Layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });

    // Create the main render pipeline
    let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state_main,
        fragment: Some(fragment_state_main),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    // Create the challenge render pipeline
    let challenge_render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Challenge Render Pipeline"),
        layout: Some(&render_pipeline_layout),
        vertex: vertex_state_challenge,
        fragment: Some(fragment_state_challenge),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList,
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw,
            cull_mode: Some(wgpu::Face::Back),
            polygon_mode: wgpu::PolygonMode::Fill,
            unclipped_depth: false,
            conservative: false,
        },
        depth_stencil: None,
        multisample: wgpu::MultisampleState {
            count: 1,
            mask: !0,
            alpha_to_coverage_enabled: false,
        },
        multiview: None,
    });

    let mut sides: u16 = 5; 
    let mut rendering_style = RenderingStyle::Polygon; // Default to polygon
    let mut previous_sides = sides;

    // Generate polygon vertices and indices
    let (vertices, indices) = Vertex::generate_polygon(sides, 0.5);

    // Create the vertex buffer
    let mut vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Vertex Buffer"),
        contents: bytemuck::cast_slice(&vertices),
        usage: wgpu::BufferUsages::VERTEX,
    });

    // Create the index buffer
    let mut index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
        label: Some("Index Buffer"),
        contents: bytemuck::cast_slice(&indices),
        usage: wgpu::BufferUsages::INDEX,
    });

    let mut num_indices = indices.len() as u32;

    let mut egui_renderer = EguiRenderer::new(&device, config.format, None, 1, &window);

    let mut close_requested = false;
    let mut modifiers = ModifiersState::default();

    let mut scale_factor = 1.0;

    let mut active_shader = "main";

    event_loop.run(move |event, elwt| {
        elwt.set_control_flow(ControlFlow::Poll);

        match event {
            Event::WindowEvent { event, .. } => {
                egui_renderer.handle_input(&window, &event);

                match event {
                    WindowEvent::CloseRequested => {
                        close_requested = true;
                    }
                    WindowEvent::ModifiersChanged(new) => {
                        modifiers = new.state();
                    }
                    WindowEvent::KeyboardInput {
                        event: kb_event, ..
                    } => {
                        if kb_event.logical_key == Key::Named(NamedKey::Escape) {
                            close_requested = true;
                        }
                    }
                    WindowEvent::Resized(new_size) => {
                        config.width = new_size.width;
                        config.height = new_size.height;
                        surface.configure(&device, &config);
                    }
                    WindowEvent::RedrawRequested => {
                        if sides != previous_sides || matches!(rendering_style, RenderingStyle::Cube) {
                            let (new_vertices, new_indices) = match rendering_style {
                                RenderingStyle::Polygon => Vertex::generate_polygon(sides, 0.5),
                                RenderingStyle::Cube => Vertex::generate_cube(),  // Call generate_cube here
                            };
                            
                            vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("Vertex Buffer"),
                                contents: bytemuck::cast_slice(&new_vertices),
                                usage: wgpu::BufferUsages::VERTEX,
                            });
                    
                            index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                                label: Some("Index Buffer"),
                                contents: bytemuck::cast_slice(&new_indices),
                                usage: wgpu::BufferUsages::INDEX,
                            });
                    
                            num_indices = new_indices.len() as u32;
                            previous_sides = sides; // Update the previous_sides value
                        }
                    
                        let surface_texture = surface
                            .get_current_texture()
                            .expect("Failed to acquire next swap chain texture");
                
                        let surface_view = surface_texture
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());
                
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });
                
                        let screen_descriptor = ScreenDescriptor {
                            size_in_pixels: [config.width, config.height],
                            pixels_per_point: window.scale_factor() as f32 * scale_factor,
                        };
                
                        // Use the main render pipeline
                        {
                            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                                label: Some("Render Pass"),
                                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                                    view: &surface_view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                            r: 0.1,
                                            g: 0.2,
                                            b: 0.3,
                                            a: 1.0,
                                        }),
                                        store: wgpu::StoreOp::Store,
                                    },
                                })],
                                depth_stencil_attachment: None,
                                // Adding missing fields with default values
                                occlusion_query_set: None, // Default value, as occlusion queries aren't used
                                timestamp_writes: None,    // Default value, as no timestamps are written
                            });
                        
                            match active_shader {
                                "main" => render_pass.set_pipeline(&render_pipeline),
                                "challenge" => render_pass.set_pipeline(&challenge_render_pipeline),
                                _ => render_pass.set_pipeline(&render_pipeline), // Default fallback
                            }
                            render_pass.set_vertex_buffer(0, vertex_buffer.slice(..));
                            render_pass.set_index_buffer(index_buffer.slice(..), wgpu::IndexFormat::Uint16);
                            render_pass.draw_indexed(0..num_indices, 0, 0..1);
                        }                        
                
                        egui_renderer.draw(
                            &device,
                            &queue,
                            &mut encoder,
                            &window,
                            &surface_view,
                            screen_descriptor,
                            |ctx| {
                                egui::Window::new("UI Window")
                                    .resizable(true)
                                    .vscroll(true)
                                    .default_open(true)
                                    .show(ctx, |ui| {
                                        ui.label("Vertex and Shader control");
    
                                        if ui.button("Switch Shader").clicked() {
                                            if active_shader == "main" {
                                                active_shader = "challenge"; // Switch to challenge shader
                                            } else {
                                                active_shader = "main"; // Switch back to main shader
                                            }
                                        }
    
                                        ui.separator();
    
                                        // Add the UI component to adjust the number of sides for polygons
                                        if let RenderingStyle::Polygon = rendering_style {
                                            ui.horizontal(|ui| {
                                                ui.label(format!("Polygon sides: {}", sides));
                                                if ui.button("-").clicked() {
                                                    sides = (sides - 1).max(3); // Ensure a minimum of 3 sides
                                                }
                                                if ui.button("+").clicked() {
                                                    sides = (sides + 1).min(12); // Set a max number of sides, for example, 12
                                                }
                                            });
                                        }
    
                                        // Add button to switch rendering style
                                        ui.separator();
                                        if ui.button("Switch to Cube").clicked() {
                                            rendering_style = match rendering_style {
                                                RenderingStyle::Polygon => RenderingStyle::Cube,
                                                RenderingStyle::Cube => RenderingStyle::Polygon,
                                            };
                                        }
    
                                        ui.separator();
                                        ui.horizontal(|ui| {
                                            ui.label(format!(
                                                "Pixels per point: {}",
                                                ctx.pixels_per_point()
                                            ));
                                            if ui.button("-").clicked() {
                                                scale_factor = (scale_factor - 0.1).max(0.3);
                                            }
                                            if ui.button("+").clicked() {
                                                scale_factor = (scale_factor + 0.1).min(3.0);
                                            }
                                        });
                                    });
                            },
                        );
                
                        queue.submit(Some(encoder.finish()));
                        surface_texture.present();
                        window.request_redraw();
                    }
                    _ => {} // Wildcard pattern to catch all unhandled WindowEvent variants
                }                
            }

            Event::AboutToWait => {
                if close_requested {
                    elwt.exit()
                }
            }
            _ => {}
        }
    });
}
