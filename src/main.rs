//main.rs

use winit_egui_wgpu::run;


fn main() {
    #[cfg(not(target_arch = "wasm32"))]
    {
        pollster::block_on(run());
    }
}