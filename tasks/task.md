# Task List: Convert QBasic `GORILLA.BAS` to Rust

## Source reviewed

- Original file: `/home/b4v1n4t0r/rust_projects/gorillas/GORILLA.BAS`
- Size: ~2300 lines
- Game: IBM/QBasic Gorillas, a two-player artillery game where players throw bananas over a randomized city skyline while accounting for angle, velocity, gravity, and wind.

## High-level conversion goals

- Recreate the classic Gorillas gameplay in Rust.
- Preserve the original game feel where practical: skyline generation, gorilla placement, banana trajectory, wind, gravity, scoring, intro/game-over screens, sun reaction, explosions, and victory dance.
- Replace QBasic-specific graphics, sound, keyboard polling, palette, and timing APIs with Rust equivalents.

## Repository state and Git workflow

To keep agent handoffs efficient as the project grows:

- `tasks/state.md` should be a compact current-state snapshot, not an append-only log.
- At the end of each verified implementation pass, rewrite/compact `tasks/state.md` to preserve only currently relevant information:
  - latest status,
  - active decisions/constraints,
  - latest completed task summary,
  - verification status,
  - next recommended task.
- Use Git history for detailed chronological logs.
- After a task is complete and verification passes, stage only related files and create a Git commit.
- Put detailed log-style handoff information in the commit message body.
- Future agents may inspect recent history with `git log --oneline -5` and `git show` when more context is needed.
- If a commit cannot be made, document the reason in `tasks/state.md` and the final response.

Tasks:

- [x] Document compact `tasks/state.md` policy.
- [x] Document Git commit handoff policy.
- [x] Compact the existing `tasks/state.md` into the new snapshot format.
- [ ] Ensure future completed implementation passes are committed after successful verification.

## Recommended Rust architecture

Design note for future online play:

- For the initial port, implement local two-player gameplay only.
- Keep the architecture ready for a future networked Player 1 vs Player 2 mode by separating:
  - Core rules/state transitions.
  - Player commands/actions.
  - Rendering.
  - Local input.
  - Future networking/session transport.
- Avoid burying gameplay decisions inside rendering or direct keyboard callbacks.

- `main.rs`
  - Program entry point.
  - Initializes configuration, renderer, audio, input, and game loop.
- `game.rs`
  - Overall game state and main gameplay loop.
  - Player data, scores, round setup, turn handling, win conditions.
- `physics.rs`
  - Banana trajectory calculations.
  - Wind and gravity handling.
  - Collision sampling helpers.
- `city.rs`
  - Building model.
  - Random skyline generation.
  - Window placement/colors.
- `entities.rs`
  - Gorilla, banana, sun, explosion, player structs/enums.
- `render.rs`
  - Drawing abstraction for pixels/shapes/sprites/text.
  - Backend-specific rendering implementation.
- `input.rs`
  - Name entry, angle/velocity entry, menu choices.
- `audio.rs`
  - Optional sound effects/music abstraction.
- `config.rs`
  - Screen size, colors, gravity defaults, speed/timing constants.

## Choose a Rust rendering/input backend

Pick one target before implementing rendering:

- **Terminal-first option:** `ratatui`/`crossterm`
  - Easier to keep dependencies simple.
  - Will be an approximation, not pixel-perfect.
- **Windowed 2D option:** `macroquad`, `ggez`, `piston`, `minifb`, or `sdl2`
  - Better for recreating QBasic graphics, simple shapes, animation, and collision sampling.
  - **Selected direction:** windowed 2D, with `macroquad` as the preferred default unless a later implementation pass documents a different choice.

Tasks:

- [x] Create a Cargo project if one does not already exist.
- [x] Decide rendering backend: use a windowed 2D backend; default to `macroquad` unless implementation discovers a better fit.
- [x] Add dependencies for rendering, input, randomness, and optional audio.
- [x] Define target resolution. Original supports:
  - EGA mode: 640x350
  - CGA mode: 320x200
- [x] Prefer one modern fixed resolution first, likely 640x350 or scaled 1280x700.

## Port constants and basic data types

Original QBasic constants/globals to model in Rust:

- `SPEEDCONST = 500`
- `TRUE = -1`, `FALSE = NOT TRUE` should become Rust `bool`.
- `HITSELF = 1`
- Color constants:
  - `BACKATTR`
  - `OBJECTCOLOR`
  - `WINDOWCOLOR`
  - `SUNATTR`
- Sun states:
  - `SUNHAPPY`
  - `SUNSHOCK`
