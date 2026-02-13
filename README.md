# Orb Pondering Simulator

A meditative idle game where you ponder a mystical orb atop an ancient tower, accumulating wisdom and discovering deep truths about the universe.

Built with [Bevy](https://bevyengine.org/) 0.18 (Rust).

## Features

**Core Loop** - Click the orb to generate wisdom. Fill the wisdom meter to discover deep truths and earn Arcane Focus Points.

**Generators** - Purchase 8 tiers of idle generators (Enchanted Candle, Crystal Ball, Ancient Tome, and more) that produce wisdom automatically. Adjacent generators gain synergy bonuses.

**Acolytes** - Summon acolyte companions for passive wisdom generation.

**Deep Focus** - Activate a temporary 3x wisdom boost on a cooldown.

**Shop** - Spend Arcane Focus Points on upgrades, generators, and collectible orbs with unique visual effects.

**Schools of Thought** - Choose a philosophical school each run (Stoicism, Mysticism, Empiricism, or Nihilism) for different strategic bonuses.

**Transcendence** - Prestige system: sacrifice your progress to earn Insight, then spend it on permanent enlightenments that make future runs stronger.

**Moments of Clarity** - Random events that grant temporary buffs or burst wisdom.

**Shadow Thoughts** - Mysterious shadows that siphon a portion of your wisdom, but can be dispelled for a multiplied payout.

**Meditation Challenges** - Test your discipline with handicap challenges for permanent reward multipliers.

**Achievements** - 23 achievements across 6 categories, each granting a permanent wisdom multiplier.

**Offline Progression** - The orb continues pondering while you're away (50% rate, up to 12 hours).

**Save System** - Auto-saves every 30 seconds to `%APPDATA%/OrbPonderingSimulator/`.

## Building

Requires [Rust](https://rustup.rs/) (edition 2024).

```bash
cargo run
```

Dev builds use dynamic linking and optimized dependencies for fast iteration.

## Controls

| Key | Action |
|-----|--------|
| Click | Ponder the orb |
| Space | Deep Focus |
| A | Summon Acolyte |
| B | Shop |
| L | Logbook |
| T | Transcendence |
| V | Achievements |
| C | Challenges |
| D | Dispel Shadows |
| F | Pet the familiar |
| Esc | Pause |

## License

[MIT](LICENSE)
