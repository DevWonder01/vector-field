# Vector Field Simulation Math

This document details the mathematical equations used in `src/noise.rs` for generating 2D Perlin Noise and Fractal Brownian Motion (FBM).

---

## 1. Linear Interpolation (Lerp)
Linear interpolation calculates a value between a start point $a$ and an end point $b$ based on a normalized weight $t \in [0, 1]$:

$$\text{lerp}(a, b, t) = a + t \cdot (b - a) = (1 - t) \cdot a + t \cdot b$$

---

## 2. Quintic Smoothstep S-Curve
To guarantee smooth transitions (continuous first and second derivatives) across grid boundaries, the fractional distance vector $\mathbf{f}$ is mapped through a quintic polynomial:

$$u(t) = 6t^5 - 15t^4 + 10t^3$$

In vector form for 2D space:

$$\mathbf{u} = \left( 6f_x^5 - 15f_x^4 + 10f_x^3,\; 6f_y^5 - 15f_y^4 + 10f_y^3 \right)$$

---

## 3. Dot Products (Corner Influences)
For a cell corner with integer coordinate $\mathbf{i}_c$, gradient vector $\mathbf{g}_c = \text{hash}(\mathbf{i}_c)$, and target coordinate $\mathbf{p}$, the distance vector is $\mathbf{d}_c = \mathbf{p} - \mathbf{i}_c$. The scalar influence $n_c$ of that corner is:

$$n_c = \mathbf{g}_c \cdot \mathbf{d}_c$$

For the four cell corners:
* **Bottom-Left ($00$):** $n_{00} = \mathbf{g}_{00} \cdot \mathbf{d}_{00}$
* **Bottom-Right ($10$):** $n_{10} = \mathbf{g}_{10} \cdot \mathbf{d}_{10}$
* **Top-Left ($01$):** $n_{01} = \mathbf{g}_{01} \cdot \mathbf{d}_{01}$
* **Top-Right ($11$):** $n_{11} = \mathbf{g}_{11} \cdot \mathbf{d}_{11}$

---

## 4. Bilinear Interpolation (2D Lerp)
Using the smooth weights $u_x$ and $u_y$, the corner influences are blended:

### Step 1: Interpolate horizontally along X
$$ix_0 = \text{lerp}(n_{00}, n_{10}, u_x) = n_{00} + u_x \cdot (n_{10} - n_{00})$$
$$ix_1 = \text{lerp}(n_{01}, n_{11}, u_x) = n_{01} + u_x \cdot (n_{11} - n_{01})$$

### Step 2: Interpolate vertically along Y
$$\text{noise2d}(\mathbf{p}) = \text{lerp}(ix_0, ix_1, u_y) = ix_0 + u_y \cdot (ix_1 - ix_0)$$

### Fully Expanded Representation
$$\text{noise2d}(\mathbf{p}) = (1 - u_x)(1 - u_y)n_{00} \;+\; u_x(1 - u_y)n_{10} \;+\; (1 - u_x)u_y n_{01} \;+\; u_x u_y n_{11}$$

---

## 5. Fractal Brownian Motion (FBM)
To construct natural, multi-scale noise, multiple "octaves" of Perlin noise are added together, doubling the frequency and halving the amplitude at each step:

$$f_{\text{FBM}}(\mathbf{x}) = \sum_{k=0}^{N-1} A_k \cdot \text{noise2d}(f_k \cdot \mathbf{x})$$

Where:
* $N$ is the number of octaves
* $A_k = 0.5^{k+1}$ (Amplitude decays by $0.5$ per octave)
* $f_k = 2^k$ (Frequency doubles per octave)

Expanding this sums to:

$$f_{\text{FBM}}(\mathbf{x}) = 0.5 \cdot \text{noise2d}(\mathbf{x}) \;+\; 0.25 \cdot \text{noise2d}(2\mathbf{x}) \;+\; 0.125 \cdot \text{noise2d}(4\mathbf{x}) \;+\dots$$
