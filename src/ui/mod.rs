//ui.rs

pub mod egui_tools;

use egui_tools::EguiRenderer;
use egui_wgpu::{
    wgpu::{Device, Queue, CommandEncoder, TextureView},
    ScreenDescriptor,
};
use winit::window::Window;



pub struct UIState {
    pub scale_factor: f32,
    pub active_shader: &'static str,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            scale_factor: 1.0,
            active_shader: "main",
        }
    }

    pub fn draw_ui(
        &mut self,
        egui_renderer: &mut EguiRenderer,
        window: &Window,
        device: &Device,
        queue: &Queue,
        encoder: &mut CommandEncoder,
        surface_view: &TextureView,
        screen_descriptor: ScreenDescriptor,
    ) {
        egui_renderer.draw(
            device,
            queue,
            encoder,
            window,
            surface_view,
            screen_descriptor,
            |ctx| {
                egui::Window::new("UI Window")
                    .resizable(true)
                    .vscroll(true)
                    .default_open(true)
                    .show(ctx, |ui| {
                        ui.label("Vertex and Shader control");

                        // Button to switch shaders
                        if ui.button("Switch Shader").clicked() {
                            if self.active_shader == "main" {
                                self.active_shader = "challenge"; // Switch to challenge shader
                            } else {
                                self.active_shader = "main"; // Switch back to main shader
                            }
                        }

                        ui.separator();

                        // Adjust pixel density
                        ui.horizontal(|ui| {
                            ui.label(format!("Pixels per point: {}", ctx.pixels_per_point()));
                            if ui.button("-").clicked() {
                                self.scale_factor = (self.scale_factor - 0.1).max(0.3);
                            }
                            if ui.button("+").clicked() {
                                self.scale_factor = (self.scale_factor + 0.1).min(3.0);
                            }
                        });
                    });
            },
        );
    }
}
