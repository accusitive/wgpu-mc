struct Uniforms {
    view_proj: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> uniform_data: Uniforms;

[[group(0), binding(0)]]
var t_texture: texture_2d<f32>;

[[group(0), binding(1)]]
var t_sampler: sampler;

[[stage(vertex)]]
fn vs_main([[location(0)]] pos_in: vec3<f32>,[[location(1)]] tex_coords: vec2<f32>) -> VertexResult {
      var vr: VertexResult;
        vr.pos = uniform_data.view_proj * vec4<f32>(pos_in, 1.0);
        vr.tex_coords = tex_coords;
}

struct VertexResult {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(fragment)]]
fn fs_main(in: VertexResult) -> [[location(0)]] vec4<f32> {
   return vec4(1.0, 0.0, 1.0, 1.0);
}