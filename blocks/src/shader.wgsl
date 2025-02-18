// Vertex shader

const BLOCK_COLORS = array<u32, 6>(
    0xff0000ff,
    0xffff00ff,
    0x00ff00ff,
    0x00ffffff,
    0x0000ffff,
    0xff00ffff,
);

@group(0) @binding(0)
var<uniform> camera: mat4x4<f32>;

struct VertexInput {
    @location(0) position_and_block_type: vec4<u32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let position = model.position_and_block_type.xyz;
    let block_type = model.position_and_block_type.w;

    var out: VertexOutput;
    out.clip_position = camera * vec4<f32>(vec3<f32>(position) / 8.0 - 1.0, 1.0);
    out.color = srgb_to_linear(BLOCK_COLORS[block_type]);
    return out;
}

// Fragment shader

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return in.color;
}

fn srgb_to_linear(color: u32) -> vec4<f32> {
    let srgb = vec4<f32>(
        f32((color >> 24) & 0xFF),
        f32((color >> 16) & 0xFF),
        f32((color >> 8) & 0xFF),
        f32(color & 0xFF)
    ) / 255.0;
    return vec4<f32>(
        pow((srgb.rgb + 0.055) / 1.055, vec3<f32>(2.4)),
        srgb.a
    );
}
