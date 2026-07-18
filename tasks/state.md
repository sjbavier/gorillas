# Current State: Rust Gorillas Port

## Snapshot

- Last updated: 2026-07-17 23:09 EDT
- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Current commit: latest commit will be `Add fixed-round game over screen` after this handoff commit.
- Source reference: `GORILLA.BAS`
- Current backend: windowed 2D via `minifb` (`macroquad` was preferred initially but failed on the available toolchain/dependency set).
- Latest verified commands: `cargo fmt`, `cargo test` (25 tests), and `cargo check` passed after fixed-round game-over work.

## Current implementation status

- Cargo binary crate exists and builds at fixed EGA-like target resolution 640x350.
- Module skeletons exist: `main`, `config`, `entities`, `game`, `city`, `physics`, `render`, `input`, and `audio`.
- Gameplay window renders QBasic-inspired instructions over the scene; Esc quits.
- City skyline generation/rendering is implemented with buildings, windows, wind generation, and wind arrow.
- Core entities include `Point`, `Bounds`, `ArmPose`, `SunMood`, `ShotResult`, `PlayerCommand::SubmitShot`, `Player`, `Gorilla`, and `Sun`.
- Game state generates a city, places gorillas, creates a sun, tracks current turn, scores, completed rounds, a fixed `round_limit`, active shot animation, generic building-hit shot explosions, gorilla hit explosions, victory dance, and `ScreenState::{Playing, GameOver}`.
- Local input collects per-turn angle and velocity text using digits, one decimal point, Enter/Tab field movement, Backspace/Delete editing, and a 0..=360 validation cap before submitting `PlayerCommand::SubmitShot`.
- Renderer draws the skyline, wind arrow, sun, simple vector gorillas, player name header, `left_score >Score< right_score`, per-turn shot prompts, rotating banana/impact marker, QBasic-inspired building-hit `DoExplosion`, QBasic-inspired gorilla explosion, and a final `GAME OVER!` score screen.
- Rendering-independent banana trajectory helpers cover player-2 angle mirroring, EGA spawn offsets, QBasic projectile coordinates, rotation frames, off-screen detection, and geometry-based shot resolution.
- Shot animation advances on the existing frame loop, uses pure `resolve_shot` output, briefly holds an impact marker, shocks/resets the sun, alternates turns, starts a transient generic explosion for building hits, or applies QBasic-style scoring after a gorilla explosion and victory dance before either a fresh city round or the final game-over screen.
- Unit tests cover city bounds/window bounds, wind range, gorilla rooftop placement, trajectory formula, wind acceleration, spawn offsets, player-2 angle transformation, off-screen simulation stop behavior, shot collision outcomes, active shot creation, sun shock/reset animation behavior, shot-input numeric validation/command creation, score mapping, delayed score/victory/round reset flow, fixed-round game-over flow, generic shot-explosion mapping/input blocking, gorilla-explosion mapping/input blocking, and victory-dance pose toggling/input blocking.

## Active decisions and constraints

- Immediate scope is a faithful local two-player port; do not implement networking yet.
- Keep rules/state transitions independent from rendering and local input to avoid future network-play refactors.
- Player actions flow as explicit commands/events (`PlayerCommand::SubmitShot`) from local input into game state.
- Rendering should remain a view of game state, not the owner of rules.
- `minifb` is the selected windowed 2D backend for now because `macroquad 0.4.15` did not compile with the available Cargo/Rust dependency environment.
- Scoring preserves QBasic `UpdateScores`: opponent scores on self-hit, otherwise thrower scores on opponent hit.
- Round/game-over policy intentionally preserves original `PlayGame`: the prompt says "Play to how many total points", but the implementation treats it as a fixed number of rounds. Default `round_limit` is currently 3 until setup input is implemented.
- Current round flow: building hits play a short generic explosion and then allow the next turn in the same city; gorilla/self hits block input while the victim gorilla explosion plays, then update score, run winner victory dance, and start a fresh city round unless the fixed round limit has been reached, in which case `ScreenState::GameOver` blocks shot input and renders the final score.
- QBasic city slope quirk: Rust intentionally maps slope value `6` to `InvertedV` to preserve apparent design intent rather than duplicating the unreachable `CASE 4` behavior.
- Collision strategy: use geometry-based collision against buildings, gorilla bounds, sun radius, and screen/bottom thresholds. QBasic used pixel-color collision with `POINT`, but geometry is simpler and keeps core logic rendering-independent.
- `tasks/state.md` should stay compact. Use Git history for detailed chronological logs.

## Latest completed task

- Selected task: add scoring/game-over flow by preserving the original fixed-round `NumGames` behavior and showing final scores.
- Changed files: `src/game.rs`, `src/render.rs`, `tasks/task.md`, `tasks/state.md`.
- Summary:
  - Replaced the single intro screen state with `ScreenState::{Playing, GameOver}`.
  - Added `round_limit` and `completed_rounds` to `GameState`; scoring gorilla hits now increments completed rounds after the gorilla explosion.
  - After victory dance, the game now starts a fresh city only if the fixed round limit has not been reached; otherwise it enters game-over state.
  - Game-over state blocks shot input and updates, and renderer shows a QBasic-style final score screen.
  - Added a unit test covering one-round game-over transition, final-score text data, and input blocking.
- Verification:
  - `cargo fmt` passed.
  - `cargo test` passed: 25 tests.
  - `cargo check` passed.
- Commit: `Add fixed-round game over screen` (created after this state update).

## Known issues / deferred work

- Tracked `target/` build artifacts exist from earlier repository history and become dirty after Cargo commands; avoid staging them for implementation commits.
- Full setup screens for player names, fixed round count, gravity, and menu choices are not implemented; `round_limit` defaults to 3 and names/default gravity are still hard-coded through current constructors.
- Input is currently gameplay-only and uses minifb key events; shifted punctuation/numpad edge cases may need polish.
- Banana animation, explosions, and victory dance are frame-advanced rather than time-accumulated, so speed still needs tuning.
- Building explosions are visual only; they do not yet remove/damage city geometry.
- The current gameplay text overlays the generated scene and prompts; this should be reorganized when proper setup/menu/game screens are added.
- Game-over screen currently instructs Esc to quit; original waits for any key and returns to setup, which remains deferred.

## Next recommended task

- Implement setup/menu flow for player names, fixed round count, and gravity, including keypress-to-continue/menu handling.
