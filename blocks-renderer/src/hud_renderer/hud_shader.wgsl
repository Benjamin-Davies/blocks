// Vertex shader

struct Camera {
    _matrix: mat4x4<f32>,
    _position: vec3<f32>,
    aspect: f32,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position: vec2<f32>,
    @location(1) texture_coords: vec2<f32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) texture_coords: vec2<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4(model.position / 16.0, 0.0, 1.0);
    out.clip_position.x /= camera.aspect;
    out.texture_coords = model.texture_coords;
    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    let color = textureSample(t_diffuse, s_diffuse, in.texture_coords);
    if (color.a == 0.0) {
        discard;
    } else {
        return color;
    }
}
