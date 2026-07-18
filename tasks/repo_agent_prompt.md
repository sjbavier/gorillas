# Reusable Repo Agent Prompt: Rust Gorillas Task Runner

You are working in the repository:

`/home/b4v1n4t0r/rust_projects/gorillas`

Your job is to continue converting the classic QBasic `GORILLA.BAS` game to Rust by using the task/state files in `tasks/` as the source of truth.

## Required files and history

Before doing implementation work, read:

1. `tasks/task.md` — full backlog and conversion plan.
2. `tasks/state.md` — compact current implementation state, latest decisions, and active/next work.
3. `GORILLA.BAS` — original QBasic source, as needed for the task being implemented.

If Rust source files already exist, inspect the relevant modules before editing.

`tasks/state.md` is intentionally **not** a permanent append-only log. Keep it compact and current. If more historical detail is needed, inspect Git history instead, for example:

- `git log --oneline -5`
- `git show --stat <commit>`
- `git show <commit> -- tasks/state.md tasks/task.md src/...`

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
   - Compact/update `tasks/state.md` so it reflects the current repository state rather than accumulating an unbounded log.
   - Update `tasks/task.md` checkboxes only for items actually completed and verified.
   - Review the final diff.
   - After the task is complete, verified, and documented, create a Git commit containing the code/docs changes.
   - Put the detailed handoff/log-style summary in the Git commit message.
   - Summarize the work and next recommended task in the final response.

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
- `tasks/state.md` is compacted/updated with current state.
- `tasks/task.md` is updated if checklist items are completed.
- The final diff has been reviewed.
- The completed and verified work has been committed to Git.
- No known regression is left undocumented.

Never say a task is complete just because code was written. If a Git commit cannot be created, document exactly why in `tasks/state.md` and in the final response.

## State file requirements

Keep `tasks/state.md` compact. It should be a current-state handoff document, not an append-only implementation journal. Prefer rewriting/compacting it at the end of each pass so a future agent can quickly understand where the project stands without reading every previous update.

Recommended structure:

```markdown
# Current State: Rust Gorillas Port

## Snapshot

- Last updated: YYYY-MM-DD HH:MM TZ
- Current commit: short git hash, if available
- Build/test status: latest verified commands

## Current implementation status

- Concise bullets describing what exists now.

## Active decisions and constraints

- Backend choice, architecture boundaries, known QBasic quirks, collision strategy, etc.

## Latest completed task

- Selected task.
- Summary of changes.
- Verification commands and results.
- Commit hash/message, or why no commit was made.

## Known issues / deferred work

- Current limitations and explicitly documented regressions, if any.

## Next recommended task

- One focused next step.
```

Do not keep more than the latest task summary unless older information is still directly relevant. Use Git commits for detailed history. If you discover design decisions, record the current decision in `state.md` and, if appropriate, update `task.md`.

## Coding guidance

- Keep the port idiomatic Rust; avoid translating QBasic globals directly into unsafe/global mutable state.
- Prefer structs and enums for game state.
- Keep pure logic testable where practical.
- Use deterministic seeds for tests involving randomness.
- Use no-op audio stubs until audio is explicitly implemented.
- Avoid busy-wait loops; use backend timing/frame updates.
- Keep collision strategy documented. Pixel-buffer collision is closest to QBasic, geometry/mask collision may be simpler.

## Git handoff requirements

After verification passes:

1. Run `git status --short` and review the changed files.
2. Stage only files that belong to the completed task, for example `git add src/... tasks/task.md tasks/state.md`.
3. Commit the work with a concise subject and a body containing the detailed log-style handoff:
   - selected task,
   - changes made,
   - verification commands/results,
   - current status,
   - next recommended task.
4. Record the commit hash or commit subject in the compact `tasks/state.md`.

Do not commit failing or unverified work. Do not commit unrelated local changes. If the working tree already contains unrelated changes, leave them unstaged and document that in the handoff.

## Expected final response after each run

Respond with:

1. Selected task.
2. Files changed.
3. Verification commands and results.
4. Whether `tasks/state.md` and `tasks/task.md` were updated/compacted.
5. Git commit hash/subject, or why no commit was made.
6. Next recommended task.
