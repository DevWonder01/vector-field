# 2D Vector Field Simulation Walkthrough

We have successfully created a high-performance, interactive **2D Vector Field Simulator** in Rust. The project leverages **macroquad** for hardware-accelerated rendering, **egui-macroquad** for a premium glassmorphic control panel, and **glam** for vectorized mathematics.

---

## Project Architecture

The simulation is split into two core Rust modules and documented in a detailed README:

1.  **[Cargo.toml](file:///home/arcbase/Documents/Github/Simulations/vector-field/Cargo.toml)**: Project configuration and dependencies.
2.  **[src/noise.rs](file:///home/arcbase/Documents/Github/Simulations/vector-field/src/noise.rs)**: Custom-built 2D Perlin noise and Fractal Brownian Motion (FBM) generator.
3.  **[src/main.rs](file:///home/arcbase/Documents/Github/Simulations/vector-field/src/main.rs)**: Main simulation loop, particle physics models, boundary handlers, rendering engine, and the interactive egui user interface.
4.  **[README.md](file:///home/arcbase/Documents/Github/Simulations/vector-field/README.md)**: Mathematical derivations, user manual, and operational guides.

---

## Core Modules and Code Highlights

### 1. Perlin Curl & Gradient Noise
In `src/noise.rs`, we implement 2D Perlin noise using a pseudo-random hashing function that assigns gradients to grid corners:

```rust
// 2D Perlin Noise implementation
pub fn noise2d(p: Vec2) -> f32 {
    let i = p.floor();
    let f = p.fract();
    
    // Quintic interpolation curve: 6t^5 - 15t^4 + 10t^3
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    // ... corner dot products & bilinear interpolation ...
}
```

In `src/main.rs`, we utilize this noise to compute a **divergence-free Curl Noise field** using central finite differences:
```rust
let fbm_y_plus  = noise::fbm2d(Vec2::new(pos.x * scale, (pos.y + eps) * scale) + Vec2::splat(t), octaves);
let fbm_y_minus = noise::fbm2d(Vec2::new(pos.x * scale, (pos.y - eps) * scale) + Vec2::splat(t), octaves);
let fbm_x_plus  = noise::fbm2d(Vec2::new((pos.x + eps) * scale, pos.y * scale) + Vec2::splat(t), octaves);
let fbm_x_minus = noise::fbm2d(Vec2::new((pos.x - eps) * scale, pos.y * scale) + Vec2::splat(t), octaves);

let dx = (fbm_x_plus - fbm_x_minus) / (2.0 * eps);
let dy = (fbm_y_plus - fbm_y_minus) / (2.0 * eps);

// Curl = (dPsi/dy, -dPsi/dx)
Vec2::new(dy, -dx)
```
*Because the field is curl-based, particles loop and wrap endlessly in fluid-like channels without accumulating in clumps, creating a beautiful organic visual flow.*

---

## Premium Visual Elements

To guarantee a premium, wowed experience:
1.  **Glow and Fading Trails**: Particles render with a neon core and a soft outer aura. Their history trails are rendered with a dynamic width gradient (tapering at the tail) and an alpha gradient that smoothly fades into the dark obsidian background.
2.  **Color Palettes**: Predefined colormaps interpolate across multiple color nodes for spectacular gradients:
    *   **Electric Neon**: Cyan $\rightarrow$ Deep Purple $\rightarrow$ Hot Pink $\rightarrow$ White.
    *   **Magma Fire**: Charcoal $\rightarrow$ Deep Violet $\rightarrow$ Red $\rightarrow$ Yellow $\rightarrow$ White.
    *   **Deep Ocean**: Navy $\rightarrow$ Teal $\rightarrow$ Mint $\rightarrow$ Ice White.
3.  **Vector Arrow Grid**: An optional grid of vector arrows that dynamically stretch, point, and change colors based on the localized field magnitude and direction.

---

## Interactive Features

*   **Dual Physics Models**: Toggle between **Velocity Flow** (ideal for mapping stream lines) and **Force Field** (particles orbit, drift, and slingshot with physical inertia and adjustable air friction).
*   **Warp Tools**: Warp field lines under your mouse using Attractor, Repeller, or Vortex tools.
*   **Gravity N-Body Sandbox**: Select this mode to spawn heavy stars. You can:
    *   Left-Click and drag stars to reposition them.
    *   Right-Click to delete stars.
    *   Select the "Add Gravity Body" mouse tool to create new wells on the fly.
    *   Watch particles form Keplerian orbits, hyperbolic escapes, and chaotic three-body patterns.

---

## Running the Project

Navigate to the project folder and run Cargo in release mode:

```bash
cd vector-field
cargo run --release
```
