## Week 2 — Events, Replay, Scoring

Goal: make matches inspectable and replayable.

Tasks:

- [ ] Define `GameEvent`
- [ ] Emit events from simulation
- [ ] Add replay header/version
- [ ] Implement replay writer
- [ ] Implement replay reader
- [ ] Store replay as JSON Lines
- [ ] Add command: `colosseum inspect <file>`
- [ ] Add command: `colosseum replay <file>`
- [ ] Define `ScoreCalculator` trait
- [ ] Implement default score calculator
- [ ] Add `MatchStanding`
- [ ] Add `CowardBot`
- [ ] Add `SniperBot`
- [ ] Add tests for:
  - [ ] event emission
  - [ ] replay read/write
  - [ ] scoring
  - [ ] deterministic replay

Definition of done:

```bash
colosseum run --bots aggressive,coward,sniper --seed 7 --save-replay
colosseum inspect ./replays/<match>.jsonl
colosseum replay ./replays/<match>.jsonl
```

works end to end.
