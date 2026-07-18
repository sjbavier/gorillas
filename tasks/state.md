# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 23:22 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: dd6fd30 (`Add victory dance between scoring rounds`)
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (21 tests), and `cargo check` passed after victory-dance work.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro/gameplay window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks current turn, scores, active shot animation, and a short `VictoryDance` animation.
- Local input collects per-turn angle and velocity text using digits, one decimal point, Enter/Tab field movement, Backspace/Delete editing, and a 0..=360 validation cap before submitting `PlayerCommand::SubmitShot`.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, player name header, `left_score >Score< right_score`, per-turn shot prompts, and a rotating banana/impact marker for the active shot.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks/resets the sun, alternates turns, applies QBasic-style scoring for gorilla/self hits, plays a winner arm-switching victory dance, then starts a fresh city round.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, sun shock/reset animation behavior, shot-input numeric validation/command creation, score updates including self-hit, scored round reset timing, and victory-dance pose toggling/input blocking.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- Scoring preserves QBasic `UpdateScores`: opponent scores on self-hit, otherwise thrower scores on opponent hit.
- Current round flow: after any scored gorilla/self hit, input is blocked while the scoring player alternates raised-arm poses for a short victory dance; only then is a fresh randomly generated round started. A true game-over/play-to-N flow is still deferred.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: implement victory dance pose switching before scored round reset.
- Changed files: `src/game.rs`, `src/main.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added `VictoryDance` state to core game state with alternating winner arm poses.
  - Blocked shot input while a victory dance is active via `GameState::accepts_shot_input`.
  - Changed scored-hit flow so scoring starts the dance first and generates the next city round only after the dance finishes.
  - Reset gorilla poses to arms-down when a fresh round starts.
  - Added tests for scored round-reset timing, victory pose alternation, and shot blocking during victory dance.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 21 tests.
  - `cargo check` passed.
- Commit: dd6fd30 `Add victory dance between scoring rounds`.

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Full setup screens for player names, play-to score, gravity, and menu choices are not implemented.
- Input is currently gameplay-only and uses minifb key events; shifted punctuation/numpad edge cases may need polish.
- Banana animation and victory dance are frame-advanced rather than time-accumulated, so speed still needs tuning.
- Shot impacts show a generic marker only; city explosions, gorilla explosions, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene and prompts; this should be reorganized when proper setup/menu/game screens are added.
- No final score/game-over screen exists yet, and the original fixed-round vs true play-to-N decision remains open.

## Next recommended task

- Implement gorilla-specific explosion animation for gorilla/self hits before the victory dance, or add generic city/building explosion effects for miss impacts.
