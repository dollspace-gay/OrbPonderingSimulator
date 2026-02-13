#import bevy_pbr::{
    mesh_view_bindings::globals,
    forward_io::VertexOutput,
}

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> sky_seed: f32;

fn hash(p: vec2<f32>) -> f32 {
    var p3 = fract(vec3<f32>(p.x, p.y, p.x) * 0.1031);
    p3 += dot(p3, p3.yzx + 33.33);
    return fract((p3.x + p3.y) * p3.z);
}

fn star_layer(uv: vec2<f32>, scale: f32, brightness: f32, seed: f32) -> vec3<f32> {
    let grid = uv * scale;
    let cell = floor(grid);
    let local = fract(grid) - 0.5;

    var col = vec3<f32>(0.0);
    for (var y = -1; y <= 1; y++) {
        for (var x = -1; x <= 1; x++) {
            let neighbor = vec2<f32>(f32(x), f32(y));
            let cell_id = cell + neighbor;
            let rand_val = hash(cell_id + seed);

            if rand_val > 0.92 {
                let star_pos = vec2<f32>(hash(cell_id * 1.7 + seed), hash(cell_id * 2.3 + seed + 17.0)) - 0.5;
                let d = length(local - neighbor - star_pos);
                let star_size = 0.015 + hash(cell_id * 3.1 + seed) * 0.025;
                let intensity = smoothstep(star_size, 0.0, d) * brightness;

                let temp = hash(cell_id * 5.7 + seed);
                var star_color = vec3<f32>(1.0, 1.0, 1.0);
                if temp < 0.3 {
                    star_color = vec3<f32>(0.7, 0.8, 1.0);
                } else if temp < 0.5 {
                    star_color = vec3<f32>(1.0, 0.9, 0.7);
                } else if temp < 0.6 {
                    star_color = vec3<f32>(0.9, 0.7, 1.0);
                }

                let twinkle = 0.7 + 0.3 * sin(globals.time * (2.0 + hash(cell_id) * 4.0) + hash(cell_id) * 6.28);
                col += star_color * intensity * twinkle;
            }
        }
    }
    return col;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    let world_pos = normalize(in.world_position.xyz);

    // Spherical UV from world position
    let theta = atan2(world_pos.z, world_pos.x);
    let phi = asin(clamp(world_pos.y, -1.0, 1.0));
    let uv = vec2<f32>(theta / 6.2832 + 0.5, phi / 3.1416 + 0.5);

    // Deep space gradient
    let horizon = smoothstep(-0.1, 0.5, world_pos.y);
    let bg = mix(
        vec3<f32>(0.02, 0.01, 0.04),
        vec3<f32>(0.005, 0.005, 0.02),
        horizon
    );

    // Subtle nebula wisps
    let n1 = sin(uv.x * 12.0 + uv.y * 8.0 + globals.time * 0.02) * 0.5 + 0.5;
    let n2 = sin(uv.x * 7.0 - uv.y * 13.0 + globals.time * 0.015) * 0.5 + 0.5;
    let nebula = n1 * n2 * 0.03;
    let nebula_color = vec3<f32>(0.1, 0.02, 0.15) * nebula;

    // Multiple star layers at different scales
    var stars = star_layer(uv, 40.0, 1.2, sky_seed);
    stars += star_layer(uv, 80.0, 0.8, sky_seed + 100.0);
    stars += star_layer(uv, 160.0, 0.4, sky_seed + 200.0);

    let final_color = bg + nebula_color + stars;
    return vec4<f32>(final_color, 1.0);
}
