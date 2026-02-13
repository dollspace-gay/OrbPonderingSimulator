#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct StandParams {
    time: f32,
    _pad0: f32,
    _pad1: f32,
    _pad2: f32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> params: StandParams;

const PI: f32 = 3.14159265;
const TAU: f32 = 6.28318530;

// ========== NOISE ==========

fn hash21(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

// ========== SDF HELPERS ==========

fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

fn glow_line(d: f32, width: f32) -> f32 {
    return smoothstep(width, 0.0, d);
}

// ========== RUNE SHAPES ==========
// Each rune drawn in local space roughly -0.5 to 0.5

// Tiwaz — arrow up
fn rune_tiwaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, -0.4), vec2<f32>(0.0, 0.4));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.4), vec2<f32>(-0.2, 0.05)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.4), vec2<f32>(0.2, 0.05)));
    return glow_line(d, 0.07);
}

// Algiz — forked branch
fn rune_algiz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, -0.4), vec2<f32>(0.0, 0.15));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.15), vec2<f32>(-0.22, 0.4)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.15), vec2<f32>(0.22, 0.4)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.05), vec2<f32>(-0.16, 0.2)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.05), vec2<f32>(0.16, 0.2)));
    return glow_line(d, 0.07);
}

// Dagaz — hourglass
fn rune_dagaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(-0.18, -0.35), vec2<f32>(0.18, 0.0));
    d = min(d, sd_segment(p, vec2<f32>(0.18, 0.0), vec2<f32>(-0.18, 0.35)));
    d = min(d, sd_segment(p, vec2<f32>(-0.18, -0.35), vec2<f32>(-0.18, 0.35)));
    d = min(d, sd_segment(p, vec2<f32>(0.18, -0.35), vec2<f32>(-0.18, 0.0)));
    d = min(d, sd_segment(p, vec2<f32>(-0.18, 0.0), vec2<f32>(0.18, 0.35)));
    d = min(d, sd_segment(p, vec2<f32>(0.18, -0.35), vec2<f32>(0.18, 0.35)));
    return glow_line(d, 0.07);
}

// Kenaz — chevron
fn rune_kenaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.12, 0.35), vec2<f32>(-0.12, 0.0));
    d = min(d, sd_segment(p, vec2<f32>(-0.12, 0.0), vec2<f32>(0.12, -0.35)));
    return glow_line(d, 0.07);
}

// Ingwaz — diamond
fn rune_ingwaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, 0.35), vec2<f32>(0.22, 0.0));
    d = min(d, sd_segment(p, vec2<f32>(0.22, 0.0), vec2<f32>(0.0, -0.35)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.35), vec2<f32>(-0.22, 0.0)));
    d = min(d, sd_segment(p, vec2<f32>(-0.22, 0.0), vec2<f32>(0.0, 0.35)));
    return glow_line(d, 0.07);
}

// Sowilo — lightning zigzag
fn rune_sowilo(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(-0.12, 0.35), vec2<f32>(0.12, 0.08));
    d = min(d, sd_segment(p, vec2<f32>(0.12, 0.08), vec2<f32>(-0.12, -0.08)));
    d = min(d, sd_segment(p, vec2<f32>(-0.12, -0.08), vec2<f32>(0.12, -0.35)));
    return glow_line(d, 0.07);
}

// Othala — inheritance (diamond with legs)
fn rune_othala(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, 0.35), vec2<f32>(0.2, 0.05));
    d = min(d, sd_segment(p, vec2<f32>(0.2, 0.05), vec2<f32>(0.0, -0.15)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.15), vec2<f32>(-0.2, 0.05)));
    d = min(d, sd_segment(p, vec2<f32>(-0.2, 0.05), vec2<f32>(0.0, 0.35)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.15), vec2<f32>(-0.15, -0.4)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.15), vec2<f32>(0.15, -0.4)));
    return glow_line(d, 0.07);
}

// Ansuz — F-like with angled branches
fn rune_ansuz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, -0.4), vec2<f32>(0.0, 0.4));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.25), vec2<f32>(0.2, 0.05)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.05), vec2<f32>(0.2, -0.15)));
    return glow_line(d, 0.07);
}

fn eval_rune(p: vec2<f32>, idx: i32) -> f32 {
    if idx == 0 { return rune_tiwaz(p); }
    if idx == 1 { return rune_algiz(p); }
    if idx == 2 { return rune_dagaz(p); }
    if idx == 3 { return rune_kenaz(p); }
    if idx == 4 { return rune_ingwaz(p); }
    if idx == 5 { return rune_sowilo(p); }
    if idx == 6 { return rune_othala(p); }
    return rune_ansuz(p);
}

