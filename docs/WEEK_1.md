## Week 1 — Core Simulation

Goal: make bots fight in a deterministic arena.

Tasks:

- [x] Create Cargo workspace
- [ ] Add core domain types:
  - [ ] `BotId`
  - [ ] `MatchId`
  - [ ] `Position`
  - [ ] `Direction`
  - [ ] `BotAction`
  - [ ] `ArenaConfig`
- [ ] Implement `World`
- [ ] Implement fixed-tick match loop
- [ ] Implement movement rules
- [ ] Implement collision rules
- [ ] Implement attack rules
- [ ] Implement damage and death
- [ ] Implement winner detection
- [ ] Add seeded RNG
- [ ] Define `Bot` trait
- [ ] Define `WorldView`
- [ ] Implement `RandomBot`
- [ ] Implement `AggressiveBot`
- [ ] Add initial CLI command: `colosseum run`
- [ ] Add tests for:
  - [ ] movement
  - [ ] attack
  - [ ] death
  - [ ] winner detection
  - [ ] deterministic result with same seed

Definition of done:

```bash
colosseum run --bots aggressive,random --seed 42
```

prints a deterministic match result.
