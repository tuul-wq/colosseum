# Future Scaling Tasks

These are intentionally not month-one requirements.

They describe how the product can grow.

---

## Storage Scaling

- [ ] Add SQLite repository
- [ ] Add PostgreSQL repository
- [ ] Add migrations
- [ ] Store match summaries in DB
- [ ] Store replay metadata in DB
- [ ] Store replay files in object storage
- [ ] Add pagination for match listing
- [ ] Add query filters:
  - [ ] bot name
  - [ ] winner
  - [ ] date range
  - [ ] seed
  - [ ] tournament ID

---

## API Scaling

- [ ] Add OpenAPI documentation
- [ ] Add request validation
- [ ] Add structured API error codes
- [ ] Add pagination to `GET /matches`
- [ ] Add `POST /tournaments/run`
- [ ] Add `GET /tournaments/:id`
- [ ] Add `GET /bots`
- [ ] Add `GET /replays/:id`
- [ ] Add authentication for admin operations
- [ ] Add rate limiting

---

## WebSocket Spectators

- [ ] Add `GET /matches/:id/stream`
- [ ] Stream live match events
- [ ] Support replay streaming
- [ ] Add spectator connection tracking
- [ ] Add backpressure handling
- [ ] Add bounded event buffers
- [ ] Add reconnect support

---

## External Bots

- [ ] Support bot executable processes
- [ ] Define JSON stdin/stdout protocol
- [ ] Add bot decision timeout
- [ ] Kill unhealthy bot processes
- [ ] Capture bot stderr logs
- [ ] Add protocol versioning
- [ ] Add external bot examples:
  - [ ] Rust
  - [ ] Python
  - [ ] JavaScript/TypeScript

---

## Sandboxing

- [ ] Add process resource limits
- [ ] Add memory limits
- [ ] Add CPU time limits
- [ ] Explore WASM bot runtime
- [ ] Add WASI-based bot execution
- [ ] Add deterministic bot runtime constraints

---

## Distributed Execution

- [ ] Split API server from match worker
- [ ] Add job queue abstraction
- [ ] Add tournament worker process
- [ ] Add Redis or Postgres-backed queue
- [ ] Add worker heartbeats
- [ ] Add retry handling
- [ ] Add match status tracking
- [ ] Add horizontal worker scaling

---

## Observability

- [ ] Add Prometheus metrics
- [ ] Add `/metrics`
- [ ] Track match duration
- [ ] Track tournament duration
- [ ] Track active matches
- [ ] Track replay write failures
- [ ] Add tracing spans per match
- [ ] Add structured correlation IDs
- [ ] Add dashboard example

---

## Cloud Deployment

- [ ] Add production Dockerfile
- [ ] Add docker-compose for local API + DB
- [ ] Add environment-based config
- [ ] Add health checks
- [ ] Add graceful shutdown
- [ ] Add Fly.io/Render/Railway deployment guide
- [ ] Add cloud database config
- [ ] Add object storage integration for replays

---

## Game Design Expansion

- [ ] Add pickups
- [ ] Add objectives
- [ ] Add fog of war
- [ ] Add different worlds
- [ ] Add world generator
- [ ] Add hazards
- [ ] Add bot vision range
- [ ] Add multiple game modes:
  - [ ] deathmatch
  - [ ] king of the hill
  - [ ] capture crystal

---

## UX Improvements

- [ ] Add Ratatui replay viewer
- [ ] Add terminal tournament dashboard
- [ ] Add browser spectator page
- [ ] Add replay controls:
  - [ ] pause
  - [ ] step
  - [ ] speed up
  - [ ] inspect bot
- [ ] Add match timeline visualization
