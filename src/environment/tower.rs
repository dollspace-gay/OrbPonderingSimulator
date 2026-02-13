use bevy::prelude::*;

#[derive(Component)]
pub struct TowerPart;

pub fn spawn_tower(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    // === MATERIALS ===
    let stone_floor = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.30, 0.26),
        perceptual_roughness: 0.9,
        metallic: 0.05,
        ..default()
    });
    let stone_wall = materials.add(StandardMaterial {
        base_color: Color::srgb(0.30, 0.26, 0.32),
        perceptual_roughness: 0.9,
        metallic: 0.02,
        ..default()
    });
    let dark_wood = materials.add(StandardMaterial {
        base_color: Color::srgb(0.28, 0.18, 0.10),
        perceptual_roughness: 0.8,
        metallic: 0.0,
        ..default()
    });
    let pedestal_stone = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.30, 0.40),
        perceptual_roughness: 0.6,
        metallic: 0.1,
        ..default()
    });
    let candle_wax = materials.add(StandardMaterial {
        base_color: Color::srgb(0.9, 0.85, 0.7),
        perceptual_roughness: 0.4,
        ..default()
    });
    let candle_flame = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.8, 0.3),
        emissive: LinearRgba::new(8.0, 5.0, 1.0, 1.0),
        ..default()
    });
    let book_red = materials.add(StandardMaterial {
        base_color: Color::srgb(0.55, 0.12, 0.12),
        perceptual_roughness: 0.7,
        ..default()
    });
    let book_blue = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.18, 0.50),
        perceptual_roughness: 0.7,
        ..default()
    });
    let book_green = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.35, 0.14),
        perceptual_roughness: 0.7,
        ..default()
    });
    let rug_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.40, 0.08, 0.15),
        perceptual_roughness: 1.0,
        ..default()
    });
    let window_mat = materials.add(StandardMaterial {
        base_color: Color::srgba(0.3, 0.3, 0.6, 0.5),
        alpha_mode: AlphaMode::Blend,
        emissive: LinearRgba::new(1.0, 1.0, 2.5, 1.0),
        ..default()
    });
    let iron_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.25, 0.25, 0.28),
        perceptual_roughness: 0.5,
        metallic: 0.8,
        ..default()
    });
    let ceiling_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.15, 0.13, 0.18),
        perceptual_roughness: 1.0,
        ..default()
    });

    // === FLOOR ===
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(6.0, 0.2, 6.0))),
        MeshMaterial3d(stone_floor),
        Transform::from_xyz(0.0, -0.1, 0.0),
        TowerPart,
    ));

    // Circular rug under pedestal
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(1.2, 0.02))),
        MeshMaterial3d(rug_mat),
        Transform::from_xyz(0.0, 0.01, 0.0),
        TowerPart,
    ));

    // === ORB TABLE (wide enough for a familiar to walk on) ===
    // Tabletop — flat round surface, top at Y=1.0
    commands.spawn((
        Mesh3d(meshes.add(Cylinder::new(1.1, 0.08))),
        MeshMaterial3d(pedestal_stone.clone()),
        Transform::from_xyz(0.0, 0.96, 0.0),
        TowerPart,
    ));
    // Decorative rim around edge of tabletop
    commands.spawn((
        Mesh3d(meshes.add(Torus::new(1.0, 1.1))),
        MeshMaterial3d(pedestal_stone.clone()),
        Transform::from_xyz(0.0, 1.0, 0.0),
        TowerPart,
    ));
    // Four sturdy legs
    let leg = meshes.add(Cuboid::new(0.12, 0.92, 0.12));
    for (x, z) in [(-0.7, -0.7), (0.7, -0.7), (-0.7, 0.7), (0.7, 0.7)] {
        commands.spawn((
            Mesh3d(leg.clone()),
            MeshMaterial3d(dark_wood.clone()),
            Transform::from_xyz(x, 0.46, z),
            TowerPart,
        ));
    }
    // Cross-braces between legs for sturdiness
    let brace = meshes.add(Cuboid::new(1.28, 0.06, 0.06));
    commands.spawn((
        Mesh3d(brace.clone()),
        MeshMaterial3d(dark_wood.clone()),
        Transform::from_xyz(0.0, 0.3, -0.7),
        TowerPart,
    ));
    commands.spawn((
        Mesh3d(brace.clone()),
        MeshMaterial3d(dark_wood.clone()),
        Transform::from_xyz(0.0, 0.3, 0.7),
        TowerPart,
    ));
    let brace_side = meshes.add(Cuboid::new(0.06, 0.06, 1.28));
    commands.spawn((
        Mesh3d(brace_side.clone()),
        MeshMaterial3d(dark_wood.clone()),
        Transform::from_xyz(-0.7, 0.3, 0.0),
        TowerPart,
    ));
    commands.spawn((
        Mesh3d(brace_side),
        MeshMaterial3d(dark_wood.clone()),
        Transform::from_xyz(0.7, 0.3, 0.0),
        TowerPart,
    ));

    // === WALLS ===
    // Back wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(6.0, 4.0, 0.3))),
        MeshMaterial3d(stone_wall.clone()),
        Transform::from_xyz(0.0, 2.0, -3.0),
        TowerPart,
    ));
    // Left wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 4.0, 6.0))),
        MeshMaterial3d(stone_wall.clone()),
        Transform::from_xyz(-3.0, 2.0, 0.0),
        TowerPart,
    ));
    // Right wall
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.3, 4.0, 6.0))),
        MeshMaterial3d(stone_wall.clone()),
        Transform::from_xyz(3.0, 2.0, 0.0),
        TowerPart,
    ));
    // Ceiling
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(6.0, 0.2, 6.0))),
        MeshMaterial3d(ceiling_mat),
        Transform::from_xyz(0.0, 4.0, 0.0),
        TowerPart,
    ));

    // === WINDOW (back wall, emissive moonlight glow) ===
    // Glowing window pane
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 1.8, 0.05))),
        MeshMaterial3d(window_mat),
        Transform::from_xyz(0.0, 2.6, -2.83),
        TowerPart,
    ));
    // Window sill (stone ledge)
    let sill_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.30, 0.35),
        perceptual_roughness: 0.8,
        ..default()
    });
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.4, 0.08, 0.2))),
        MeshMaterial3d(sill_mat),
        Transform::from_xyz(0.0, 1.68, -2.75),
        TowerPart,
    ));
    // Frame (iron border around window)
    let frame_v = meshes.add(Cuboid::new(0.08, 1.9, 0.1));
    let frame_h = meshes.add(Cuboid::new(1.36, 0.08, 0.1));
    // Left frame
    commands.spawn((
        Mesh3d(frame_v.clone()),
        MeshMaterial3d(iron_mat.clone()),
        Transform::from_xyz(-0.64, 2.6, -2.81),
        TowerPart,
    ));
    // Right frame
    commands.spawn((
        Mesh3d(frame_v),
        MeshMaterial3d(iron_mat.clone()),
        Transform::from_xyz(0.64, 2.6, -2.81),
        TowerPart,
    ));
    // Top frame
    commands.spawn((
        Mesh3d(frame_h.clone()),
        MeshMaterial3d(iron_mat.clone()),
        Transform::from_xyz(0.0, 3.52, -2.81),
        TowerPart,
    ));
    // Bottom frame
    commands.spawn((
        Mesh3d(frame_h),
        MeshMaterial3d(iron_mat.clone()),
        Transform::from_xyz(0.0, 1.7, -2.81),
        TowerPart,
    ));
    // Cross divider
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.04, 1.8, 0.08))),
        MeshMaterial3d(iron_mat.clone()),
        Transform::from_xyz(0.0, 2.6, -2.81),
        TowerPart,
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(1.2, 0.04, 0.08))),
        MeshMaterial3d(iron_mat),
        Transform::from_xyz(0.0, 2.6, -2.81),
        TowerPart,
    ));
    // Moonlight streaming through window
    commands.spawn((
        PointLight {
            color: Color::srgb(0.5, 0.5, 0.8),
            intensity: 5000.0,
            range: 10.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(0.0, 2.6, -2.2),
    ));

    // === BOOKSHELF (left wall) ===
    // Shelf frame
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.4, 2.0, 1.2))),
        MeshMaterial3d(dark_wood.clone()),
        Transform::from_xyz(-2.7, 1.0, -1.5),
        TowerPart,
    ));
    // Books on shelves
    let book_materials = [
        book_red,
        book_blue.clone(),
        book_green.clone(),
        book_blue,
        book_green,
    ];
    for (i, bmat) in book_materials.iter().enumerate() {
        let x_off = -0.05 + (i as f32) * 0.18;
        let height = 0.25 + (i % 3) as f32 * 0.03;
        // Bottom shelf books
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, height, 0.18))),
            MeshMaterial3d(bmat.clone()),
            Transform::from_xyz(-2.65, 0.35 + height * 0.5, -1.8 + x_off),
            TowerPart,
        ));
        // Top shelf books
        commands.spawn((
            Mesh3d(meshes.add(Cuboid::new(0.12, height, 0.18))),
            MeshMaterial3d(bmat.clone()),
            Transform::from_xyz(-2.65, 1.35 + height * 0.5, -1.8 + x_off),
            TowerPart,
        ));
    }

    // === WALL SCONCE CANDLES (mounted at eye height) ===
    let sconce_positions = [
        // Back wall, flanking window
        (Vec3::new(-1.5, 2.0, -2.75), Vec3::new(0.0, 0.0, 0.08)),
        (Vec3::new(1.5, 2.0, -2.75), Vec3::new(0.0, 0.0, 0.08)),
        // Left wall
        (Vec3::new(-2.75, 2.0, -0.5), Vec3::new(0.08, 0.0, 0.0)),
        (Vec3::new(-2.75, 2.0, 1.5), Vec3::new(0.08, 0.0, 0.0)),
        // Right wall
        (Vec3::new(2.75, 2.0, 0.0), Vec3::new(-0.08, 0.0, 0.0)),
    ];
    let sconce_plate = meshes.add(Cuboid::new(0.15, 0.03, 0.12));
    let sconce_arm = meshes.add(Cuboid::new(0.03, 0.15, 0.03));
    let candle_body = meshes.add(Cylinder::new(0.04, 0.20));
    let flame_mesh = meshes.add(Sphere::new(0.05));
    let sconce_mat = materials.add(StandardMaterial {
        base_color: Color::srgb(0.35, 0.30, 0.20),
        metallic: 0.7,
        perceptual_roughness: 0.4,
        ..default()
    });

    for (pos, offset) in &sconce_positions {
        // Wall bracket arm
        commands.spawn((
            Mesh3d(sconce_arm.clone()),
            MeshMaterial3d(sconce_mat.clone()),
            Transform::from_xyz(pos.x + offset.x * 0.5, pos.y - 0.1, pos.z + offset.z * 0.5),
            TowerPart,
        ));
        // Sconce plate
        commands.spawn((
            Mesh3d(sconce_plate.clone()),
            MeshMaterial3d(sconce_mat.clone()),
            Transform::from_xyz(pos.x + offset.x, pos.y, pos.z + offset.z),
            TowerPart,
        ));
        // Candle
        commands.spawn((
            Mesh3d(candle_body.clone()),
            MeshMaterial3d(candle_wax.clone()),
            Transform::from_xyz(pos.x + offset.x, pos.y + 0.12, pos.z + offset.z),
            TowerPart,
        ));
        // Flame
        commands.spawn((
            Mesh3d(flame_mesh.clone()),
            MeshMaterial3d(candle_flame.clone()),
            Transform::from_xyz(pos.x + offset.x, pos.y + 0.26, pos.z + offset.z)
                .with_scale(Vec3::new(1.0, 1.5, 1.0)),
            TowerPart,
        ));
        // Warm candlelight
        commands.spawn((
            PointLight {
                color: Color::srgb(1.0, 0.8, 0.4),
                intensity: 3000.0,
                range: 8.0,
                shadows_enabled: false,
                ..default()
            },
            Transform::from_xyz(pos.x + offset.x, pos.y + 0.4, pos.z + offset.z),
        ));
    }

    // Candle on the table
    commands.spawn((
        Mesh3d(candle_body.clone()),
        MeshMaterial3d(candle_wax.clone()),
        Transform::from_xyz(2.2, 0.83, -1.0),
        TowerPart,
    ));
    commands.spawn((
        Mesh3d(flame_mesh.clone()),
        MeshMaterial3d(candle_flame.clone()),
        Transform::from_xyz(2.2, 0.97, -1.0).with_scale(Vec3::new(1.0, 1.5, 1.0)),
        TowerPart,
    ));
    commands.spawn((
        PointLight {
            color: Color::srgb(1.0, 0.8, 0.4),
            intensity: 2000.0,
            range: 6.0,
            shadows_enabled: false,
            ..default()
        },
        Transform::from_xyz(2.2, 1.2, -1.0),
    ));

    // === SMALL TABLE (right side) ===
    // Table top
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(0.8, 0.05, 0.5))),
        MeshMaterial3d(dark_wood.clone()),
        Transform::from_xyz(2.2, 0.7, -1.0),
        TowerPart,
    ));
    // Table legs
    let leg_mesh = meshes.add(Cuboid::new(0.06, 0.7, 0.06));
    for (dx, dz) in [(-0.33, -0.18), (0.33, -0.18), (-0.33, 0.18), (0.33, 0.18)] {
        commands.spawn((
            Mesh3d(leg_mesh.clone()),
            MeshMaterial3d(dark_wood.clone()),
            Transform::from_xyz(2.2 + dx, 0.35, -1.0 + dz),
            TowerPart,
        ));
    }

    // === STONE PILLARS (decorative corners) ===
    let pillar_mesh = meshes.add(Cylinder::new(0.15, 3.5));
    for (x, z) in [(-2.5, -2.5), (2.5, -2.5)] {
        commands.spawn((
            Mesh3d(pillar_mesh.clone()),
            MeshMaterial3d(stone_wall.clone()),
            Transform::from_xyz(x, 1.75, z),
            TowerPart,
        ));
    }
}
