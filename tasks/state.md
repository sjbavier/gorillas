# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 23:10 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: pending this handoff commit (`Add score updates and round reset`)
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (20 tests), and `cargo check` passed after scoring/round-reset work.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro/gameplay window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks the current turn, scores, and an active shot animation.
- Local input collects per-turn angle and velocity text using digits, one decimal point, Enter/Tab field movement, Backspace/Delete editing, and a 0..=360 validation cap before submitting `PlayerCommand::SubmitShot`.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, player name header, `left_score >Score< right_score`, per-turn shot prompts, and a rotating banana for the active shot.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks/resets the sun, alternates turns, applies QBasic-style scoring for gorilla/self hits, and starts a fresh city round after a scored hit.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, sun shock/reset animation behavior, shot-input numeric validation/command creation, and score updates including self-hit.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- Scoring now preserves QBasic `UpdateScores`: opponent scores on self-hit, otherwise thrower scores on opponent hit. Current implementation starts a fresh randomly generated round after any scored gorilla hit. A true game-over/play-to-N flow is still deferred.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: implement score/turn outcome handling for shot results.
- Changed files: `src/game.rs`, `src/render.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added QBasic-style score mapping from `ShotResult` to scoring player, including opponent scoring for `HitSelf`.
  - Applied scoring after shot impact hold completes and started a fresh city/gorilla/sun round after scored hits.
  - Rejected shot submissions from non-current players while a turn is active.
  - Rendered player names at the top and current score as `left_score >Score< right_score`.
  - Added tests for score mapping and scored-hit round reset.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 20 tests.
  - `cargo check` passed.
- Commit subject: `Add score updates and round reset` (see Git HEAD after commit for exact hash).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Full setup screens for player names, play-to score, gravity, and menu choices are not implemented.
- Input is currently gameplay-only and uses minifb key events; shifted punctuation/numpad edge cases may need polish.
- Banana animation is frame-advanced rather than time-accumulated, so speed still needs tuning.
- Shot impacts show a generic marker only; city explosions, gorilla explosions, victory dance, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene and prompts; this should be reorganized when proper setup/menu/game screens are added.
- No final score/game-over screen exists yet, and the original fixed-round vs true play-to-N decision remains open.

## Next recommended task

- Implement gorilla-specific explosion/victory feedback for scored hits, or add a minimal game-over/play-to score flow if choosing to resolve the round-count decision first.
