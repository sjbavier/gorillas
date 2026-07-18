# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 23:05 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: latest commit is `Add building hit explosion animation`; inspect `git log -1 --oneline` for the hash.
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (24 tests), and `cargo check` passed after generic city/building explosion work.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Intro/gameplay window renders with QBasic-inspired instructions; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks current turn, scores, active shot animation, generic building-hit shot explosions, gorilla hit explosions, and a short `VictoryDance` animation.
- Local input collects per-turn angle and velocity text using digits, one decimal point, Enter/Tab field movement, Backspace/Delete editing, and a 0..=360 validation cap before submitting `PlayerCommand::SubmitShot`.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, player name header, `left_score >Score< right_score`, per-turn shot prompts, a rotating banana/impact marker for the active shot, QBasic-inspired expanding/erasing rings for building-hit `DoExplosion`, and a QBasic-inspired expanding gorilla explosion for gorilla/self hits.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks/resets the sun, alternates turns, starts a transient generic explosion for building hits, or applies QBasic-style scoring after a gorilla explosion and victory dance before a fresh city round.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, sun shock/reset animation behavior, shot-input numeric validation/command creation, score mapping, delayed score/victory/round reset flow, generic shot-explosion mapping/input blocking, gorilla-explosion mapping/input blocking, and victory-dance pose toggling/input blocking.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- Scoring preserves QBasic `UpdateScores`: opponent scores on self-hit, otherwise thrower scores on opponent hit.
- Current round flow: building hits play a short generic explosion and then allow the next turn in the same city; gorilla/self hits block input while the victim gorilla explosion plays, then update score, run winner victory dance, and start a fresh randomly generated round. A true game-over/play-to-N flow is still deferred.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: implement generic city/building explosion effects for miss impacts (`DoExplosion`).
- Changed files: `src/game.rs`, `src/render.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Added `ShotExplosion` transient animation state for building-hit miss impacts.
  - Derived generic explosion position from `ShotResolution` only for `CollisionKind::Building` misses, leaving edge/bottom misses without an explosion.
  - Blocked new shot input while the generic explosion plays, then resumed the next player's turn in the same city.
  - Rendered QBasic-inspired expanding colored rings followed by background-color erasing rings at the impact point.
  - Added tests for building-miss explosion mapping, input blocking, completion, and edge-miss non-explosion behavior.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 24 tests.
  - `cargo check` passed.
- Commit: `Add building hit explosion animation` (latest commit; inspect `git log -1 --oneline` for hash).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Full setup screens for player names, play-to score, gravity, and menu choices are not implemented.
- Input is currently gameplay-only and uses minifb key events; shifted punctuation/numpad edge cases may need polish.
- Banana animation, explosions, and victory dance are frame-advanced rather than time-accumulated, so speed still needs tuning.
- Building explosions are visual only; they do not yet remove/damage city geometry.
- The current intro text overlays the generated scene and prompts; this should be reorganized when proper setup/menu/game screens are added.
- No final score/game-over screen exists yet, and the original fixed-round vs true play-to-N decision remains open.

## Next recommended task

- Implement setup/menu flow for player names, play-to score, and gravity, or begin game-over/play-to-N flow if keeping setup deferred.
