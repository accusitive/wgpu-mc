struct CameraUniform {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;

};

struct Transforms {
    mats: array<mat4x4<f32>>;
};

[[group(3), binding(0)]]
var<uniform> uniform_data: Uniforms;

[[group(0), binding(0)]]
var<storage> transforms: Transforms;

struct VertexResult {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
    [[location(1)]] normal: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] pos_in: vec3<f32>,
    [[location(1)]] tex_coords: vec2<f32>,
    [[location(2)]] normal: vec3<f32>,
    [[location(3)]] part_id: u32,
    [[location(4)]] entity_index: u32,
    [[location(5)]] entity_texture_index: u32,
    [[location(6)]] parts_per: u32
) -> VertexResult {
    var vr: VertexResult;

    var part_transform_index: u32 = entity_index * parts_per;
    var part_transform: mat4x4<f32> = transforms.mats[part_transform_index];

    //vr.pos = uniform_data.view_proj * part_transform * vec4<f32>(pos_in, 0.0);
    vr.pos = uniform_data.proj * uniform_data.view * part_transform * vec4<f32>(pos_in, 1.0);
    vr.tex_coords = tex_coords;
    vr.normal = mat3x3<f32>(part_transform[0].xyz, part_transform[1].xyz, part_transform[2].xyz) * normal;

    return vr;
}

[[group(2), binding(0)]]
var t_texture: texture_2d<f32>;

[[group(2), binding(1)]]
var t_sampler: sampler;

[[stage(fragment)]]
fn fs_main(in: VertexResult) -> [[location(0)]] vec4<f32> {
   return textureSample(t_texture, t_sampler, in.tex_coords);
}