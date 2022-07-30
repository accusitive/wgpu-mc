struct CameraUniform {
    view: mat4x4<f32>;
    proj: mat4x4<f32>;

};
[[group(0), binding(0)]]
var<uniform> projection: CameraUniform;

struct VertexResult {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] color: vec3<f32>;
};

[[stage(vertex)]]
fn vs_main(
    [[location(0)]] pos_in: vec3<f32>,
    [[location(1)]] color: vec3<f32>
) -> VertexResult {
    var vr: VertexResult;

    vr.pos = projection.proj * projection.view * vec4<f32>(pos_in, 1.0);

    vr.color = color;

    return vr;
}

[[stage(fragment)]]
fn fs_main(in: VertexResult) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.color, 1.0);
}