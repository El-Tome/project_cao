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

/// Smallest `w` (distance in front of the camera) a line vertex may keep.
const NEAR_W: f32 = 1e-4;

/// How far outside the screen a line is still drawn, in screen widths.
/// Everything beyond is invisible anyway, and cutting there keeps coordinates
/// small enough that adding a half-width to them stays exact in f32.
const NDC_LIMIT: f32 = 2.0;

/// Signed distance of a clip-space point to one of the five planes we clip
/// against, positive inside. Kept in clip space — dividing by `w` first would
/// blow up for points near the camera and wreck the precision of the cut.
fn plane_distance(clip: vec4<f32>, plane: u32) -> f32 {
    switch plane {
        case 0u: { return clip.w - NEAR_W; }
        case 1u: { return clip.x + NDC_LIMIT * clip.w; }
        case 2u: { return NDC_LIMIT * clip.w - clip.x; }
        case 3u: { return clip.y + NDC_LIMIT * clip.w; }
        default: { return NDC_LIMIT * clip.w - clip.y; }
    }
}

/// The sub-range of the segment `a`→`b` that survives all five planes, as a
/// pair of parameters. Returns an empty range when nothing survives.
fn clip_range(a: vec4<f32>, b: vec4<f32>) -> vec2<f32> {
    var enter = 0.0;
    var exit = 1.0;

    for (var plane = 0u; plane < 5u; plane++) {
        let start = plane_distance(a, plane);
        let end = plane_distance(b, plane);

        if (start < 0.0 && end < 0.0) {
            return vec2<f32>(1.0, 0.0);
        }
        if (start >= 0.0 && end >= 0.0) {
            continue;
        }

        let crossing = start / (start - end);
        if (start < 0.0) {
            enter = max(enter, crossing);
        } else {
            exit = min(exit, crossing);
        }
    }

    return vec2<f32>(enter, exit);
}

/// Lines are drawn as screen-space quads: `wgpu` has no line width, and a
/// one-pixel hairline is unreadable on a high-DPI display.
///
/// Each instance is one segment, expanded here into two triangles. The quad is
/// emitted directly in normalized device coordinates (w = 1), which keeps the
/// width exact however far the line reaches. The depth is carried across by
/// hand, so that a line can be hidden by the part standing in front of it.
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

    var out: VertexOutput;
    // A dropped segment collapses to a single point, which rasterizes to
    // nothing. Emitting w = 0 instead would be a division by zero, and drivers
    // disagree on what that draws.
    out.clip_position = vec4<f32>(0.0, 0.0, 0.0, 1.0);
    out.color = vec4<f32>(0.0);

    let a = LineEnd(
        uniforms.view_projection * vec4<f32>(start_position, 1.0),
        start_color,
        start_width,
    );
    let b = LineEnd(
        uniforms.view_projection * vec4<f32>(end_position, 1.0),
        end_color,
        end_width,
    );

    let range = clip_range(a.clip, b.clip);
    if (range.x > range.y) {
        return out;
    }

    let clipped_a = mix_ends(a, b, range.x);
    let clipped_b = mix_ends(a, b, range.y);
    let screen_a = clipped_a.clip.xy / clipped_a.clip.w;
    let screen_b = clipped_b.clip.xy / clipped_b.clip.w;

    var current = clipped_a;
    var position = screen_a;
    if (at_end) {
        current = clipped_b;
        position = screen_b;
    }

    let resolution = max(uniforms.params.yz, vec2<f32>(1.0, 1.0));
    let screen_delta = (screen_b - screen_a) * resolution;

    var direction = vec2<f32>(1.0, 0.0);
    if (length(screen_delta) > 1e-6) {
        direction = normalize(screen_delta);
    }
    let normal = vec2<f32>(-direction.y, direction.x);
    let offset = normal * side * current.width / resolution;

    // Both ends of the quad take the depth of the end they belong to, so a
    // line running away from the camera is hidden gradually rather than all at
    // once.
    let depth = clamp(current.clip.z / current.clip.w, 0.0, 1.0);
    out.clip_position = vec4<f32>(position + offset, depth, 1.0);
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
