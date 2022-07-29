use std::sync::Arc;

use wgpu_mc::{
    mc::{
        particle::{Particle, ParticleInstanceTransforms},
        resource::ResourcePath,
    },
    render::{atlas::Atlas, particle::ParticleInstances},
    texture::BindableTexture,
    WmRenderer,
};

pub fn make_particle(wm: &WmRenderer) -> ParticleInstances {
    let angry: ResourcePath = "minecraft:textures/particle/angry.png".into();
    let angry_resource = wm.mc.resource_provider.get_bytes(&angry).unwrap();
    let test_particle_atlas = Atlas::new(&*wm.wgpu_state, &*wm.render_pipeline_manager.load_full());
    test_particle_atlas.allocate([(&angry, &angry_resource)], &*wm.mc.resource_provider);
    test_particle_atlas.upload(wm);

    let particle = Arc::new(Particle::new(&wm.wgpu_state, test_particle_atlas.bindable_texture.clone()));
    let pi = ParticleInstances::new(
        particle,
        vec![ParticleInstanceTransforms {
            position: [0.0, 0.0, 0.0],
        },
        ParticleInstanceTransforms {
            position: [1.0, 0.0, 0.0],
        },
        ParticleInstanceTransforms {
            position: [0.0, 0.0, 1.0],
        },
        ParticleInstanceTransforms {
            position: [1.0, 0.0, 1.0],
        }],
    );
    pi.upload(wm);

    pi
}
