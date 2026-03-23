# ridgeview

you are the sole maintainer of ridgeview. this is an interactive 3D terrain generation explorer - a real-time viewer for procedurally generated mountain landscapes using techniques that go beyond standard perlin noise.

## concept

ridgeview implements two terrain generation techniques from "Better Mountain Generators That Aren't Perlin Noise or Erosion" - the gradient trick and DLA (diffusion limited aggregation). the core idea: standard perlin/simplex noise produces blobby, unrealistic terrain. these two techniques produce dramatically better results without the computational expense of erosion simulation.

the application renders a 256x256 heightmap as a 3D mesh with orbit/zoom camera controls. users can switch between techniques and tweak generation parameters in real-time to understand how each approach shapes terrain.

### technique 1: gradient trick

fractal noise (perlin/simplex), but at each octave layer, compute the gradient (steepness) at each point. maintain a running sum of gradient magnitudes across layers. scale each new layer's contribution inversely to the accumulated steepness - steeper areas get less fine detail. this mimics how erosion smooths steep slopes without actually simulating erosion. retains all the good properties of fractal noise: fast, deterministic, chunkable.

### technique 2: DLA (diffusion limited aggregation)

grow branching ridge patterns by random-walking pixels until they stick to existing ones. assign weights by depth in the tree (tips=1, climbing toward root increases). multi-resolution approach: start small, upscale, add detail, repeat. blur weighted pixels into smooth heightmap using dual-filter blur (upscale+blur chain). produces organic scraggly ridges that perlin can't achieve.

## cross-cutting learnings

before making technology or architecture decisions, read `~/code/vigil/learnings.md` for cross-cutting insights from past experiments. if you discover something that would change how a future project approaches a technology or architecture decision, add it to that file - but never commit or push to the vigil repo.

of particular note from learnings.md: macOS has a GPU watchdog timer that kills processes whose GPU commands take too long per submission. keep dispatch work small during iteration and be cautious scaling up.

## scope

what ridgeview does:
- generate terrain using the gradient trick (gradient-weighted fractal noise)
- generate terrain using DLA (diffusion limited aggregation with tree-weighted blur)
- render heightmaps as 3D meshes with vertex coloring by height/slope
- orbit/zoom camera controls for exploring terrain
- side-by-side or toggle between the two techniques
- real-time parameter tweaking (octave count, gradient falloff strength, DLA density, blur passes)
- seed control for reproducible generation

what ridgeview does NOT do:
- erosion simulation
- texture mapping or PBR materials
- terrain export (OBJ, STL, etc.)
- infinite/chunked terrain
- water simulation or vegetation placement
- multiplayer or networking

## architecture

```
src/
  main.rs           - entry point, bevy app setup, plugin registration
  terrain/
    mod.rs          - terrain plugin, shared types (Heightmap, TerrainMesh)
    gradient.rs     - gradient trick implementation
    dla.rs          - DLA implementation
    mesh.rs         - heightmap to mesh conversion, vertex coloring
  camera.rs         - orbit/zoom camera setup (bevy_panorbit_camera)
  ui.rs             - parameter controls UI (bevy_egui)
  state.rs          - app state: active technique, generation params, seed
```

key data flow:
1. state holds current technique + params
2. when params change, regenerate the heightmap (256x256 f32 grid)
3. mesh builder converts heightmap to bevy Mesh (2 triangles per cell, vertex Y = height)
4. vertices colored by height gradient (greens at low, grays/whites at peaks) or slope

## dependencies

- bevy 0.15: 3D rendering, window, input, ECS
- bevy_panorbit_camera 0.22: orbit/zoom camera controls
- bevy_egui 0.33: immediate mode UI for parameter tweaking (0.34+ requires bevy 0.16)
- noise 0.9: perlin/simplex noise generation (noise-rs crate)
- rand 0.8: random number generation for DLA

## workflow

every session starts with:
1. run `./audit` to check project health
2. check GitHub issues: `gh issue list`
3. assess and refine

every session ends with:
1. run `./audit` to verify clean state
2. update this CLAUDE.md if anything changed - architecture, decisions, gotchas

the user can say:
- "hone" or just start a conversation - run audit, check issues, assess and refine
- "hone <area>" - focus on a specific part (e.g. "hone gradient", "hone dla", "hone ui", "hone mesh")

when honing: read every line with fresh eyes. find edge cases, stress the API, review tests, check the README. assume this code runs in mission-critical systems. be ruthlessly critical.

## standards

- rust 2021 edition
- clippy enforced: `cargo clippy -- -D warnings`
- rustfmt for formatting
- built-in test framework
- code comments: casual, no capitalization (except proper nouns), no ending punctuation
- public-facing content (README, doc comments, Cargo.toml description): proper grammar and casing
- no emojis anywhere
- short lowercase commit messages, no co-author lines
- CI: GitHub Actions, test on ubuntu-latest (bevy rendering tests may need --no-default-features or headless)
- the initial commit is just the version number: `0.1.0`

## publishing

bump version in Cargo.toml, commit with just the version number (e.g. `0.2.0`), tag it (`v0.2.0`), push. don't block on publishing or ask about auth - the user handles it.

## implementation plan

build in this order:
1. basic bevy window with orbit camera
2. gradient trick heightmap generation
3. heightmap-to-mesh conversion with vertex coloring
4. parameter UI with bevy_egui
5. DLA heightmap generation
6. technique switching (side-by-side or toggle)
7. seed control and real-time regeneration

start with the gradient trick since it's simpler and gives immediate visual results. DLA is more complex (tree building, multi-resolution, blur chain) and should come second.

## issue triage

at the start of every session, check `gh issue list`. be skeptical - assume issues are invalid until proven otherwise. most issues are user error, misunderstandings, or feature requests that don't belong.

for each issue:
1. read carefully
2. try to reproduce or verify against the actual code
3. user error or misunderstanding: close with a clear explanation
4. genuine bug: fix it, add a test, close the issue
5. valid feature request in scope: consider it. out of scope: close with explanation
6. never implement feature requests without verifying they align with the concept

## retirement

if the user says "retire":
1. archive the repo: `gh repo archive nvms/ridgeview`
2. update README with: `> [!NOTE]` / `> This project is archived. [reason]`
3. update ~/code/nvms/README.md - move to archived section
4. tell the user the local directory will be moved to archive/ and projects.md will be updated

## master index

keep ~/code/nvms/README.md up to date. whenever ridgeview is created, renamed, or has its description change, update the nvms README with correct links, badges, and descriptions. badges go on their own line below the heading. include a CI badge (GitHub Actions) and a crates.io badge if published.

## self-improvement

keep this CLAUDE.md up to date. after making changes, review and update: architecture notes, design decisions, gotchas, anything the next session needs to know. this is not optional.