// ========== FRAGMENT ==========

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = params.time;
    let normal = normalize(in.world_normal);
    let uv = in.uv;

    // Detect side face vs cap face based on normal direction
    let is_side = abs(normal.y) < 0.3;

    // === BASE METALLIC COLOR ===
    // Dark bronze/iron tone
    let base_dark = vec3<f32>(0.04, 0.03, 0.025);
    let base_mid = vec3<f32>(0.08, 0.06, 0.05);

    // Simple lighting
    let light_dir = normalize(vec3<f32>(0.3, 1.0, 0.5));
    let ndl = max(dot(normal, light_dir), 0.0);
    let ambient = 0.12;

    // View-dependent specular for metallic sheen
    let view_dir = normalize(vec3<f32>(0.0, 1.6, 3.0) - in.world_position.xyz);
    let half_vec = normalize(light_dir + view_dir);
    let spec = pow(max(dot(normal, half_vec), 0.0), 32.0) * 0.3;

    var base_color = base_mid * (ambient + ndl * 0.5) + vec3<f32>(0.12, 0.09, 0.06) * spec;

    // Subtle surface variation (micro-texture)
    let noise_val = hash21(uv * 200.0);
    base_color *= 0.9 + noise_val * 0.2;

    // === RUNE ENGRAVINGS (side faces only) ===
    var rune_glow = 0.0;
    var rune_soft = 0.0;

    if is_side {
        let rune_count = 8;
        let segment_width = 1.0 / f32(rune_count);

        for (var i = 0; i < rune_count; i++) {
            let segment_center = (f32(i) + 0.5) * segment_width;
            let du = uv.x - segment_center;

            // Only evaluate if close enough to this segment
            if abs(du) < segment_width * 0.6 {
                // Map to local rune space (-0.5 to 0.5)
                let local_u = du / segment_width;
                let local_v = uv.y * 2.0 - 1.0;
                let local_p = vec2<f32>(local_u, local_v) * 1.2;

                let r = eval_rune(local_p, i % 8);
                rune_glow += r;
            }
        }

        rune_glow = min(rune_glow, 1.0);

        // Softer, wider glow halo around runes (re-evaluate with wider threshold)
        for (var i = 0; i < rune_count; i++) {
            let segment_center = (f32(i) + 0.5) * segment_width;
            let du = uv.x - segment_center;
            if abs(du) < segment_width * 0.6 {
                let local_u = du / segment_width;
                let local_v = uv.y * 2.0 - 1.0;
                let local_p = vec2<f32>(local_u, local_v) * 1.2;
                let r = eval_rune(local_p, i % 8);
                rune_soft += r * 0.3;
            }
        }
        rune_soft = min(rune_soft, 1.0);

        // Engraved groove lines at top and bottom edges
        let edge_top = smoothstep(0.02, 0.0, abs(uv.y - 0.9));
        let edge_bot = smoothstep(0.02, 0.0, abs(uv.y - 0.1));
        rune_glow += (edge_top + edge_bot) * 0.4;

        // Small separator dots between rune segments
        for (var i = 0; i < rune_count; i++) {
            let sep_x = f32(i) * segment_width;
            let dot_d = length(vec2<f32>(uv.x - sep_x, uv.y - 0.5));
            rune_glow += smoothstep(0.015, 0.0, dot_d) * 0.5;
        }
    }

    // === CAP DETAIL (top/bottom faces) ===
    var cap_detail = 0.0;
    if !is_side {
        // Concentric ring pattern on caps
        let cap_dist = length(uv - 0.5) * 2.0;
        let ring1 = smoothstep(0.02, 0.0, abs(cap_dist - 0.7));
        let ring2 = smoothstep(0.015, 0.0, abs(cap_dist - 0.4));
        cap_detail = (ring1 + ring2) * 0.3;
    }

    // === GLOW ANIMATION ===
    let pulse = 0.6 + 0.4 * sin(t * 1.5);
    let glow_color = vec3<f32>(0.9, 0.55, 0.15); // warm amber/gold

    // Rune glow with slight color variation per position
    let glow_tint = mix(
        vec3<f32>(0.9, 0.5, 0.1),
        vec3<f32>(1.0, 0.7, 0.3),
        sin(uv.x * TAU * 2.0 + t) * 0.5 + 0.5
    );

    // Engraved depth: darken the base where runes are carved
    base_color *= 1.0 - rune_glow * 0.3;

    // Add the emissive glow from the engravings
    let emissive = glow_tint * rune_glow * pulse * 1.5
                 + glow_color * rune_soft * pulse * 0.3
                 + glow_color * cap_detail * pulse * 0.8;

    let final_color = base_color + emissive;

    return vec4<f32>(final_color, 1.0);
}
