// ui.rs

use egui::Context;
use egui_wgpu::{wgpu::{self, Device}, ScreenDescriptor};

use crate::egui_tools::*;


pub struct UIState {
    pub sides: u16,
    pub rendering_style: RenderingStyle,
    pub scale_factor: f32,
    pub active_shader: &'static str,
}

impl UIState {
    pub fn new() -> Self {
        Self {
            sides: 5,
            rendering_style: RenderingStyle::Polygon,
            scale_factor: 1.0,
            active_shader: "main",
        }
    }

    pub fn draw_ui(
        &mut self,
        ctx: &Context,
        egui_renderer: &mut EguiRenderer,
        window: &winit::window::Window,
        device: &Device,
        surface_view: &wgpu::TextureView,
        screen_descriptor: ScreenDescriptor,
    ) {
        egui_renderer.draw(
            device,
            &egui_renderer.queue,
            &mut egui_renderer.encoder,
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

                        if ui.button("Switch Shader").clicked() {
                            if self.active_shader == "main" {
                                self.active_shader = "challenge"; // Switch to challenge shader
                            } else {
                                self.active_shader = "main"; // Switch back to main shader
                            }
                        }

                        ui.separator();

                        // Add the UI component to adjust the number of sides for polygons
                        if let RenderingStyle::Polygon = self.rendering_style {
                            ui.horizontal(|ui| {
                                ui.label(format!("Polygon sides: {}", self.sides));
                                if ui.button("-").clicked() {
                                    self.sides = (self.sides - 1).max(3); // Ensure a minimum of 3 sides
                                }
                                if ui.button("+").clicked() {
                                    self.sides = (self.sides + 1).min(12); // Set a max number of sides, for example, 12
                                }
                            });
                        }

                        // Add button to switch rendering style
                        ui.separator();
                        if ui.button("Switch to Cube").clicked() {
                            self.rendering_style = match self.rendering_style {
                                RenderingStyle::Polygon => RenderingStyle::Cube,
                                RenderingStyle::Cube => RenderingStyle::Polygon,
                            };
                        }

                        ui.separator();
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

// Rendering styles enum should probably stay here or be moved to a shared file
#[derive(Clone, Copy, Debug)]
pub enum RenderingStyle {
    Polygon,
    Cube,
}
