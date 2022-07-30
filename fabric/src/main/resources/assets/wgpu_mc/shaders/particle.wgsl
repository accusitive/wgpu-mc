struct Uniforms {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;
};
[[group(1), binding(0)]]
var<uniform> uniform_data: Uniforms;

[[group(0), binding(0)]]
var t_texture: texture_2d<f32>;

[[group(0), binding(1)]]
var t_sampler: sampler;

struct VertexResult {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] tex_coords: vec2<f32>;
};

[[stage(vertex)]]
fn vs_main([[location(0)]] pos_in: vec3<f32>, [[location(1)]] tex_coords: vec2<f32>, [[location(2)]] pos2_in: vec3<f32>) -> VertexResult {
    var vr: VertexResult;
    vr.pos = uniform_data.proj * uniform_data.view * vec4<f32>(pos_in.x + pos2_in.x, pos_in.y + pos2_in.y, pos_in.z + pos2_in.z, 1.0);
    // var cam_right: vec3<f32> = vec3<f32>(uniform_data.view[0][0], uniform_data.view[1][0], uniform_data.view[2][0]);
    // var cam_up: vec3<f32> = vec3<f32>(uniform_data.view[0][1], uniform_data.view[1][1], uniform_data.view[2][1]);
    vr.tex_coords = tex_coords;

    return vr;
}



[[stage(fragment)]]
fn fs_main(in: VertexResult) -> [[location(0)]] vec4<f32> {
    var sample: vec4<f32> = textureSample(t_texture, t_sampler, in.tex_coords / 128.0);
    if (sample.w == 0.0) {
        discard;
    }
    return sample;
}