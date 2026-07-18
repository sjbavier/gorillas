# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-18 15:31 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: `Map renderer colors through QBasic palette` (latest commit for this pass; inspect `git log` for exact hash).
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (36 tests), and `cargo check` passed after QBasic palette mapping.

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
- Local input is separate from rules: setup/menu/key-continue events and per-turn `SubmitShot` commands flow into game state.
- Renderer draws intro/setup/menu screens, gameplay skyline/wind/sun/gorillas/header/prompts/banana/explosions/victory dance, and final game-over scores.
- Renderer primitive helpers cover clear, pixel text, QBasic-style centered text support, line drawing, rectangle outline/fill, circles/arcs, and set/get pixel access; helper behavior has unit-test coverage.
- Config now centralizes QBasic SCREEN 9/EGA color attributes and semantic palette colors for background, gorilla/object, buildings, lit/unlit windows, sun, explosion/wind, text, prompts, and banana.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Animation uses elapsed frame time from `main` and a 0.02-second logical animation step, matching QBasic `Rest .02` shot pacing intent while avoiding busy waits; catch-up work is capped per rendered frame.
- Audio output is intentionally out of scope for the first playable version. `audio.rs` exposes no-op intro/throw/explosion/gorilla-explosion/victory methods, and `GameState` queues rendering-independent `AudioCue`s for gameplay events.
- Unit tests cover config palette mapping, city bounds/window bounds, wind range, gorilla placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transform, off-screen stop behavior, shot collision outcomes, active shot creation, sun shock/reset, setup defaults/limits, setup flow state transitions, shot input validation/commands, global quit key mapping, score mapping, round/game-over flow, game-over continuation, explosions, victory dance, audio cue queuing, delta-time animation accumulation/catch-up capping, and renderer helper primitives.
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
- Palette strategy: keep QBasic attribute constants and semantic colors centralized in `config.rs`; renderer/city should consume semantic palette fields rather than hard-coded gameplay colors.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: tune color palette mapping toward the original EGA/QBasic look.
- Changed files: `src/config.rs`, `src/city.rs`, `src/render.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added QBasic SCREEN 9 attribute constants, a `qbasic_screen9_color` mapper, and semantic `Palette::qbasic_ega` fields.
  - Routed city building/window generation through semantic palette colors instead of local hard-coded colors.
  - Routed renderer screen clears, menu gorilla feature color, banana color, prompt/score highlight text, dim text, and alternate gorilla explosion ring color through palette helpers.
  - Marked palette mapping and `SetScreen` checklist items complete in `tasks/task.md`.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 36 tests.
  - `cargo check` passed.
- Commit: `Map renderer colors through QBasic palette` (latest commit for this pass; inspect `git log` for exact hash).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Setup text input currently supports uppercase letters, digits, spaces, and decimal points; shifted punctuation/lowercase/numpad edge cases may need polish.
- The menu's `V = View Intro` returns to the text intro rather than reproducing the full original gorilla musical intro animation.
- Animation uses QBasic-inspired logical pacing, but feel still needs manual tuning.
- Building explosions are visual only; they do not yet remove/damage city geometry.
- Audio output is still silent by design.

## Next recommended task

- Add deterministic random seed support for repeatable tests and optional reproducible local scenes.
