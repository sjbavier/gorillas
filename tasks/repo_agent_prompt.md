# Reusable Repo Agent Prompt: Rust Gorillas Task Runner

You are working in the repository:

`/home/b4v1n4t0r/rust_projects/gorillas`

Your job is to continue converting the classic QBasic `GORILLA.BAS` game to Rust by using the task/state files in `tasks/` as the source of truth.

## Required files

Before doing implementation work, read:

1. `tasks/task.md` — full backlog and conversion plan.
2. `tasks/state.md` — current implementation state and latest decisions.
3. `GORILLA.BAS` — original QBasic source, as needed for the task being implemented.

If Rust source files already exist, inspect the relevant modules before editing.

## Product direction

Use a **windowed 2D graphics option**, not a terminal UI.

Future product direction to keep in mind:

- The game should eventually support online/network play: Player 1 vs Player 2 over a network.
- Do **not** implement networking yet unless explicitly tasked.
- Make early design choices that avoid a painful networking refactor later:
  - Keep core game rules and projectile physics independent from rendering/input.
  - Represent player actions as explicit commands/events, e.g. `SubmitShot { player_id, angle, velocity }`.
  - Keep deterministic game-state transitions where practical.
  - Avoid coupling turn logic directly to local keyboard input.
  - Keep rendering as a view of game state, not the owner of game rules.
  - Plan for a future split between local UI/client and authoritative game/session state.
- For now, prioritize a basic faithful local port of the original game.

Preferred backend direction:

- Use a simple windowed 2D Rust backend suitable for recreating QBasic-style drawing, animation, input, and collision.
- `macroquad` is the preferred default unless the repo already selected another windowed 2D backend.
- Keep the first implementation gameplay-faithful rather than over-engineered.
- Target the original EGA-like resolution first: `640x350`, optionally scaled by the backend/window.

## Operating loop: RALPH

Use a RALPH-style loop for every task. Do not mark a task complete until the loop passes.

RALPH means:

1. **Read**
   - Read `tasks/task.md` and `tasks/state.md`.
   - Inspect existing code and the relevant section of `GORILLA.BAS`.
   - Identify the highest-priority unfinished task that is small enough to complete in one pass.

2. **Analyze**
   - Determine the expected behavior from the QBasic source and current Rust implementation.
   - Identify files that need to change.
   - Decide what can be verified automatically and what needs manual verification.

3. **Loop / Implement**
   - Make the smallest coherent implementation step.
   - Keep changes focused on the selected task.
   - Do not work on unrelated tasks unless required by dependencies.

4. **Prove**
   - Run appropriate checks before claiming completion:
     - `cargo fmt` if a Cargo project exists.
     - `cargo check` if a Cargo project exists.
     - `cargo test` if tests exist or were added.
     - Any relevant targeted command for the changed area.
   - If checks fail, fix the issue and repeat the loop.
   - If a check cannot be run, record why in `tasks/state.md`.

5. **Hand off**
   - Update `tasks/state.md` with:
     - What task was selected.
     - What was changed.
     - What checks were run and their results.
     - Any remaining issues or follow-up tasks.
   - Update `tasks/task.md` checkboxes only for items actually completed and verified.
   - Summarize the work and next recommended task.

## Task selection rules

When choosing the next task:

1. Prefer tasks that unblock the most future work.
2. Prefer earlier implementation phases in `tasks/task.md`.
3. Do not skip foundational work such as creating the Cargo project, choosing backend, or defining core structs.
4. If multiple tasks are available, choose the smallest task that produces a buildable/checkable result.
5. If the current state says a task is in progress, continue that task before starting a new one.

Recommended initial order if no Rust implementation exists:

1. Initialize Cargo project.
2. Select and add `macroquad` as the windowed 2D backend.
3. Create module skeletons.
4. Show a basic window with intro text.
5. Add core config/types.
6. Implement static scene generation/rendering.

## Completion rules

A task is complete only when all are true:

- Code or documentation changes for the selected task are finished.
- The relevant verification command succeeds, or inability to run it is documented.
- `tasks/state.md` is updated.
- `tasks/task.md` is updated if checklist items are completed.
- No known regression is left undocumented.

Never say a task is complete just because code was written.

## State file requirements

Keep `tasks/state.md` current. Every implementation pass must append or update a section with:

```markdown
## Update: YYYY-MM-DD HH:MM TZ

### Selected task

- ...

### Changes made

- ...

### Verification

- Command: `...`
- Result: Passed/Failed/Not run
- Notes: ...

### Current status

- ...

### Next recommended task

- ...
```

If you discover design decisions, record them in `state.md` and, if appropriate, update `task.md`.

## Coding guidance

- Keep the port idiomatic Rust; avoid translating QBasic globals directly into unsafe/global mutable state.
- Prefer structs and enums for game state.
- Keep pure logic testable where practical.
- Use deterministic seeds for tests involving randomness.
- Use no-op audio stubs until audio is explicitly implemented.
- Avoid busy-wait loops; use backend timing/frame updates.
- Keep collision strategy documented. Pixel-buffer collision is closest to QBasic, geometry/mask collision may be simpler.

## Expected final response after each run

Respond with:

1. Selected task.
2. Files changed.
3. Verification commands and results.
4. Whether `tasks/state.md` and `tasks/task.md` were updated.
5. Next recommended task.
