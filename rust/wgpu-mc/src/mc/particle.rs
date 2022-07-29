use std::sync::Arc;

use bytemuck::{Zeroable, Pod};
use wgpu::util::{BufferInitDescriptor, DeviceExt};

use crate::{render::particle::ParticleVertex, texture::BindableTexture, WgpuState};

use super::entity::Position;
const PARTICLE_VERTICIES: [ParticleVertex; 4] = [
    ParticleVertex {
        position: [1.0, 1.0, 0.0],
        tex_coords: [1.0, 1.0],
    },
    // Bottom right
    ParticleVertex {
        position: [1.0, 0.0, 0.0],
        tex_coords: [1.0, 0.0],
    },
    // Bottom left
    ParticleVertex {
        position: [0.0, 0.0, 0.0],
        tex_coords: [0.0, 0.0],
    },
    ParticleVertex {
        position: [0.0, 1.0, 0.0],
        tex_coords: [0.0, 1.0],
    },
];
#[derive(Debug)]
pub struct Particle {
    pub texture: Arc<BindableTexture>,
    pub mesh: Arc<wgpu::Buffer>,
    pub index: Arc<wgpu::Buffer>,
}
impl Particle {
    pub fn new(wgpu_state: &WgpuState, texture: Arc<BindableTexture>) -> Self {
      

        Self {
            texture,
            index: Arc::new(wgpu_state.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: &[0, 1, 3, 1, 2, 3],
                usage: wgpu::BufferUsages::INDEX,
            })),
            mesh: Arc::new(wgpu_state.device.create_buffer_init(&BufferInitDescriptor {
                label: None,
                contents: bytemuck::cast_slice(&PARTICLE_VERTICIES[..]),
                usage: wgpu::BufferUsages::VERTEX,
            })),
        }
    }
}
#[derive(Copy, Clone, Zeroable, Pod)]
#[repr(C)]
// serves like EntityInstanceVBOEntry aswell as EntityInstanceTransforms
pub struct ParticleInstanceTransforms {
    // Uses entity position, maybe bad?
    pub position: [f32; 3]
}
impl ParticleInstanceTransforms {
    // 0 is Float32x3 for position
    // 1 is Float32x2 for texcoords
    const VAA: [wgpu::VertexAttribute; 1] = wgpu::vertex_attr_array![
        // Position2
        2 => Float32x3,
    ];
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::VAA
        }
    }
}
#[derive(Clone)]
pub(crate) struct UploadedParticleInstances {
    pub(crate) instance_vbo: Arc<wgpu::Buffer>,
    pub(crate) count: u32
}

// impl ParticleInstanceTransforms {
//     pub fn get_matrices(&self, particle: &Particle) -> Vec<[[f32;4]; 4]> {

//     }
// }