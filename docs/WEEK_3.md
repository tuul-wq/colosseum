## Week 3 — Tournament Runner

Goal: run many matches and produce a useful leaderboard.

Tasks:

- [ ] Define `TournamentConfig`
- [ ] Define `TournamentRunner`
- [ ] Generate deterministic seed sequence
- [ ] Run many matches
- [ ] Aggregate match standings
- [ ] Add `TournamentStanding`
- [ ] Add leaderboard output
- [ ] Add command: `colosseum tournament`
- [ ] Add `--matches`
- [ ] Add `--parallelism`
- [ ] Parallelize match execution with Rayon
- [ ] Print elapsed execution time
- [ ] Add tests for:
  - [ ] aggregation
  - [ ] deterministic tournament output
  - [ ] leaderboard sorting

Definition of done:

```bash
colosseum tournament \
  --bots aggressive,random,coward,sniper \
  --matches 1000 \
  --parallelism 4
```

produces a stable leaderboard.
