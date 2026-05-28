## Week 4 — API, Storage, Polish

Goal: make the project look backend-ready.

Tasks:

- [ ] Define `MatchRepository` trait
- [ ] Implement `FileMatchRepository`
- [ ] Store match summaries
- [ ] Add command: `colosseum matches`
- [ ] Add Axum API crate
- [ ] Add API server binary
- [ ] Implement `GET /health`
- [ ] Implement `GET /matches`
- [ ] Implement `GET /matches/:id`
- [ ] Implement `GET /leaderboard`
- [ ] Implement `POST /matches/run`
- [ ] Add structured logging with `tracing`
- [ ] Add basic config through CLI/env
- [ ] Add Dockerfile
- [ ] Write README
- [ ] Add architecture notes
- [ ] Add tests for API handlers where practical

Definition of done:

```bash
colosseum serve --port 8080
curl http://localhost:8080/health
curl http://localhost:8080/matches
```

works locally.
