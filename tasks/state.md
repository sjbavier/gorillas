# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 22:38 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: `9bd4279`
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (15 tests), and `cargo check` passed after minimal shot animation wiring.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks the current turn, and can host an active shot animation.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, and a rotating banana for the active shot.
- Space starts a temporary demo shot for the current player until real setup/turn input is implemented.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks the sun when crossed, resets the sun after the shot, and alternates the demo turn.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, and sun shock/reset animation behavior.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions should flow as explicit commands/events where practical.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: wire shot resolution into a minimal gameplay/animation state.
- Changed files: `src/game.rs`, `src/input.rs`, `src/main.rs`, `src/render.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added `ActiveShot` state and `GameState::submit_shot`/`update_animation` so pure shot resolution can drive an on-screen banana animation.
  - Added a temporary Space-key demo shot trigger while real per-turn numeric input remains pending.
  - Added banana rendering with four rotation frames and an impact marker.
  - Switched the throwing gorilla pose during launch and reset it during animation.
  - Updated sun visual state to become shocked when the animated banana crosses the sun and reset after the shot finishes.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 15 tests.
  - `cargo check` passed.
- Commit: `9bd4279` / `Add minimal shot animation`.

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Gameplay setup screens and numeric angle/velocity input are not implemented.
- The Space-triggered shot is only a temporary manual/demo hook, not the final turn input flow.
- Banana animation is frame-advanced rather than time-accumulated, so speed still needs tuning.
- Shot impacts show a generic marker only; city explosions, gorilla explosions, scoring, victory dance, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene; this is acceptable for early static-scene/shot verification but should be reorganized when proper setup/menu screens are added.

## Next recommended task

- Implement per-turn angle/velocity input as local input that submits `PlayerCommand::SubmitShot`, replacing the temporary Space-key demo shot while preserving the rendering-independent shot resolution path.
