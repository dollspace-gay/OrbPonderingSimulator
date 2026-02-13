#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct CircleParams {
    time: f32,
    intensity: f32,
    _pad0: f32,
    _pad1: f32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> params: CircleParams;

// ========== HELPERS ==========

const PI: f32 = 3.14159265;
const TAU: f32 = 6.28318530;

// Soft ring at given radius with given width
fn ring(dist: f32, radius: f32, width: f32) -> f32 {
    return smoothstep(width, 0.0, abs(dist - radius));
}

// Distance from point p to line segment a->b
fn sd_segment(p: vec2<f32>, a: vec2<f32>, b: vec2<f32>) -> f32 {
    let pa = p - a;
    let ba = b - a;
    let h = clamp(dot(pa, ba) / dot(ba, ba), 0.0, 1.0);
    return length(pa - ba * h);
}

// Convert segment SDF to a soft glow line
fn glow_line(d: f32, width: f32) -> f32 {
    return smoothstep(width, 0.0, d);
}

// ========== RUNE SHAPES ==========
// Each rune is drawn in local space roughly -0.5 to 0.5

// Tiwaz ↑ — arrow pointing up
fn rune_tiwaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, -0.45), vec2<f32>(0.0, 0.45));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.45), vec2<f32>(-0.22, 0.1)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.45), vec2<f32>(0.22, 0.1)));
    return glow_line(d, 0.06);
}

// Algiz ψ — forked branch
fn rune_algiz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, -0.45), vec2<f32>(0.0, 0.2));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.2), vec2<f32>(-0.25, 0.45)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.2), vec2<f32>(0.25, 0.45)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.0), vec2<f32>(-0.18, 0.25)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, 0.0), vec2<f32>(0.18, 0.25)));
    return glow_line(d, 0.06);
}

// Dagaz ⋈ — hourglass/butterfly
fn rune_dagaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(-0.2, -0.4), vec2<f32>(0.2, 0.0));
    d = min(d, sd_segment(p, vec2<f32>(0.2, 0.0), vec2<f32>(-0.2, 0.4)));
    d = min(d, sd_segment(p, vec2<f32>(-0.2, -0.4), vec2<f32>(-0.2, 0.4)));
    d = min(d, sd_segment(p, vec2<f32>(0.2, -0.4), vec2<f32>(-0.2, 0.0)));
    d = min(d, sd_segment(p, vec2<f32>(-0.2, 0.0), vec2<f32>(0.2, 0.4)));
    d = min(d, sd_segment(p, vec2<f32>(0.2, -0.4), vec2<f32>(0.2, 0.4)));
    return glow_line(d, 0.06);
}

// Kenaz < — angled chevron
fn rune_kenaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.15, 0.4), vec2<f32>(-0.15, 0.0));
    d = min(d, sd_segment(p, vec2<f32>(-0.15, 0.0), vec2<f32>(0.15, -0.4)));
    return glow_line(d, 0.06);
}

// Ingwaz ◇ — diamond
fn rune_ingwaz(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(0.0, 0.4), vec2<f32>(0.25, 0.0));
    d = min(d, sd_segment(p, vec2<f32>(0.25, 0.0), vec2<f32>(0.0, -0.4)));
    d = min(d, sd_segment(p, vec2<f32>(0.0, -0.4), vec2<f32>(-0.25, 0.0)));
    d = min(d, sd_segment(p, vec2<f32>(-0.25, 0.0), vec2<f32>(0.0, 0.4)));
    return glow_line(d, 0.06);
}

// Sowilo ⚡ — lightning zigzag
fn rune_sowilo(p: vec2<f32>) -> f32 {
    var d = sd_segment(p, vec2<f32>(-0.15, 0.4), vec2<f32>(0.15, 0.1));
    d = min(d, sd_segment(p, vec2<f32>(0.15, 0.1), vec2<f32>(-0.15, -0.1)));
    d = min(d, sd_segment(p, vec2<f32>(-0.15, -0.1), vec2<f32>(0.15, -0.4)));
    return glow_line(d, 0.06);
}

// Evaluate a rune by index (0-5)
fn eval_rune(p: vec2<f32>, idx: i32) -> f32 {
    if idx == 0 { return rune_tiwaz(p); }
    if idx == 1 { return rune_algiz(p); }
    if idx == 2 { return rune_dagaz(p); }
    if idx == 3 { return rune_kenaz(p); }
    if idx == 4 { return rune_ingwaz(p); }
    return rune_sowilo(p);
}

