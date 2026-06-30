use macroquad::math::Vec2;

// Pseudo-random hash mapping Vec2 to Vec2 in range [-1, 1]
fn hash22(p: Vec2) -> Vec2 {
    let x = (p.x * 127.1 + p.y * 311.7).sin() * 43758.5453123;
    let y = (p.x * 269.5 + p.y * 183.3).sin() * 43758.5453123;
    Vec2::new(x.fract(), y.fract()) * 2.0 - Vec2::ONE
}

// 2D Perlin Noise implementation
pub fn noise2d(p: Vec2) -> f32 {
    let i = p.floor();
    let f = p.fract();
    
    // Quintic interpolation curve: 6t^5 - 15t^4 + 10t^3
    let u = f * f * f * (f * (f * 6.0 - 15.0) + 10.0);
    
    // Get gradients at grid corners
    let g00 = hash22(i);
    let g10 = hash22(i + Vec2::new(1.0, 0.0));
    let g01 = hash22(i + Vec2::new(0.0, 1.0));
    let g11 = hash22(i + Vec2::new(1.0, 1.0));
    
    // Vector from corner to point
    let d00 = f;
    let d10 = f - Vec2::new(1.0, 0.0);
    let d01 = f - Vec2::new(0.0, 1.0);
    let d11 = f - Vec2::new(1.0, 1.0);
    
    // Dot products
    let n00 = g00.dot(d00);
    let n10 = g10.dot(d10);
    let n01 = g01.dot(d01);
    let n11 = g11.dot(d11);
    
    // Interpolate
    let ix0 = n00 + (n10 - n00) * u.x;
    let ix1 = n01 + (n11 - n01) * u.x;
    
    ix0 + (ix1 - ix0) * u.y
}

// Fractal Brownian Motion (FBM) noise by summing octaves
pub fn fbm2d(p: Vec2, octaves: usize) -> f32 {
    let mut value = 0.0;
    let mut amplitude = 0.5;
    let mut frequency = 1.0;
    let mut current_p = p;
    for _ in 0..octaves {
        // noise2d returns values in roughly [-0.5, 0.5], map to float
        value += amplitude * noise2d(current_p * frequency);
        frequency *= 2.0;
        amplitude *= 0.5;
    }
    value
}
