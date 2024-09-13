// vertex.rs

use bytemuck::{Pod, Zeroable};
use egui_wgpu::wgpu;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 2] =
        wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3];

    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        use std::mem;

        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }

    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self { position, color }
    }

    pub fn generate_cube() -> (Vec<Vertex>, Vec<u16>) {
        // Define the static vertices of a cube
        let vertices = vec![
            // Front face
            Vertex::new([-0.5, -0.5,  0.5], [1.0, 0.0, 0.0]),  // Bottom-left
            Vertex::new([ 0.5, -0.5,  0.5], [0.0, 1.0, 0.0]),  // Bottom-right
            Vertex::new([ 0.5,  0.5,  0.5], [0.0, 0.0, 1.0]),  // Top-right
            Vertex::new([-0.5,  0.5,  0.5], [1.0, 1.0, 0.0]),  // Top-left
            // Back face
            Vertex::new([-0.5, -0.5, -0.5], [1.0, 0.0, 1.0]),  // Bottom-left
            Vertex::new([ 0.5, -0.5, -0.5], [0.0, 1.0, 1.0]),  // Bottom-right
            Vertex::new([ 0.5,  0.5, -0.5], [1.0, 1.0, 1.0]),  // Top-right
            Vertex::new([-0.5,  0.5, -0.5], [0.5, 0.5, 0.5]),  // Top-left
        ];

        let indices = vec![
            // Front face
            0, 1, 2, 0, 2, 3,
            // Back face
            4, 5, 6, 4, 6, 7,
            // Left face
            4, 0, 3, 4, 3, 7,
            // Right face
            1, 5, 6, 1, 6, 2,
            // Top face
            3, 2, 6, 3, 6, 7,
            // Bottom face
            4, 5, 1, 4, 1, 0,
        ];

        (vertices, indices)
    }

    pub fn generate_polygon(sides: u16, radius: f32) -> (Vec<Vertex>, Vec<u16>) {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();
        let angle_step = 2.0 * std::f32::consts::PI / sides as f32;

        vertices.push(Vertex::new([0.0, 0.0, 0.0], [0.5, 0.0, 0.5]));  // Center vertex

        for i in 0..sides {
            let angle = i as f32 * angle_step;
            let x = radius * angle.cos();
            let y = radius * angle.sin();
            vertices.push(Vertex::new([x, y, 0.0], [0.5, 0.0, 0.5]));  // Adjust color as needed
        }

        for i in 0..sides {
            indices.push(0);  
            indices.push(i + 1);  
            indices.push((i + 1) % sides + 1);  
        }

        (vertices, indices)
    }
}
