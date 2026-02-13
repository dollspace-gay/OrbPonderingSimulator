# Orb Pondering Simulator - Claude Code Guide

## Project Overview
A meditative idle/clicker game built with **Bevy 0.18** (Rust). The player ponders a mystical orb to generate wisdom, discover deep truths, and progress through arcane systems.

## Build & Run
```bash
cargo run          # Run the game (dev profile with dynamic linking)
cargo test         # Run tests
cargo build        # Build without running
```

## Architecture
The game is organized into Bevy plugins:

- **`src/main.rs`** - App entry point, window config, plugin registration
- **`src/orb/`** - Orb rendering, custom WGSL shaders, orb types and materials
- **`src/gameplay/`** - All game mechanics (see below)
- **`src/environment/`** - Tower scene, day/night cycle, sky shader, lighting
- **`src/familiars/`** - Cat familiar with distraction mechanics
- **`src/ui/`** - HUD, truth popups, logbook
- **`src/audio/`** - Ambient and reactive audio

### Gameplay Module (`src/gameplay/`)
The core idle game loop lives here. All game state is stored in Bevy `Resource`s:

| File | Purpose |
|------|---------|
| `state.rs` | `GameState` enum (Playing, Paused, ShopOpen, etc.) |
| `pondering.rs` | Click-to-ponder, Deep Focus active ability |
| `wisdom.rs` | Wisdom meter, truth generation, deep truths pool |
| `progression.rs` | Arcane Focus Points (AFP), truth counting |
| `acolytes.rs` | Passive wisdom generators (summonable helpers) |
| `generators.rs` | 8-tier idle generators (Candle through Cosmic Eye) |
| `synergies.rs` | Cross-generator bonus multipliers, milestones |
| `shop.rs` | Shop UI for upgrades, generators, orbs |
| `schools.rs` | Schools of Thought (per-run specialization) |
| `transcendence.rs` | Prestige system (reset for permanent bonuses) |
| `moments.rs` | Moments of Clarity (random buff events) |
| `shadow_thoughts.rs` | Shadow drain/dispel mechanic (wrinkler-like) |
| `challenges.rs` | Meditation Challenges for permanent rewards |
| `achievements.rs` | 23 achievements with wisdom multiplier rewards |
| `persistence.rs` | JSON save/load, offline progression, welcome-back UI |

### Key Patterns
- **Multiplier stacking**: Click/passive wisdom flows through multiplier chains: base * shop * moments * transcendence * school * achievements * challenges
- **Permanent vs per-run state**: Achievements, transcendence insight, and challenge completions survive prestige. Generators, acolytes, wisdom, and shop purchases reset.
- **Save file location**: `%APPDATA%/OrbPonderingSimulator/orb_pondering_save.json`

## Bevy 0.18 API Notes
These are critical differences from older Bevy versions:

- **Events are now Messages**: Use `#[derive(Message)]`, `MessageWriter`/`MessageReader`, `add_message`
- **`Timer`**: Use `.just_finished()` or `.is_finished()` (not `.finished()`)
- **`WindowResolution`**: Accepts `(u32, u32)` only
- **`BorderRadius`**: Field on `Node`, not a separate component
- **WGSL material bindings**: Use `@group(#{MATERIAL_BIND_GROUP})` placeholder
- **`ShaderRef`**: Import from `bevy::shader::ShaderRef`
- **`AmbientLight`**: Now a Component on Camera; use `GlobalAmbientLight` for Resource

## Controls
| Key | Action |
|-----|--------|
| Click | Ponder the orb |
| Space | Deep Focus (3x wisdom, 10s) |
| A | Summon Acolyte |
| B | Open Shop |
| L | Open Logbook |
| T | Open Transcendence |
| V | View Achievements |
| C | Challenges (open panel / cancel active) |
| D | Dispel Shadows |
| F | Pet the cat |
| Esc | Pause |

## Conventions
- Game state lives in Bevy `Resource`s, not components (except UI markers)
- UI panels use absolute-positioned backdrop + centered content panel pattern
- State transitions use `GameState` enum with `OnEnter`/`OnExit` for UI spawn/despawn
- New features should integrate with persistence (save/load) and transcendence (reset)
- Serde `#[serde(default)]` on new SaveData fields for backwards compatibility
