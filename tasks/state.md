# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 22:50 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: Git HEAD after this committed handoff (`Add per-turn shot input`)
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (18 tests), and `cargo check` passed after per-turn angle/velocity input work.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro/gameplay window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks the current turn, and can host an active shot animation.
- Local input now collects per-turn angle and velocity text using digits, one decimal point, Enter/Tab field movement, Backspace/Delete editing, and a 0..=360 validation cap before submitting `PlayerCommand::SubmitShot`.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, per-turn shot prompts, and a rotating banana for the active shot.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks the sun when crossed, resets the sun after the shot, and alternates the turn.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, sun shock/reset animation behavior, and shot-input numeric validation/command creation.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: implement per-turn angle/velocity input that submits shot commands.
- Changed files: `src/input.rs`, `src/main.rs`, `src/render.rs`, `src/game.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Replaced the temporary Space-key demo shot with a `ShotInputState` that captures angle and velocity for the current player.
  - Added numeric input validation matching the original constraints closely enough for this backend: digits, a single decimal point, maximum length, and 0..=360 parsed values.
  - Submitted local shots as `PlayerCommand::SubmitShot` and kept `GameState::submit_shot` as the rendering-independent turn execution path.
  - Rendered current-player angle/velocity prompts near the appropriate side of the screen.
  - Added unit tests for shot number validation, decimal filtering, range capping, and command creation.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 18 tests.
  - `cargo check` passed.
- Commit subject: `Add per-turn shot input` (see Git HEAD for exact hash).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Full setup screens for player names, play-to score, gravity, and menu choices are not implemented.
- Input is currently gameplay-only and uses minifb key events; shifted punctuation/numpad edge cases may need polish.
- Banana animation is frame-advanced rather than time-accumulated, so speed still needs tuning.
- Shot impacts show a generic marker only; city explosions, gorilla explosions, scoring, victory dance, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene and prompts; this should be reorganized when proper setup/menu/game screens are added.

## Next recommended task

- Implement score/turn outcome handling for shot results: update scores for opponent hits and self-hits, show the current score header, and prepare round reset behavior after a scored hit.
