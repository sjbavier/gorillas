# Gorillas (Rust Port)

A local, windowed Rust port of the classic QBasic `GORILLA.BAS` artillery game.
The current implementation targets the original EGA-style `640x350` playfield and uses `minifb` for 2D window/input handling.

## Build and run

Prerequisites:

- A Rust toolchain with Cargo.
- X11 development/runtime support for the selected `minifb` backend on Linux.

Commands:

```sh
cargo run
```

Useful verification commands for development:

```sh
cargo fmt
cargo test
cargo check
```

## Controls and local flow

General:

- `Esc`: quit the program.
- Any key on the intro or game-over screen: continue.

Setup screen:

- Type each value, then press `Enter` to advance.
- Blank Player 1 / Player 2 names default to `Player 1` / `Player 2`.
- Player names are capped at 10 characters.
- Blank round count defaults to `3`.
- Blank gravity defaults to `9.8`.

Menu:

- `V`: view the intro screen again.
- `P`: play a match.

During play:

- Type the current player's angle, then press `Enter`.
- Type velocity, then press `Enter` to throw.
- The second player's angle is mirrored internally, matching the QBasic original.

## Manual test checklist

Use this checklist after major gameplay/rendering changes. Run from a clean build with `cargo run` unless noted otherwise.

- [ ] Intro appears and advances on keypress.
- [ ] Setup accepts default values by pressing `Enter` through all fields.
- [ ] Setup accepts custom player names, round count, and gravity.
- [ ] `V = View Intro` returns to the intro screen, and `P = Play Game` starts a match.
- [ ] Skyline appears and varies between rounds/matches.
- [ ] Gorillas are placed on rooftops near opposite sides of the skyline.
- [ ] Wind arrow appears when wind is nonzero.
- [ ] Angle/velocity prompts alternate between players.
- [ ] Banana follows the expected artillery arc.
- [ ] Banana collides with buildings and shows a city/building explosion.
- [ ] Banana can pass through/hit the sun area and temporarily change the sun expression.
- [ ] Banana can hit each gorilla and trigger the gorilla-specific explosion.
- [ ] Scores update correctly, including opponent scoring on self-hit.
- [ ] Victory dance runs after a scoring hit.
- [ ] Game-over screen appears after the configured number of rounds and returns to setup on keypress.
- [ ] `Esc` quits cleanly from intro/setup/menu/play/game-over screens.

## Current scope notes

- Gameplay is local two-player only; online/network play is intentionally deferred.
- Audio is currently a no-op placeholder.
- Collision is geometry-based rather than QBasic pixel-color sampling.
- Building explosions are visual only and do not yet remove city geometry.