- Gorilla arm states:
  - `RIGHTUP`
  - `LEFTUP`
  - `ARMSDOWN`

Tasks:

- [ ] Replace QBasic global state with Rust structs.
- [x] Define `Point`/`Vec2` equivalents for `XYPoint`.
- [x] Define `GameConfig` for dimensions, gravity, palette/colors, timing.
- [ ] Define enums:
  - [x] `ArmPose::{RightUp, LeftUp, Down}`
  - [x] `SunMood::{Happy, Shocked}`
  - [x] `ShotResult::{Miss, HitPlayer(usize), HitSelf}` or similar.
- [ ] Avoid QBasic-style global mutable arrays except where backend requires buffers.

## Main flow to port

Original main flow:

1. Initialize variables and graphics mode.
2. Show intro.
3. Get player names, number of games/points, and gravity.
4. Show gorilla intro/menu.
5. Play game.
6. Return to input loop.

Tasks:

- [x] Implement startup initialization.
- [x] Implement intro screen with instructions.
- [ ] Implement input collection:
  - [x] Player 1 name, default `Player 1`, max 10 chars.
  - [x] Player 2 name, default `Player 2`, max 10 chars.
  - [x] Play-to score, default `3`.
  - [x] Gravity, default `9.8`.
- [ ] Implement menu choice:
  - [x] View intro animation.
  - [x] Play game.
- [x] Implement game-over screen and final score.
- [x] Decide whether to loop back for another match after game over.

## Rendering tasks

QBasic drawing calls to replace:

- `SCREEN`, `WIDTH`, `COLOR`, `CLS`, `LOCATE`, `PRINT`
- `LINE`, `CIRCLE`, `PAINT`, `PSET`, `POINT`
- `GET`, `PUT`, `XOR`, `PSET` sprite operations
- `PALETTE`

Tasks:

- [ ] Create a renderer interface with primitives:
  - [ ] Clear screen/background.
  - [ ] Draw text at row/column or pixel coordinates.
  - [ ] Draw line.
  - [ ] Draw rectangle outline/fill.
  - [x] Draw circle/arc.
  - [x] Fill circle/region as needed.
  - [ ] Set/get pixel or collision layer value.
- [x] Decide collision strategy:
  - [ ] Pixel-color collision like QBasic `POINT`, or
  - [x] Geometry-based collision against buildings/gorillas/sun.
- [ ] Implement text centering equivalent to `Center`.
- [ ] Implement color palette mapping for background, gorillas, buildings, windows, sun, explosion, banana.
- [ ] Implement screen scaling equivalent to `Scl(n)` if supporting multiple resolutions.

## City skyline generation

Original routine: `MakeCityScape(BCoor())`

Behavior to preserve:

- Random building widths.
- Random building heights.
- Four major slope patterns:
  - Upward slope.
  - Downward slope.
  - V-shaped slope.
  - Inverted-V slope, though note the original `SELECT CASE` has overlapping `CASE 3 TO 5` and `CASE 4`, making the `CASE 4` branch unreachable in QBasic-style order.
- Building bottom around 335 for EGA / 190 for CGA.
- Windows spaced across building faces with random lit/unlit color.
- Wind generated after city generation.
- Wind arrow drawn at bottom of screen.

Tasks:

- [x] Define `Building { x, y, width, height, color, windows }`.
- [x] Port skyline generation math.
- [x] Preserve or intentionally fix the original slope-case quirk; document decision.
- [x] Store buildings for rendering and collision.
- [x] Generate window rectangles.
- [x] Render skyline.
- [x] Generate wind using original rules:
  - [x] Base wind: random 1..10 minus 5.
  - [x] One-third chance to add extra magnitude in same sign direction.
- [x] Render wind arrow.

## Gorilla placement and drawing

Original routines:

- `DrawGorilla(x, y, arms)`
- `PlaceGorillas(BCoor())`
- `ClearGorillas()` is declared but not implemented/used in the provided source.

Behavior to preserve:

- Gorillas are placed on the second or third building from each edge.
- Gorilla sprite has three poses:
  - Arms down.
  - Left arm up.
  - Right arm up.
- Original draws gorilla using lines, rectangles, circles/arcs, then captures sprite buffers with `GET`.

Tasks:

- [x] Define `Gorilla { position, player_index, pose }`.
- [x] Port gorilla placement rules.
- [x] Implement gorilla drawing with primitives, or create static pixel-art/vector sprite assets.
- [x] Implement pose switching for throws and victory dance.
  - [x] Switch the throwing gorilla pose during the throw animation.
  - [x] Implement victory dance pose switching.
