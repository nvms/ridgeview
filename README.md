<p align="center">
  <img src="logo.svg" width="200" style="border-radius: 16px;" />
</p>

<h1 align="center">ridgeview</h1>

<p align="center">Interactive 3D terrain generation explorer.</p>

---

Ridgeview implements two mountain generation techniques as an interactive 3D viewer. Orbit, zoom, tweak parameters, and watch terrain regenerate in real-time.

## Techniques

**Gradient trick** - Fractal noise where each octave's contribution is scaled inversely to the accumulated steepness at that point. Steep areas get less fine detail, mimicking erosion without simulating it. Fast, deterministic, and chunkable.

**DLA (diffusion limited aggregation)** - Grows branching ridge structures by random-walking pixels until they stick to existing ones. Tree-weighted blurring produces organic, scraggly ridgelines that perlin noise can't achieve.

## Usage

```sh
cargo run
```

Orbit with left mouse drag, zoom with scroll wheel. The UI panel lets you switch between techniques and adjust generation parameters.

## Parameters

| Parameter | Technique | Effect |
|-----------|-----------|--------|
| Octaves | Gradient | Number of noise layers (more = finer detail) |
| Gradient falloff | Gradient | How aggressively steepness suppresses detail |
| Seed | Both | Deterministic seed for reproducible terrain |
| DLA density | DLA | Number of random walkers (more = denser ridges) |
| Blur passes | DLA | Smoothing iterations on the weighted heightmap |

---

This project is an experiment in AI-maintained open source - autonomously built, tested, and refined by AI with human oversight. Regular audits, thorough test coverage, continuous refinement. The emphasis is on high quality, rigorously tested, production-grade code.
