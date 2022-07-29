use wgpu::{
    RenderPassDescriptor, RenderPipelineDescriptor, VertexAttribute, VertexBufferLayout,
    VertexFormat,
};

use crate::{
    mc::particle::ParticleInstanceTransforms,
    render::{
        particle::{ParticleInstances, ParticleVertex},
        shader::{WgslShader, WmShader},
    },
};

use super::WmPipeline;

pub struct ParticlePipeline<'particles> {
    pub particles: &'particles [&'particles ParticleInstances],
}

impl<'frames> WmPipeline for ParticlePipeline<'frames> {
    fn name(&self) -> &'static str {
        "wgpu_mc:pipelines/particle"
    }
    fn provide_shaders(
        &self,
        wm: &crate::WmRenderer,
    ) -> std::collections::HashMap<String, Box<dyn crate::render::shader::WmShader>> {
        [(
            "wgpu_mc:shaders/particle".into(),
            Box::new(
                WgslShader::init(
                    &"wgpu_mc:shaders/particle.wgsl".try_into().unwrap(),
                    &*wm.mc.resource_provider,
                    &wm.wgpu_state.device,
                    "fs_main".into(),
                    "vs_main".into(),
                )
                .unwrap(),
            ) as Box<dyn WmShader>,
        )]
        .into_iter()
        .collect()
    }
    fn atlases(&self) -> &'static [&'static str] {
        &[]
    }
    fn build_wgpu_pipeline_layouts(
        &self,
        wm: &crate::WmRenderer,
    ) -> std::collections::HashMap<String, wgpu::PipelineLayout> {
        let pipeline_manager = wm.render_pipeline_manager.load_full();
        let layouts = &pipeline_manager.bind_group_layouts.read();

        let mut map = std::collections::HashMap::new();
        map.insert(
            "wgpu_mc:layouts/particle".into(),
            wm.wgpu_state
                .device
                .create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Particle Pipeline Layout"),
                    bind_group_layouts: &[
                        layouts.get("texture").unwrap(),
                        layouts.get("matrix4").unwrap(),
                    ],
                    push_constant_ranges: &[],
                }),
        );
        map
    }
    fn build_wgpu_pipelines(
        &self,
        wm: &crate::WmRenderer,
    ) -> std::collections::HashMap<String, wgpu::RenderPipeline> {
        let pipeline_manager = wm.render_pipeline_manager.load_full();
        let layouts = &pipeline_manager.pipeline_layouts.load_full();
        let shader_map = pipeline_manager.shader_map.read();
        let shader = shader_map.get("wgpu_mc:shaders/particle").unwrap();

        let mut map = std::collections::HashMap::new();

        map.insert(
            "wgpu_mc:pipelines/particle".into(),
            wm.wgpu_state
                .device
                .create_render_pipeline(&RenderPipelineDescriptor {
                    label: None,
                    layout: Some(layouts.get("wgpu_mc:layouts/particle").unwrap()),
                    vertex: wgpu::VertexState {
                        module: shader.get_vert().0,
                        entry_point: shader.get_vert().1,
                        buffers: &[ParticleVertex::desc(), ParticleInstanceTransforms::desc()],
                    },
                    primitive: wgpu::PrimitiveState {
                        topology: wgpu::PrimitiveTopology::TriangleList,
                        strip_index_format: None,
                        front_face: wgpu::FrontFace::Cw,
                        // cull_mode: Some(wgpu::Face::Back),
                        cull_mode: None,
                        unclipped_depth: false,
                        polygon_mode: Default::default(),
                        conservative: false,
                    },
                    depth_stencil: Some(wgpu::DepthStencilState {
                        format: wgpu::TextureFormat::Depth32Float,
                        depth_write_enabled: true,
                        depth_compare: wgpu::CompareFunction::Less,
                        stencil: wgpu::StencilState::default(),
                        bias: wgpu::DepthBiasState::default(),
                    }),
                    multisample: wgpu::MultisampleState {
                        count: 1,
                        mask: !0,
                        alpha_to_coverage_enabled: false,
                    },
                    fragment: Some(wgpu::FragmentState {
                        module: shader.get_frag().0,
                        entry_point: shader.get_frag().1,
                        targets: &[wgpu::ColorTargetState {
                            format: wgpu::TextureFormat::Bgra8Unorm,
                            blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                            write_mask: Default::default(),
                        }],
                    }),
                    multiview: None,
                }),
        );

        map
    }

    fn render<
        'pipeline: 'render_pass,
        'wm,
        'pass_borrow,
        'render_pass: 'pass_borrow,
        'arena: 'pass_borrow + 'render_pass,
    >(
        &'pipeline self,

        wm: &'wm crate::WmRenderer,
        render_pass: &'pass_borrow mut wgpu::RenderPass<'render_pass>,
        arena: &'pass_borrow mut crate::util::WmArena<'arena>,
    ) {
        render_pass.set_pipeline(
            arena.alloc(
                wm.render_pipeline_manager
                    .load()
                    .render_pipelines
                    .load()
                    .get("wgpu_mc:pipelines/particle")
                    .unwrap()
                    .clone(),
            ),
        );

        self.particles.iter().for_each(|instances| {
            let uploaded = {
                let lock = instances.uploaded.read();
                arena.alloc(lock.as_ref().unwrap().clone())
            };

            let particle = arena.alloc(instances.particle.clone());

            render_pass.set_bind_group(0, &particle.texture.bind_group, &[]);
            render_pass.set_bind_group(
                1,
                (**arena.alloc(wm.mc.camera_bind_group.load_full()))
                    .as_ref()
                    .unwrap(),
                &[],
            );

            render_pass.set_vertex_buffer(0, particle.mesh.slice(..));
            render_pass.set_vertex_buffer(1, uploaded.instance_vbo.slice(..));
            render_pass.set_index_buffer(particle.index.slice(..), wgpu::IndexFormat::Uint16);

            render_pass.draw_indexed(0..6, 0, 0..instances.instances.len() as u32);
        });
    }
}
