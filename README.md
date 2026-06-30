# Interactive Vector Field Sandbox

A real-time, high-performance, and visually stunning interactive vector field simulator built in Rust using **Macroquad**, **Egui**, and **Glam**.

This sandbox allows you to visualize and interact with complex 2D vector fields. It simulates thousands of colorful particles flowing through dynamic, time-varying mathematical fields. 

<video src="vector_field_2.mp4" width="100%" controls autoplay loop muted></video>

---

## Core Features

*   **7 Mathematical Field Presets**:
    *   **Perlin Curl Noise**: Generates divergence-free fluid-like currents (vortices and laminar streams).
    *   **Perlin Gradient Noise**: Organic topographic field where particles settle in local extrema.
    *   **Vortex / Hurricane**: A classic rotational flow spiraling outward or inward.
    *   **Electric Dipole**: Visualizes field lines between a positive source and a negative sink.
    *   **Saddle Point**: A hyperbolic flow demonstrating saddle point deflection.
    *   **Trig Flow (Parametric)**: Fully customizable combinations of sine and cosine waves.
    *   **Gravity N-Body Sandbox**: Place, drag, and delete heavy stars to simulate orbital gravitational wells.
*   **Dual Physics Models**:
    *   **Velocity Flow (First-order)**: Particle velocities are directly mapped to the vector field ($v = F$). Particles follow field lines exactly.
    *   **Force Field (Second-order / Inertia)**: The vector field acts as an acceleration force ($a = F / m$). Particles possess inertia, enabling orbits, chaotic loops, and slingshots.
*   **Aesthetics & Visuals**:
    *   Smooth, tapered, alpha-faded particle trails that taper off into the obsidian background.
    *   Optional Neon Glow Core flare around particle heads.
    *   High-fidelity color mappings (Spectral Rainbow, Magma Fire, Deep Ocean, Neon Electric, and Viridis).
    *   Optional arrow grid overlay reflecting field directions and strengths in real-time.
*   **Direct Viewport Interaction**:
    *   **Attract & Repel**: Warps the field under your cursor.
    *   **Clockwise & Counter-Clockwise Vortex**: Twists the field locally.
    *   **Gravity Sandboxing**: Place stars with Left-Click, drag them around, or delete them with Right-Click.

---

## Mathematical Fields

### 1. Perlin Curl Noise (Divergence-Free)
To simulate fluid-like currents that don't aggregate into sinks, we compute the curl of a 2D scalar Fractal Brownian Motion field $\psi(x, y, t)$:
$$\mathbf{V}(x, y) = \left( \frac{\partial \psi}{\partial y}, -\frac{\partial \psi}{\partial x} \right)$$
We approximate these partial derivatives numerically via central differences:
$$\frac{\partial \psi}{\partial x} \approx \frac{\psi(x + \epsilon, y) - \psi(x - \epsilon, y)}{2\epsilon}$$

### 2. Electric Dipole
Calculates forces acting from a source at $\mathbf{p}_{\text{src}}$ and a sink at $\mathbf{p}_{\text{snk}}$:
$$\mathbf{V}(\mathbf{x}) = \frac{\mathbf{x} - \mathbf{p}_{\text{src}}}{\|\mathbf{x} - \mathbf{p}_{\text{src}}\|^3 + \delta} - \frac{\mathbf{x} - \mathbf{p}_{\text{snk}}}{\|\mathbf{x} - \mathbf{p}_{\text{snk}}\|^3 + \delta}$$
*(where $\delta$ is a softening factor to prevent singularities).*

### 3. Custom Trig Flow
Controlled via 8 independent coefficient sliders in the GUI:
$$V_x = A \sin(B \cdot y + t) + C \cos(D \cdot x - t)$$
$$V_y = E \sin(F \cdot x - t) + G \cos(H \cdot y + t)$$

---

## Installation & Running

Ensure you have Rust and Cargo installed.

1.  Navigate to the project directory:
    ```bash
    cd vector-field
    ```
2.  Build and run the project:
    ```bash
    cargo run --release
    ```

*Note: Running with the `--release` flag is highly recommended to compile code optimizations, allowing you to run 10,000+ particles with trails smoothly at 60+ FPS.*

---

## Controls & Sandbox Usage

*   **Interactive Panel (egui)**: Tweak presets, particle counts (from 100 to 9,000), trail lengths, neon glows, particle sizes, damping rates, and color palettes.
*   **Viewport Interaction**:
    *   **Left-Click + Drag**: Apply the active mouse tool (Attract, Repel, Vortex) to distort the field lines.
    *   **Gravity Mode (Sandbox)**:
        *   *Place Star*: Select "Add Gravity Body" mouse tool and Left-Click.
        *   *Move Star*: Left-Click and drag any existing star.
        *   *Delete Star*: Right-Click on a star to remove it.
        *   *Clear*: Use the "Clear Stars" button in the menu.
# vector-field
# vector-field
