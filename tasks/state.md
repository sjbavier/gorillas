# Current State: Rust Gorillas Port

## Last updated

- 2026-07-17 22:01 EDT

## Repository state observed

- Working directory: `/home/b4v1n4t0r/rust_projects/gorillas`
- Source reference present: `GORILLA.BAS`
- `tasks/` directory exists.
- No Rust/Cargo project files were observed during this planning pass.
- No Rust implementation has been started by this task.

## Work completed in this pass

- Reviewed the original `GORILLA.BAS` structure and major routines.
- Created `tasks/task.md` with a detailed task list for converting the QBasic game to Rust.
- Created this `tasks/state.md` file to track implementation state.

## Current implementation status

- **Planning:** Started / initial planning document created.
- **Rust project skeleton:** Not started.
- **Rendering backend decision:** Not decided.
- **Core game structs:** Not started.
- **City generation:** Not started.
- **Gorilla rendering/placement:** Not started.
- **Sun rendering/interaction:** Not started.
- **Banana physics:** Not started.
- **Collision detection:** Not started.
- **Input screens:** Not started.
- **Scoring/game loop:** Not started.
- **Animations/explosions/victory dance:** Not started.
- **Audio:** Not started / scope undecided.
- **Tests:** Not started.

## Important findings from `GORILLA.BAS`

- Main game flow is:
  1. Initialize graphics/configuration.
  2. Show intro.
  3. Get player names, target score/round count, and gravity.
  4. Show gorilla intro/menu.
  5. Play the match.
- Original supports two graphics modes:
  - EGA-like mode 9: 640x350.
  - CGA-like mode 1: 320x200.
- The game relies heavily on QBasic drawing APIs and pixel color collision via `POINT`.
- `MakeCityScape` generates random buildings, windows, and wind.
- `PlaceGorillas` places gorillas on the second or third building from each side.
- `PlotShot` contains the key projectile physics formula using angle, velocity, gravity, and wind.
- `DoSun` changes the sun expression when hit/passed by the banana.
- `UpdateScores` awards the opponent a point on self-hit.
- The prompt says "Play to how many total points", but the original code loops `FOR i = 1 TO NumGames`, so it behaves more like a fixed number of rounds than true play-to-score.
- `ClearGorillas` is declared but no implementation was found in the reviewed file excerpt/full symbol list.

## Recommended next step

Choose the rendering/input backend before writing Rust code. For a close remake, use a windowed 2D backend such as `macroquad` or `minifb`. After that, initialize a Cargo project and implement the Phase 1 skeleton from `tasks/task.md`.

## Notes for future updates

Update this file whenever implementation state changes, especially after:

- Creating the Cargo project.
- Selecting a rendering backend.
- Adding modules.
- Completing each implementation phase.
- Changing design decisions from the task list.

## Update: 2026-07-17 22:03 EDT

### Selected task

- Create a reusable prompt for future repo-agent runs.
- Record the user's backend preference: windowed 2D.

### Changes made

- Added `tasks/repo_agent_prompt.md`.
- The reusable prompt instructs future agents to read `tasks/task.md`, `tasks/state.md`, and relevant `GORILLA.BAS` sections; choose the next unfinished task; use a RALPH-style Read/Analyze/Loop/Prove/Hand-off process; verify before claiming completion; and keep task/state files updated.
- Updated `tasks/task.md` to mark the rendering backend decision as made: use a windowed 2D backend, with `macroquad` as the preferred default unless superseded by a documented implementation decision.

### Verification

- Command: documentation/file update only; no build commands applicable because no Rust/Cargo project exists yet.
- Result: Passed by file creation/update.
- Notes: Future implementation passes should run `cargo fmt`, `cargo check`, and/or `cargo test` once a Cargo project exists.

### Current status

- Planning documents exist.
- Reusable task-runner prompt exists.
- Rendering direction is now windowed 2D, preferably `macroquad`.
- Rust implementation has not started yet.

### Next recommended task

- Initialize the Cargo project and add the selected windowed 2D backend dependency, preferably `macroquad`.

## Update: 2026-07-17 22:05 EDT

### Selected task

- Record future product direction: eventual online/network Player 1 vs Player 2 support.
- Keep the immediate scope as a basic local port of the original QBasic game.

### Changes made

- Updated `tasks/repo_agent_prompt.md` so future task-runner agents keep eventual network play in mind without implementing it yet.
- Updated `tasks/task.md` with architecture guidance for separating core game rules, player commands, rendering, local input, and future networking/session transport.
- Added a future online/network play considerations checklist to `tasks/task.md`.

### Verification

- Command: documentation/file update only; no Rust build commands applicable because no Cargo project exists yet.
- Result: Passed by file update.
- Notes: The network-play requirement is now captured as future design guidance, not current implementation scope.

### Current status

- Basic local port remains the current implementation target.
- Windowed 2D remains selected, with `macroquad` preferred by default.
- Future online/network support should influence architecture boundaries from the start.

### Next recommended task

- Initialize the Cargo project and add `macroquad`, while keeping core game state/rules separate from rendering and local input.
