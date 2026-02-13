#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

struct OrbParams {
    pondering_power: f32,
    color_phase: f32,
    glow_intensity: f32,
    orb_type_index: u32,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> params: OrbParams;

// ========== NOISE ==========

fn hash31(p: vec3<f32>) -> f32 {
    var p3 = fract(p * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn noise3d(p: vec3<f32>) -> f32 {
    let i = floor(p);
    let f = fract(p);
    let u = f * f * (3.0 - 2.0 * f);

    return mix(
        mix(
            mix(hash31(i + vec3(0.0, 0.0, 0.0)), hash31(i + vec3(1.0, 0.0, 0.0)), u.x),
            mix(hash31(i + vec3(0.0, 1.0, 0.0)), hash31(i + vec3(1.0, 1.0, 0.0)), u.x),
            u.y
        ),
        mix(
            mix(hash31(i + vec3(0.0, 0.0, 1.0)), hash31(i + vec3(1.0, 0.0, 1.0)), u.x),
            mix(hash31(i + vec3(0.0, 1.0, 1.0)), hash31(i + vec3(1.0, 1.0, 1.0)), u.x),
            u.y
        ),
        u.z
    );
}

fn fbm(p: vec3<f32>, octaves: i32) -> f32 {
    var value = 0.0;
    var amplitude = 0.5;
    var frequency = 1.0;
    for (var i = 0; i < octaves; i++) {
        value += amplitude * noise3d(p * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    return value;
}

// ========== COLOR ==========

fn oklab_to_linear(c: vec3<f32>) -> vec3<f32> {
    let l_ = c.x + 0.3963377774 * c.y + 0.2158037573 * c.z;
    let m_ = c.x - 0.1055613458 * c.y - 0.0638541728 * c.z;
    let s_ = c.x - 0.0894841775 * c.y - 1.2914855480 * c.z;
    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;
    return vec3<f32>(
        4.0767416621 * l - 3.3077115913 * m + 0.2309699292 * s,
        -1.2684380046 * l + 2.6097574011 * m - 0.3413193965 * s,
        -0.0041960863 * l - 0.7034186147 * m + 1.7076147010 * s,
    );
}

fn crystal_palette(t: f32) -> vec3<f32> {
    let deep_blue = vec3<f32>(0.55, 0.15, -0.20);
    let cyan = vec3<f32>(0.75, -0.08, -0.12);
    let white_glow = vec3<f32>(0.92, 0.0, 0.0);
    let c = mix(deep_blue, mix(cyan, white_glow, t), t);
    return max(oklab_to_linear(c), vec3<f32>(0.0));
}

fn obsidian_palette(t: f32) -> vec3<f32> {
    let dark = vec3<f32>(0.25, 0.06, 0.02);
    let ember = vec3<f32>(0.55, 0.15, 0.01);
    let orange = vec3<f32>(0.80, 0.20, 0.05);
    return mix(dark, mix(ember, orange, t), t);
}

fn mercury_palette(t: f32) -> vec3<f32> {
    let silver = vec3<f32>(0.75, 0.75, 0.80);
    let bright = vec3<f32>(0.95, 0.95, 1.0);
    let dim = vec3<f32>(0.3, 0.3, 0.35);
    return mix(dim, mix(silver, bright, t), t);
}

fn galaxy_palette(t: f32) -> vec3<f32> {
    let deep_purple = vec3<f32>(0.15, 0.05, 0.25);
    let nebula_pink = vec3<f32>(0.5, 0.15, 0.4);
    let star_white = vec3<f32>(0.95, 0.9, 1.0);
    return mix(deep_purple, mix(nebula_pink, star_white, t * t), t);
}

fn get_palette(t: f32, orb_type: u32) -> vec3<f32> {
    if orb_type == 1u {
        return obsidian_palette(t);
    } else if orb_type == 2u {
        return mercury_palette(t);
    } else if orb_type == 3u {
        return galaxy_palette(t);
    }
    return crystal_palette(t);
}

// ========== FRAGMENT ==========

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let t = globals.time;
    let world_normal = normalize(in.world_normal);
    let world_pos = in.world_position.xyz;

    // Proper view direction from camera
    let view_dir = normalize(vec3<f32>(0.0, 1.8, 3.0) - world_pos);

    // Multi-layered Fresnel
    let ndotv = max(dot(world_normal, view_dir), 0.0);
    let fresnel_soft = pow(1.0 - ndotv, 2.0);
    let fresnel_sharp = pow(1.0 - ndotv, 5.0);
    let rim_glow = (fresnel_soft * 0.6 + fresnel_sharp * 0.4) * params.glow_intensity * (1.0 + params.pondering_power * 3.0);

    // Domain-warped nebula swirl (double warp for extra depth)
    let warp_speed = 0.3 + params.pondering_power * 0.7;
    let warp1 = vec3<f32>(
        fbm(world_pos * 2.0 + vec3<f32>(t * warp_speed, 0.0, 0.0), 3),
        fbm(world_pos * 2.0 + vec3<f32>(0.0, t * warp_speed * 0.7, 0.0), 3),
        fbm(world_pos * 2.0 + vec3<f32>(0.0, 0.0, t * warp_speed * 0.5), 3),
    );
    let warp2 = vec3<f32>(
        fbm(world_pos * 1.5 + warp1 * 1.5 + vec3<f32>(t * 0.1, 0.0, 0.0), 2),
        fbm(world_pos * 1.5 + warp1 * 1.5 + vec3<f32>(0.0, t * 0.08, 0.0), 2),
        fbm(world_pos * 1.5 + warp1 * 1.5 + vec3<f32>(0.0, 0.0, t * 0.06), 2),
    );
    let nebula = fbm(world_pos * 3.0 + warp1 * 1.5 + warp2 * 0.8, 5);

    // Secondary detail layer
    let detail = fbm(world_pos * 8.0 + warp1 * 0.5 + t * 0.05, 3);

    // Color from palette with detail modulation
    let phase = fract(params.color_phase + nebula * 0.5 + detail * 0.1);
    let color = get_palette(phase, params.orb_type_index);

    // Deep core glow (brighter at center)
    let core_factor = pow(ndotv, 2.0);
    let core_color = color * 1.8 * core_factor;

    // Interior with depth illusion
    let interior = color * (0.15 + nebula * 0.5 + detail * 0.2);

    // Rim glow in complementary tint
    let rim_tint = mix(color * 1.5, vec3<f32>(0.6, 0.7, 1.0), 0.3);
    let rim = rim_tint * rim_glow;

    var final_color = interior + core_color * 0.3 + rim;

    // Pondering pulse (multi-frequency)
    let pulse1 = sin(t * 4.0) * 0.5 + 0.5;
    let pulse2 = sin(t * 1.7 + 0.5) * 0.5 + 0.5;
    let pulse = 1.0 + params.pondering_power * 0.2 * (pulse1 * 0.7 + pulse2 * 0.3);
    final_color = final_color * pulse;

    // Bright sparkle points
    let sparkle_noise = noise3d(world_pos * 20.0 + t * 2.0);
    let sparkle = smoothstep(0.85, 0.95, sparkle_noise) * params.pondering_power * 0.8;
    final_color += vec3<f32>(1.0, 0.95, 0.8) * sparkle;

    // Outer glow haze (soft bloom approximation)
    let haze = fresnel_soft * fresnel_soft * 0.15 * params.glow_intensity;
    final_color += color * haze;

    // Alpha: opaque core, translucent rim with glow bleed
    let alpha = 0.85 + 0.15 * (1.0 - fresnel_soft);

    return vec4<f32>(final_color, alpha);
}
