# Colosseum

Deterministic bot arena and tournament engine written in Rust.

Colosseum simulates programmable bots fighting in a grid-based arena, records replayable event logs, runs tournaments in parallel, and exposes match data through a CLI and HTTP API.

The project is intentionally designed as a backend/systems-focused Rust application rather than a traditional game.

---

## Why?

Most game projects showcase rendering and UI.

Colosseum focuses on:

* deterministic simulation
* event-driven architecture
* CPU-bound parallel workloads
* async APIs
* replay systems
* clean modular design
* scalability boundaries

The goal is to explore how competitive game infrastructure could be built in Rust.

---

## Features

* Deterministic fixed-tick simulation
* Multiple built-in bots
* Replay recording and playback
* Event-based scoring system
* Parallel tournament execution
* REST API with Axum
* Structured logging with tracing
* Modular Cargo workspace architecture

---

## Example

Run a single match:

```bash
cargo run -p cli -- run \
  --bots aggressive,random,coward \
  --seed 42
```

Run a tournament:

```bash
cargo run -p cli -- tournament \
  --bots aggressive,random,coward,sniper \
  --matches 1000 \
  --parallelism 4
```

Start API server:

```bash
cargo run -p api-server
```

---

## Workspace

```text
colosseum/
├── crates/
│   ├── colosseum-domain/
│   ├── colosseum-simulation/
│   ├── colosseum-bots/
│   ├── colosseum-replay/
│   ├── colosseum-scoring/
│   ├── colosseum-tournament/
│   ├── colosseum-storage/
│   └── colosseum-api/
│
├── apps/
│   ├── cli/
│   └── api-server/
```

---

## Architecture

Colosseum is built around an event-driven simulation model.

The engine produces immutable events:

```rust
GameEvent::BotMoved { .. }
GameEvent::DamageDealt { .. }
GameEvent::BotKilled { .. }
```

Replays, scoring, analytics, and debugging are derived from those events.

Simulation logic is isolated from:

* HTTP transport
* persistence
* CLI rendering
* replay storage

This keeps the core deterministic and testable.

---

## Bot API

Bots implement a shared trait:

```rust
pub trait Bot: Send {
    fn name(&self) -> &str;

    fn decide(&mut self, view: WorldView) -> BotAction;
}
```

Current built-in bots:

* RandomBot
* AggressiveBot
* CowardBot
* SniperBot

---

## Tech Stack

* Rust
* Tokio
* Axum
* Rayon
* Clap
* Serde
* Tracing

---

## Future Work

* WebSocket spectators
* PostgreSQL storage
* external process bots
* WASM bot sandboxing
* distributed tournament workers
* TUI replay viewer
* cloud deployment
