# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 23:02 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: `Add gorilla hit explosion animation` (latest commit; inspect `git log -1 --oneline` for hash)
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (22 tests), and `cargo check` passed after gorilla-explosion work.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro/gameplay window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks current turn, scores, active shot animation, gorilla hit explosions, and a short `VictoryDance` animation.
- Local input collects per-turn angle and velocity text using digits, one decimal point, Enter/Tab field movement, Backspace/Delete editing, and a 0..=360 validation cap before submitting `PlayerCommand::SubmitShot`.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, player name header, `left_score >Score< right_score`, per-turn shot prompts, a rotating banana/impact marker for the active shot, and a QBasic-inspired expanding gorilla explosion for gorilla/self hits.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks/resets the sun, alternates turns, applies QBasic-style scoring for gorilla/self hits after a gorilla explosion, plays a winner arm-switching victory dance, then starts a fresh city round.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, sun shock/reset animation behavior, shot-input numeric validation/command creation, score mapping, delayed score/victory/round reset flow, gorilla-explosion mapping/input blocking, and victory-dance pose toggling/input blocking.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- Scoring preserves QBasic `UpdateScores`: opponent scores on self-hit, otherwise thrower scores on opponent hit.
- Current round flow: after any scored gorilla/self hit, input is blocked while the victim gorilla explosion plays; then the score updates, the scoring player alternates raised-arm poses for a short victory dance, and only then a fresh randomly generated round starts. A true game-over/play-to-N flow is still deferred.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: implement gorilla-specific explosion animation for gorilla/self hits before victory dance.
- Changed files: `src/game.rs`, `src/render.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added `GorillaExplosion` animation state to core game state, with victim/scoring-player mapping derived from shot results.
  - Blocked new shot input during gorilla explosions.
  - Delayed score update and victory dance until after the gorilla explosion finishes.
  - Rendered a QBasic-inspired expanding/alternating ring explosion over the victim gorilla while hiding the victim sprite.
  - Added tests for explosion mapping, input blocking, and the explosion -> score -> victory dance -> fresh round sequence.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 22 tests.
  - `cargo check` passed.
- Commit: `Add gorilla hit explosion animation` (latest commit; inspect `git log -1 --oneline` for hash).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Full setup screens for player names, play-to score, gravity, and menu choices are not implemented.
- Input is currently gameplay-only and uses minifb key events; shifted punctuation/numpad edge cases may need polish.
- Banana animation, gorilla explosion, and victory dance are frame-advanced rather than time-accumulated, so speed still needs tuning.
- City/building miss impacts show a generic marker only; generic `DoExplosion` city/building explosion, game-over flow, and audio effects remain unimplemented.
- The current intro text overlays the generated scene and prompts; this should be reorganized when proper setup/menu/game screens are added.
- No final score/game-over screen exists yet, and the original fixed-round vs true play-to-N decision remains open.

## Next recommended task

- Implement generic city/building explosion effects for miss impacts (`DoExplosion`), reusing the new transient animation-state pattern without coupling collision rules to rendering.
