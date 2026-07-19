# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-19 19:48 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: latest commit for this pass (`Centralize QBasic scaling helper`; inspect `git log` for exact hash).
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (40 tests), and `cargo check` passed after centralizing QBasic `Scl` scaling.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Startup shows a QBasic-inspired intro screen, waits for any key, collects setup input, shows a `V = View Intro` / `P = Play Game` menu, and then starts local gameplay.
- `Esc` is the documented global quit key and is handled in the main loop from intro, setup, menu, gameplay, and game-over screens.
- Setup input collects Player 1/Player 2 names (blank defaults; 10-char max), fixed round count (blank default 3), and gravity (blank default 9.8), then applies those values to `GameState` before the menu.
- Game-over screen shows final scores/rounds and accepts any key to return to setup for another match.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks current turn, scores, completed rounds, a fixed `round_limit`, active shot animation, generic building-hit shot explosions, gorilla hit explosions, victory dance, and `ScreenState::{Intro, Setup, Menu, Playing, GameOver}`.
- Random scene generation defaults to QBasic-like timer entropy. `GameConfig::random_seed` and the `GORILLAS_SEED=<u64>` environment variable allow deterministic city/wind/gorilla scene sequences for tests/debugging.
- Local input is separate from rules: setup/menu/key-continue events and per-turn `SubmitShot` commands flow into game state.
- Renderer draws intro/setup/menu screens, gameplay skyline/wind/sun/gorillas/header/prompts/banana/explosions/victory dance, and final game-over scores.
- Renderer primitive helpers cover clear, pixel text, QBasic-style centered text support, line drawing, rectangle outline/fill, circles/arcs, and set/get pixel access; helper behavior has unit-test coverage.
- Config centralizes QBasic SCREEN 9/EGA color attributes, semantic palette colors, selected `ScreenMode`, and a tested `scl(n, mode)` helper matching original EGA/CGA `Scl` behavior. Runtime still targets EGA 640x350 only.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Animation uses elapsed frame time from `main` and a 0.02-second logical animation step, matching QBasic `PlotShot` / `Rest .02` shot pacing while avoiding busy waits; catch-up work is capped per rendered frame.
- Animation timing constants are tuned/documented against QBasic pacing: quick final-banana impact hold, 14-frame generic explosions, 48-frame gorilla explosions, and 80-frame victory dance with 0.2-second pose toggles.
- Audio output is intentionally out of scope for the first playable version. `audio.rs` exposes no-op intro/throw/explosion/gorilla-explosion/victory methods, and `GameState` queues rendering-independent `AudioCue`s for gameplay events.
- Unit tests cover config palette/seed/scaling behavior, city bounds/window bounds, wind range, deterministic seeded scenes, gorilla placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transform, off-screen stop behavior, shot collision outcomes, active shot creation, sun shock/reset, setup defaults/limits, setup flow state transitions, shot input validation/commands, global quit key mapping, score mapping, round/game-over flow, game-over continuation, explosions, victory dance, audio cue queuing, delta-time animation accumulation/catch-up capping, animation timing constants, and renderer helper primitives.
- `README.md` documents build/run commands, optional `GORILLAS_SEED`, controls, local flow, scaling scope, scope notes, and a manual test checklist for the playable local flow.

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
- Palette/scaling strategy: keep QBasic attribute constants, semantic colors, `ScreenMode`, and `scl` centralized in `config.rs`; EGA is the active runtime target, while CGA support remains deferred.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: centralize QBasic `Scl(n)` scaling behavior for future multi-resolution support.
- Changed files: `src/config.rs`, `src/entities.rs`, `src/game.rs`, `src/physics.rs`, `src/render.rs`, `README.md`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added `ScreenMode::{Ega,Cga}` and stored the active mode in `GameConfig`, defaulting to EGA 640x350.
  - Implemented and tested `config::scl(n, mode)` to match the original `GORILLA.BAS` `Scl` routine, including fractional CGA sentinel behavior like `2.9 -> 1`.
  - Routed sun construction/ray constants and gorilla rooftop placement offsets through mode-aware helpers while preserving the current EGA output.
  - Documented that scaling behavior is centralized but alternate runtime resolutions remain deferred.
  - Marked `Scl` rendering/routine checklist items complete in `tasks/task.md`.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 40 tests.
  - `cargo check` passed.
- Commit: latest commit for this pass (`Centralize QBasic scaling helper`; inspect `git log` for exact hash).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Setup text input currently supports uppercase letters, digits, spaces, and decimal points; shifted punctuation/lowercase/numpad edge cases may need polish.
- The menu's `V = View Intro` returns to the text intro rather than reproducing the full original gorilla musical intro animation.
- Building explosions are visual only; they do not yet remove/damage city geometry.
- Runtime remains fixed at EGA 640x350 despite the new mode-aware `Scl` helper; CGA/scale selection is deferred.
- Timing constants were tuned by source review and automated tests in this environment; visual feel should still be manually playtested in a window.
- Audio output is still silent by design.

## Next recommended task

- Perform a manual playable-flow pass with a fixed `GORILLAS_SEED`, update the manual checklist for intro/setup/skyline/gorillas/wind/building/gorilla collisions/scores, and address the first observed gameplay polish issue.
