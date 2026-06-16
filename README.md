# Colosseum

A Rust tactical combat engine where programmable heroes fight in formation-based battles.

Colosseum is built as a backend/systems project first: domain modeling, turn logic, world state, hero abilities, and AI decision-making are separated into small workspace crates. The goal is to grow it into a deterministic battle simulator with replays, tournaments, and external bot support.

## Current State

- Rust 2024 Cargo workspace
- Hero domain model with health, stats, id, and abilities
- Mage and Warrior hero archetypes
- Position-based formations: frontline, midline, backline
- World state for two opposing sides
- Movement, placement, removal, and ally swapping logic
- Early `HeroAI` decision interface
- Starter CLI for creating two 3-hero teams

## Roadmap

- Complete turn-based combat loop
- Ability targeting and damage resolution
- Smarter built-in hero AI
- Deterministic fight simulation
- Replayable battle event logs
- Tournament runner
- CLI fight output and summaries
- HTTP API or TUI viewer
- External bot support

## Run

```bash
cargo run -p cli -- mage,warrior,mage warrior,mage,warrior
```

## Test

```bash
cargo test
```

## Workspace

```text
apps/
  cli/          Command-line interface

crates/
  domain/      Heroes, stats, abilities, positions
  world/       Formations and battlefield state
  controller/  Hero AI and turn decisions
  arena/       Battle orchestration, in progress
```

## Tech Stack

Rust, Cargo workspaces, Clap, Rand, Thiserror
