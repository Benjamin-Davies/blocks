// Vertex shader

const CORNFLOWER_BLUE: vec4<f32> = vec4<f32>(0.4, 0.6, 0.9, 1.0);

struct Camera {
    matrix: mat4x4<f32>,
    position: vec3<f32>,
    _aspect: f32,
}

@group(0) @binding(0)
var<uniform> camera: Camera;

struct VertexInput {
    @location(0) position_and_block_type: vec4<u32>,
    @location(1) normal_and_padding: vec4<i32>,
    @location(2) subchunk_position: vec3<i32>,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) light_intensity: f32,
    @location(1) block_type: u32,
    @location(2) texture_coords: vec2<f32>,
    @location(3) relative_position: vec3<f32>,
};

@vertex
fn vs_main(
    model: VertexInput,
) -> VertexOutput {
    let position = vec3<f32>(model.position_and_block_type.xyz);
    let block_type = model.position_and_block_type.w;
    let normal = model.normal_and_padding.xyz;
    let subchunk_position = vec3<f32>(model.subchunk_position);
    var out: VertexOutput;

    let world_position = position + 16.0 * subchunk_position;
    out.clip_position = camera.matrix * vec4(world_position, 1.0);
    out.relative_position = world_position - camera.position;

    let light_direction = normalize(vec3<f32>(1.0, 3.0, -2.0));
    let value = 0.5 + 0.5 * max(0.0, dot(vec3<f32>(normal), light_direction));
    out.light_intensity = value;
    out.block_type = block_type;

    var texture_coords: vec2<f32> = vec2<f32>(1.0, 0.0);
    if (normal.x == -1) {
        texture_coords = vec2(position.z, -position.y);
    }
    if (normal.x == 1) {
        texture_coords = vec2(-position.z, -position.y);
    }
    if (normal.y == -1) {
        texture_coords = vec2(position.x, -position.z);
    }
    if (normal.y == 1) {
        texture_coords = vec2(-position.x, -position.z);
    }
    if (normal.z == -1) {
        texture_coords = vec2(-position.x, -position.y);
    }
    if (normal.z == 1) {
        texture_coords = vec2(position.x, -position.y);
    }
    out.texture_coords = texture_coords / 2.0;

    return out;
}

// Fragment shader

@group(1) @binding(0)
var t_diffuse: texture_2d<f32>;
@group(1) @binding(1)
var s_diffuse: sampler;

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {

    let texture_coords = (in.texture_coords + 16.0) % 1.0;
    let texture_position = vec2(f32(in.block_type), 0.0);
    let atlas_coords = (texture_coords + texture_position) / vec2(4.0, 1.0);

    let sample = textureSample(t_diffuse, s_diffuse, atlas_coords);
    let world_color = darken(sample, in.light_intensity);

    let too_far = clamp((length(in.relative_position) - 40.0) / 8.0, 0.0, 1.0);
    return world_color + too_far * (CORNFLOWER_BLUE - world_color);
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

fn darken(color: vec4<f32>, value: f32) -> vec4<f32> {
    return vec4(color.rgb * value, color.a);
}