// ========== FRAGMENT ==========

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = params.time;

    // UV to centered coordinates (-1 to 1)
    let uv = in.uv * 2.0 - 1.0;
    let dist = length(uv);
    let angle = atan2(uv.y, uv.x);

    // Rotating angles
    let rot = angle + t * 0.4;
    let rot_slow = angle + t * 0.15;
    let rot_rev = angle - t * 0.25;

    // ===== RINGS =====
    let outer = ring(dist, 0.88, 0.025) * 1.2;
    let mid = ring(dist, 0.68, 0.018);
    let inner = ring(dist, 0.42, 0.015) * 0.8;
    let core_ring = ring(dist, 0.22, 0.008) * 0.5;

    // ===== OUTER RUNE RING — 12 runes between outer and mid ring =====
    let outer_num = 12;
    let outer_rune_radius = 0.78;
    let outer_band = smoothstep(0.68, 0.71, dist) * smoothstep(0.88, 0.85, dist);
    var outer_runes = 0.0;

    for (var j = 0; j < outer_num; j++) {
        let oa = f32(j) / f32(outer_num) * TAU + t * 0.15;
        let oc = vec2<f32>(cos(oa), sin(oa)) * outer_rune_radius;
        let ol = (uv - oc) * 10.0; // smaller runes, tighter scale
        if length(ol) < 3.5 {
            let r = eval_rune(ol, j % 6);
            outer_runes += r * outer_band;
        }
    }
    outer_runes = min(outer_runes, 1.0) * 0.6;

    // ===== INNER RUNIC SYMBOLS — 6 runes between mid and inner ring =====
    let inner_num = 6;
    let inner_rune_radius = 0.55;
    let inner_band = smoothstep(0.42, 0.45, dist) * smoothstep(0.68, 0.65, dist);
    var inner_runes = 0.0;

    for (var i = 0; i < inner_num; i++) {
        let ia = f32(i) / f32(inner_num) * TAU + t * 0.2;
        let ic = vec2<f32>(cos(ia), sin(ia)) * inner_rune_radius;
        let il = (uv - ic) * 6.0;
        if length(il) < 3.5 {
            let r = eval_rune(il, i);
            inner_runes += r * inner_band;
        }
    }
    inner_runes = min(inner_runes, 1.0) * 0.8;

    // ===== RADIAL LINES connecting rings =====
    let radial_count = 8.0;
    let radial_line = smoothstep(0.015, 0.0, abs(sin(rot_slow * radial_count * 0.5)));
    let radial_mask = smoothstep(0.20, 0.25, dist) * smoothstep(0.90, 0.85, dist);
    let radials = radial_line * radial_mask * 0.25;

    // ===== DOT ACCENTS on mid ring =====
    let dot_count = 24.0;
    let dot_angle = fract(rot / TAU * dot_count);
    let dot = smoothstep(0.08, 0.0, abs(dot_angle - 0.5)) * ring(dist, 0.68, 0.03) * 0.4;

    // ===== CENTRAL GLOW =====
    let center_glow = exp(-dist * 4.0) * 0.25;

    // ===== COMBINE =====
    var pattern = outer + mid + inner + core_ring
                + outer_runes + inner_runes + radials + dot + center_glow;

    // Pulsing
    let pulse = 0.7 + 0.3 * sin(t * 2.5);
    pattern = pattern * pulse * params.intensity;

    // Warm golden color
    let col = vec3<f32>(1.0, 0.75, 0.3) * pattern;

    // Color variation: outer runes slightly orange, inner runes slightly whiter
    let col_final = col
        + vec3<f32>(0.3, 0.1, 0.0) * outer_runes * pulse * params.intensity
        + vec3<f32>(0.3, 0.4, 0.5) * inner_runes * pulse * params.intensity;

    // Fade at edge
    let edge_fade = 1.0 - smoothstep(0.88, 1.0, dist);

    let alpha = clamp(pattern * edge_fade, 0.0, 1.0);
    if alpha < 0.01 {
        discard;
    }

    return vec4<f32>(col_final * edge_fade, alpha);
}
