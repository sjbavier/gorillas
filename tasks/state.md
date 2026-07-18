# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 23:32 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: this commit (`Add no-op audio event hooks`).
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (28 tests), and `cargo check` passed after no-op audio hook changes.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Startup shows a QBasic-inspired intro screen, waits for any key, collects setup input, shows a `V = View Intro` / `P = Play Game` menu, and then starts local gameplay.
- Setup input collects Player 1/Player 2 names (blank defaults; 10-char max), fixed round count (blank default 3), and gravity (blank default 9.8), then applies those values to `GameState` before the menu.
- Game-over screen shows final scores/rounds and accepts any key to return to setup for another match.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks current turn, scores, completed rounds, a fixed `round_limit`, active shot animation, generic building-hit shot explosions, gorilla hit explosions, victory dance, and `ScreenState::{Intro, Setup, Menu, Playing, GameOver}`.
- Local input is separate from rules: setup/menu/key-continue events and per-turn `SubmitShot` commands flow into game state.
- Renderer draws intro/setup/menu screens, gameplay skyline/wind/sun/gorillas/header/prompts/banana/explosions/victory dance, and final game-over scores.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Audio output is intentionally out of scope for the first playable version. `audio.rs` exposes no-op intro/throw/explosion/gorilla-explosion/victory methods, and `GameState` queues rendering-independent `AudioCue`s for gameplay events.
- Unit tests cover city bounds/window bounds, wind range, gorilla placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transform, off-screen stop behavior, shot collision outcomes, active shot creation, sun shock/reset, setup defaults/limits, setup flow state transitions, shot input validation/commands, score mapping, round/game-over flow, game-over continuation, explosions, victory dance, and audio cue queuing.
- `README.md` documents build/run commands, controls, local flow, scope notes, and a manual test checklist for the playable local flow.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- Real audio is deferred; current audio methods are no-op call sites with original QBasic `PLAY` strings documented in comments.
- Scoring preserves QBasic `UpdateScores`: opponent scores on self-hit, otherwise thrower scores on opponent hit.
- Round/game-over policy intentionally preserves original `PlayGame`: the prompt says "Play to how many total points", but the implementation treats it as a fixed number of rounds.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: add no-op audio call sites around intro/throw/explosion/victory events.
- Changed files: `src/audio.rs`, `src/game.rs`, `src/main.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Expanded `audio.rs` with no-op methods for original intro, throw, generic explosion, gorilla explosion, and victory cues, preserving the QBasic `PLAY` strings as comments.
  - Added `AudioCue` queuing/draining to `GameState` so gameplay events request sound without coupling rules to the audio backend.
  - Drained cues in `main` and dispatched them to the current no-op `Audio` implementation.
  - Added unit assertions for throw/explosion/victory cue queuing and marked no-op audio scope decisions complete in `tasks/task.md`.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 28 tests.
  - `cargo check` passed.
- Commit: this commit (`Add no-op audio event hooks`).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Setup text input currently supports uppercase letters, digits, spaces, and decimal points; shifted punctuation/lowercase/numpad edge cases may need polish.
- The menu's `V = View Intro` returns to the text intro rather than reproducing the full original gorilla musical intro animation.
- Banana animation, explosions, and victory dance are frame-advanced rather than time-accumulated, so speed still needs tuning.
- Building explosions are visual only; they do not yet remove/damage city geometry.
- Audio output is still silent by design.

## Next recommended task

- Tune animation timing/speed to use elapsed frame time more consistently instead of frame-count-only progression.
