struct Uniforms {
    view_projection: mat4x4<f32>,
    // x = 1 when the target format is not sRGB and the shader must encode it.
    // yz = viewport size in physical pixels, needed to give lines a width.
    params: vec4<f32>,
};

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) color: vec4<f32>,
};

/// Everything about one end of a line once it is in clip space.
struct LineEnd {
    clip: vec4<f32>,
    color: vec4<f32>,
    width: f32,
};

fn mix_ends(a: LineEnd, b: LineEnd, t: f32) -> LineEnd {
    return LineEnd(mix(a.clip, b.clip, t), mix(a.color, b.color, t), mix(a.width, b.width, t));
}

const NEAR_W: f32 = 1e-5;

/// Lines are drawn as screen-space quads: `wgpu` has no line width, and a
/// one-pixel hairline is unreadable on a high-DPI display.
///
/// Each instance is one segment, expanded here into two triangles.
@vertex
fn vs_line(
    @builtin(vertex_index) index: u32,
    @location(0) start_position: vec3<f32>,
    @location(1) start_color: vec4<f32>,
    @location(2) start_width: f32,
    @location(3) end_position: vec3<f32>,
    @location(4) end_color: vec4<f32>,
    @location(5) end_width: f32,
) -> VertexOutput {
    var corners = array<u32, 6>(0u, 1u, 2u, 2u, 1u, 3u);
    let corner = corners[index];
    let at_end = corner >= 2u;
    let side = select(-1.0, 1.0, (corner & 1u) == 1u);

    var a = LineEnd(
        uniforms.view_projection * vec4<f32>(start_position, 1.0),
        start_color,
        start_width,
    );
    var b = LineEnd(
        uniforms.view_projection * vec4<f32>(end_position, 1.0),
        end_color,
        end_width,
    );

    var out: VertexOutput;

    // A segment crossing the camera plane has to be cut at the near plane:
    // dividing by a negative w would wrap it to the wrong side of the screen.
    if (a.clip.w < NEAR_W && b.clip.w < NEAR_W) {
        out.clip_position = vec4<f32>(0.0, 0.0, 0.0, 0.0);
        out.color = vec4<f32>(0.0);
        return out;
    }
    if (a.clip.w < NEAR_W) {
        a = mix_ends(a, b, (NEAR_W - a.clip.w) / (b.clip.w - a.clip.w));
    } else if (b.clip.w < NEAR_W) {
        b = mix_ends(b, a, (NEAR_W - b.clip.w) / (a.clip.w - b.clip.w));
    }

    var current = a;
    if (at_end) {
        current = b;
    }
    let resolution = max(uniforms.params.yz, vec2<f32>(1.0, 1.0));
    let screen_delta = (b.clip.xy / b.clip.w - a.clip.xy / a.clip.w) * resolution;

    var direction = vec2<f32>(1.0, 0.0);
    if (length(screen_delta) > 1e-6) {
        direction = normalize(screen_delta);
    }
    let normal = vec2<f32>(-direction.y, direction.x);
    let offset = normal * side * current.width / resolution;

    out.clip_position = vec4<f32>(
        (current.clip.xy / current.clip.w + offset) * current.clip.w,
        current.clip.z,
        current.clip.w,
    );
    out.color = current.color;
    return out;
}

@vertex
fn vs_solid(
    @location(0) position: vec3<f32>,
    @location(1) color: vec4<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = uniforms.view_projection * vec4<f32>(position, 1.0);
    out.color = color;
    return out;
}

fn linear_to_srgb(channel: f32) -> f32 {
    if (channel <= 0.0031308) {
        return channel * 12.92;
    }
    return 1.055 * pow(channel, 1.0 / 2.4) - 0.055;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if (uniforms.params.x > 0.5) {
        return vec4<f32>(
            linear_to_srgb(in.color.r),
            linear_to_srgb(in.color.g),
            linear_to_srgb(in.color.b),
            in.color.a,
        );
    }
    return in.color;
}
