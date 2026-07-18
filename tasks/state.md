# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 22:22 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test`, `cargo check` all passed after gorilla/sun static-scene work.

## Current implementation status

- Cargo binary crate exists and builds.
- Fixed EGA-like target resolution is 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `render`, `input`, and `audio`.
- Intro window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, and creates a happy sun.
- Renderer draws the skyline, wind arrow, sun, and simple vector gorillas.
- Unit tests cover city bounds/window bounds, wind range, and gorilla rooftop placement.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions should flow as explicit commands/events where practical.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy is not yet finalized; geometry/mask collision is likely the next practical step, though QBasic used pixel-color collision with `POINT`.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: update repository handoff documentation so `tasks/state.md` stays compact and Git commits carry detailed chronological logs.
- Changed files: `tasks/repo_agent_prompt.md`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Documented that `tasks/state.md` is a compact current-state snapshot, not an append-only log.
  - Added guidance for using `git log` / `git show` when historical context is needed.
  - Added completion and handoff requirements for reviewing diffs, staging only task-related files, and committing verified work.
  - Added a repository state/Git workflow section to `tasks/task.md`.
  - Compacted this file into the new snapshot format.
- Verification:
  - Markdown-only documentation update; no Rust build was required for this docs task.
  - Final diff reviewed with `git diff -- tasks/repo_agent_prompt.md tasks/task.md tasks/state.md`.
- Commit: subject `Document compact state and git handoff workflow`.

## Known issues / deferred work

- Uncommitted non-doc changes/build artifacts were present while making this documentation update; this task stages and commits only the related markdown files.
- Gameplay input/setup screens are not implemented.
- Banana physics and trajectory rendering are not implemented.
- Collision detection, sun-hit transitions, explosions, scoring, turn flow, victory dance, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene; this is acceptable for early static-scene verification but may be reorganized when proper setup/menu screens are added.

## Next recommended task

- Implement rendering-independent banana trajectory physics in `physics.rs` and tests for the original QBasic projectile formula before wiring it into shot input/rendering.
