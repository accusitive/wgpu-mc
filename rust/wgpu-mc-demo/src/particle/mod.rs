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

    let mut instances = vec![];
    for i in 0..100{
        for j in 0..100{
            instances.push(ParticleInstanceTransforms { position: [i as f32, 0.0, j as f32] });
        }
    }
    let pi = ParticleInstances::new(
        particle,
        instances,
    );
    pi.upload(wm);

    pi
}