- [x] Define gorilla collision bounds/mask.
- [ ] Implement gorilla explosion animation.

## Sun

Original routine: `DoSun(Mouth)`

Behavior to preserve:

- Sun is centered near top of screen.
- Happy sun has smile.
- Shocked sun has open mouth when banana passes through/hits sun area.
- `SunHit` is set during shot plotting and reset after shot.

Tasks:

- [x] Define `Sun { position, radius, mood }`.
- [x] Render sun body, rays, eyes, and mouth.
- [x] Detect when banana enters sun area.
- [x] Temporarily switch sun to shocked state.
- [x] Reset sun after shot if it was hit.

## Banana and shot physics

Original routines:

- `DoShot(PlayerNum, x, y)`
- `PlotShot(StartX, StartY, Angle, Velocity, PlayerNum)`
- `DrawBan(xc, yc, r, bc)`

Original trajectory formula:

```text
angle_radians = angle_degrees / 180 * pi
initial_x_velocity = cos(angle) * velocity
initial_y_velocity = sin(angle) * velocity
x = start_x + initial_x_velocity * t + 0.5 * (wind / 5) * t^2
y = start_y + (-initial_y_velocity * t + 0.5 * gravity * t^2) * (screen_height / 350)
t += 0.1
```

Player 2 transforms input angle with `angle = 180 - angle`.

Tasks:

- [x] Implement angle/velocity prompt for current player.
- [x] Validate numeric input similarly to original `GetNum#`:
  - [x] Digits and one decimal point.
  - [x] Angle/velocity value capped around 360 by original input code.
- [x] Implement player 2 angle transformation.
- [x] Implement banana spawn offset based on throwing player.
- [x] Implement banana rotation frames.
- [x] Implement projectile simulation using the original formula.
- [x] Implement off-screen detection.
- [x] Implement collision with:
  - [x] Buildings/city.
  - [x] Gorillas.
  - [x] Sun.
  - [x] Bottom/screen edges.
- [x] Preserve original low-velocity behavior where velocity `< 2` can result in self-hit logic.
- [x] Trigger generic explosion for city/building hits.
- [x] Trigger gorilla explosion for gorilla hits.
- [x] Trigger score update for gorilla hits.

## Scoring and rounds

Original routines:

- `PlayGame(Player1$, Player2$, NumGames)`
- `UpdateScores(Record(), PlayerNum, Results)`
- `VictoryDance(Player)`

Behavior to preserve:

- Players alternate turns.
- Round ends when a gorilla is hit.
- Winner performs victory dance.
- Total wins are shown as `left_score >Score< right_score`.
- Game ends after the configured number of rounds/points in the original loop. Note: original `NumGames` is prompted as "Play to how many total points" but implemented as a fixed number of rounds.

Tasks:

- [x] Decide whether to preserve fixed-round behavior or implement true play-to-N-points behavior; document decision.
- [x] Implement alternating turns.
- [x] Implement score updates.
- [x] Handle self-hit scoring; opponent should receive point.
- [x] Show current score during gameplay.
- [x] Show final score on game-over screen.

## Input handling

Original routines:

- `GetInputs`
- `GetNum#`
- `SparklePause`

Tasks:

- [x] Implement blocking text input for setup screens.
- [x] Implement per-turn angle and velocity input.
- [x] Implement keypress-to-continue screens.
- [x] Implement menu key handling for `V` and `P`.
- [ ] Consider escape/quit support.

## Timing and animation

Original routines:

- `CalcDelay!`
- `Rest(t#)`

QBasic calibrated delay loops are not appropriate in Rust.

Tasks:

- [ ] Replace busy-wait delay loops with frame timing or async/sleep depending on backend.
- [x] Define a fixed timestep or per-frame delta time for projectile animation.
- [ ] Tune animation speed to feel close to original.
- [ ] Avoid CPU-burning busy loops.

## Audio

Original uses QBasic `PLAY` strings for intro, throw, explosions, and victory dance.

Tasks:

- [ ] Decide whether audio is in scope for the first Rust version.
- [ ] If audio is in scope, choose an audio crate or backend feature.
- [ ] Map original `PLAY` music/effects to simple tones or bundled sound effects.
- [ ] If audio is out of scope, stub `audio.rs` so calls are no-ops.

## Testing and verification

Tasks:

