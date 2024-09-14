use winit_egui_wgpu::run;
mod camera;
mod vertex;
mod ui;
mod egui_tools;


fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}