use std::sync::Arc;

use parking_lot::RwLock;
use wgpu::util::{DeviceExt, BufferInitDescriptor};

use crate::{mc::particle::{Particle, ParticleInstanceTransforms, UploadedParticleInstances}, WmRenderer};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct ParticleVertex {
    pub position: [f32; 3],
    pub tex_coords: [f32; 2],
}
impl ParticleVertex {
    #[must_use]
    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        use std::mem;
        wgpu::VertexBufferLayout {
            array_stride: mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                //Position
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                //Texcoords
                wgpu::VertexAttribute {
                    offset: mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

pub struct ParticleInstances {
    pub(crate) particle: Arc<Particle>,
    pub instances: Vec<ParticleInstanceTransforms>,
    pub(crate) uploaded: RwLock<Option<UploadedParticleInstances>>,
}
impl ParticleInstances {
    pub fn new(particle: Arc<Particle>, instances: Vec<ParticleInstanceTransforms>) -> Self {
        Self {
            particle,
            instances,
            uploaded: RwLock::new(None),
        }
    }

    pub fn upload(&self, wm: &WmRenderer) {

        let instances_bytes = bytemuck::cast_slice(&self.instances[..]);

        let instance_vbo = Arc::new(wm
            .wgpu_state
            .device
            .create_buffer_init(
                &BufferInitDescriptor {
                    label: None,
                    contents: instances_bytes,
                    usage: wgpu::BufferUsages::VERTEX
                }
            ));
            
        *self.uploaded.write() = Some(UploadedParticleInstances {
            count: self.instances.len() as u32,
            instance_vbo
        })
    }
}