- [ ] Add unit tests for pure logic:
  - [x] Trajectory coordinate calculations.
  - [x] Wind generation range/rules.
  - [x] Score updates including self-hit.
  - [x] Player 2 angle transformation.
  - [x] Building generation does not exceed screen bounds.
- [ ] Add deterministic random seed support for repeatable tests.
- [ ] Add manual test checklist:
  - [ ] Intro appears.
  - [ ] Player setup works with defaults.
  - [ ] Skyline appears and varies between rounds.
  - [ ] Gorillas are placed on rooftops.
  - [ ] Wind arrow appears when wind is nonzero.
  - [x] Banana follows expected arc.
  - [ ] Banana collides with buildings.
  - [x] Banana can hit sun and change expression.
  - [ ] Banana can hit each gorilla.
  - [ ] Scores update correctly.
  - [x] Game-over screen appears.

## Suggested implementation phases

### Phase 1: Project skeleton

- [x] Initialize Cargo project.
- [x] Add chosen graphics/input dependencies.
- [x] Create modules and core structs.
- [x] Show a window/screen with the intro text.

### Phase 2: Static scene

- [x] Render background.
- [x] Generate/render skyline.
- [x] Place/render gorillas.
- [x] Render sun.
- [x] Render score/header text.

### Phase 3: Gameplay loop

- [x] Collect setup input.
- [ ] Alternate turns.
- [x] Collect angle/velocity.
- [x] Simulate/render banana trajectory.
- [x] Detect building collisions.

### Phase 4: Complete interactions

- [x] Detect gorilla collisions.
- [x] Detect sun interactions.
- [ ] Add explosions.
  - [x] Add gorilla-specific explosion before victory dance.
  - [x] Add generic city/building explosion.
- [x] Add victory dance.
- [x] Add scoring and game-over flow.

### Phase 5: Polish

- [ ] Add audio or no-op audio stubs.
- [ ] Tune colors, animation speed, and scaling.
- [ ] Add tests.
- [ ] Document controls and build/run instructions.

## QBasic routine mapping checklist

- [ ] `InitVars` -> `GameConfig::new` / renderer setup.
- [ ] `CalcDelay!` -> remove/replace with frame timing.
- [ ] `Center` -> renderer text helper.
- [x] `Intro` -> intro screen state.
- [x] `SparklePause` -> keypress wait, optional border animation.
- [x] `GetInputs` -> setup input screen.
- [x] `GorillaIntro` -> menu/intro animation state.
- [ ] `SetScreen` -> palette/theme setup.
- [x] `MakeCityScape` -> `city::generate_city`.
- [x] `PlaceGorillas` -> `game::place_gorillas`.
- [x] `DrawGorilla` -> `render::draw_gorilla` or sprite asset generation.
- [x] `DoSun` -> `render::draw_sun` / `Sun` state.
- [x] `DoShot` -> turn input plus shot execution.
- [x] `PlotShot` -> projectile simulation and collision.
- [x] `DrawBan` -> `render::draw_banana`.
- [x] `DoExplosion` -> explosion animation.
- [x] `ExplodeGorilla` -> gorilla-specific explosion and hit result.
- [x] `UpdateScores` -> score logic.
- [x] `VictoryDance` -> winner animation.
- [ ] `Rest` -> sleep/frame delay.
- [ ] `Scl` -> scaling helper if needed.

## Future online/network play considerations

Networking is **not** part of the first basic port, but early architecture should reduce future refactor cost.

- [x] Define player actions as serializable command-like structs/enums when core gameplay begins.
- [x] Keep projectile physics and scoring deterministic and independent from rendering.
- [x] Keep local input code separate from turn resolution.
- [x] Keep game-state updates possible from either local input or future network messages.
- [ ] Consider adding `serde` later for commands/state snapshots if/when networking starts.
- [ ] Defer actual sockets/lobbies/matchmaking until after the local game is playable.

## Open decisions

- [x] Which graphics backend should be used? Windowed 2D; prefer `macroquad` by default.
- [ ] Should this be a pixel-perfect-ish port or a gameplay-faithful remake? Current leaning: gameplay-faithful first, with QBasic-style visuals.
- [x] Should the original CGA/EGA dual-mode scaling be preserved? Decision: target EGA 640x350 first; CGA scaling is deferred.
- [ ] Should `NumGames` preserve original fixed-round behavior or become true play-to-N-points?
- [ ] Is audio required for the first playable Rust version?
- [x] Should collision use pixel buffer color sampling or explicit geometry/masks?
