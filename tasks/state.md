# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 22:27 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test`, `cargo check` all passed after rendering-independent banana physics work.

## Current implementation status

- Cargo binary crate exists and builds.
- Fixed EGA-like target resolution is 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, and creates a happy sun.
- Renderer draws the skyline, wind arrow, sun, and simple vector gorillas.
- Rendering-independent banana trajectory helpers now cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, and off-screen detection.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, and off-screen simulation stop behavior.

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

- Selected task: implement rendering-independent banana trajectory physics and tests for the original QBasic projectile formula.
- Changed files: `src/physics.rs`, `src/main.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added pure shot physics helpers for angle mirroring, banana spawn offsets, trajectory sampling, rotation-frame calculation, off-screen detection, and finite simulation.
  - Added unit tests for the QBasic formula, wind acceleration, player-2 angle transformation, spawn offsets, and off-screen stopping.
  - Updated task checklist entries for completed physics/test items.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 8 tests.
  - `cargo check` passed.
- Commit: `4f00604 Add banana trajectory physics`.

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and can become dirty after Cargo commands; avoid staging them for implementation commits.
- Gameplay input/setup screens are not implemented.
- Banana physics is pure/tested but not yet wired into turn input, rendering, collision, explosions, or scoring.
- Collision detection, sun-hit transitions, explosions, scoring, turn flow, victory dance, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene; this is acceptable for early static-scene verification but may be reorganized when proper setup/menu screens are added.

## Next recommended task

- Wire the pure banana trajectory into a minimal gameplay/animation state that can render a test shot path, then add geometry-based building collision using the existing city model.
