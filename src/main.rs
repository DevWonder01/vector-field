use macroquad::prelude::*;
use std::collections::VecDeque;

mod noise;

// Vector field presets
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum FieldPreset {
    PerlinCurl,
    PerlinGradient,
    Vortex,
    Dipole,
    Saddle,
    TrigFlow,
    GravityNBody,
}

impl FieldPreset {
    fn name(&self) -> &'static str {
        match self {
            FieldPreset::PerlinCurl => "Perlin Curl (Fluid)",
            FieldPreset::PerlinGradient => "Perlin Gradient",
            FieldPreset::Vortex => "Vortex / Hurricane",
            FieldPreset::Dipole => "Electric Dipole",
            FieldPreset::Saddle => "Saddle Point",
            FieldPreset::TrigFlow => "Trig Flow (Parametric)",
            FieldPreset::GravityNBody => "Gravity N-Body Sandbox",
        }
    }
}

// Particle color mapping choices
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ColorMap {
    Rainbow,
    Magma,
    ElectricPurple,
    Ocean,
    Viridis,
}

impl ColorMap {
    fn name(&self) -> &'static str {
        match self {
            ColorMap::Rainbow => "Spectral Rainbow",
            ColorMap::Magma => "Magma Fire",
            ColorMap::ElectricPurple => "Electric Neon",
            ColorMap::Ocean => "Deep Ocean",
            ColorMap::Viridis => "Viridis Scientific",
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum ColorMode {
    Speed,
    Angle,
    Age,
}

impl ColorMode {
    fn name(&self) -> &'static str {
        match self {
            ColorMode::Speed => "Speed Magnitude",
            ColorMode::Angle => "Velocity Direction (Angle)",
            ColorMode::Age => "Particle Lifetime / Age",
        }
    }
}

// How particles behave when hitting screen boundaries
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum BoundaryBehavior {
    Respawn,
    Wrap,
    Bounce,
}

impl BoundaryBehavior {
    fn name(&self) -> &'static str {
        match self {
            BoundaryBehavior::Respawn => "Respawn Randomly",
            BoundaryBehavior::Wrap => "Wrap Around",
            BoundaryBehavior::Bounce => "Bounce Off Walls",
        }
    }
}

// Particle update model
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum PhysicsMode {
    VelocityFlow, // v = F (first-order)
    ForceField,   // a = F/m (second-order)
}

impl PhysicsMode {
    fn name(&self) -> &'static str {
        match self {
            PhysicsMode::VelocityFlow => "Velocity Flow (Direct)",
            PhysicsMode::ForceField => "Force Field (Inertia)",
        }
    }
}

// Interactive mouse tool
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum MouseMode {
    None,
    Attract,
    Repel,
    VortexCw,
    VortexCcw,
    AddGravityBody,
}

impl MouseMode {
    fn name(&self) -> &'static str {
        match self {
            MouseMode::None => "None (View Only)",
            MouseMode::Attract => "Attractor (Pull)",
            MouseMode::Repel => "Repeller (Push)",
            MouseMode::VortexCw => "Vortex Clockwise",
            MouseMode::VortexCcw => "Vortex Counter-Clockwise",
            MouseMode::AddGravityBody => "Add Gravity Body (Sandbox)",
        }
    }
}

// Gravitational bodies in N-Body mode
struct GravityBody {
    pos: Vec2,
    mass: f32,
}

// Particle representation
struct Particle {
    pos: Vec2,
    vel: Vec2,
    history: VecDeque<Vec2>,
    age: f32,
    lifetime: f32,
}

// Parametric trigonometric sliders
struct TrigParams {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
    e: f32,
    f: f32,
    g: f32,
    h: f32,
}

impl Default for TrigParams {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 2.0,
            c: 0.5,
            d: 1.5,
            e: 1.0,
            f: 2.0,
            g: 0.5,
            h: 1.5,
        }
    }
}

// General configuration state
struct SimConfig {
    preset: FieldPreset,
    color_map: ColorMap,
    color_mode: ColorMode,
    boundary: BoundaryBehavior,
    physics_mode: PhysicsMode,
    mouse_mode: MouseMode,
    
    // Field settings
    field_strength: f32,
    field_scale: f32,
    time_evolution_speed: f32,
    noise_octaves: usize,
    
    // Trig parameters
    trig_params: TrigParams,
    
    // Particle settings
    num_particles: usize,
    particle_speed: f32,
    particle_size: f32,
    trail_length: usize,
    particle_mass: f32,
    damping: f32,
    
    // Visual toggles
    show_arrows: bool,
    arrow_density: usize,
    arrow_scale: f32,
    arrow_thickness: f32,
    show_particles: bool,
    particle_alpha: f32,
    trail_alpha: f32,
    neon_glow: bool,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            preset: FieldPreset::PerlinCurl,
            color_map: ColorMap::ElectricPurple,
            color_mode: ColorMode::Speed,
            boundary: BoundaryBehavior::Respawn,
            physics_mode: PhysicsMode::VelocityFlow,
            mouse_mode: MouseMode::None,
            
            field_strength: 1.2,
            field_scale: 2.5,
            time_evolution_speed: 0.25,
            noise_octaves: 3,
            
            trig_params: TrigParams::default(),
            
            num_particles: 4000,
            particle_speed: 1.2,
            particle_size: 1.8,
            trail_length: 8,
            particle_mass: 1.0,
            damping: 0.03,
            
            show_arrows: true,
            arrow_density: 30,
            arrow_scale: 0.75,
            arrow_thickness: 1.2,
            show_particles: true,
            particle_alpha: 0.9,
            trail_alpha: 0.45,
            neon_glow: true,
        }
    }
}

// Convert HSV representation to a Color
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let c = v * s;
    let hp = h / 60.0;
    let x = c * (1.0 - ((hp % 2.0) - 1.0).abs());
    let (r, g, b) = if hp < 1.0 {
        (c, x, 0.0)
    } else if hp < 2.0 {
        (x, c, 0.0)
    } else if hp < 3.0 {
        (0.0, c, x)
    } else if hp < 4.0 {
        (0.0, x, c)
    } else if hp < 5.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };
    let m = v - c;
    Color::new(r + m, g + m, b + m, 1.0)
}

// Linearly interpolate colors
fn lerp_color(c1: Color, c2: Color, t: f32) -> Color {
    Color::new(
        c1.r + (c2.r - c1.r) * t,
        c1.g + (c2.g - c1.g) * t,
        c1.b + (c2.b - c1.b) * t,
        c1.a + (c2.a - c1.a) * t,
    )
}

// Map a normalized value [0.0, 1.0] to a color palette
fn color_map(val: f32, cmap: ColorMap) -> Color {
    let val = val.clamp(0.0, 1.0);
    match cmap {
        ColorMap::Rainbow => {
            let h = val * 300.0; // Red to Violet
            hsv_to_rgb(h, 0.85, 0.95)
        }
        ColorMap::Magma => {
            if val < 0.25 {
                let t = val / 0.25;
                lerp_color(Color::from_rgba(10, 7, 28, 255), Color::from_rgba(80, 18, 123, 255), t)
            } else if val < 0.5 {
                let t = (val - 0.25) / 0.25;
                lerp_color(Color::from_rgba(80, 18, 123, 255), Color::from_rgba(182, 54, 121, 255), t)
            } else if val < 0.75 {
                let t = (val - 0.5) / 0.25;
                lerp_color(Color::from_rgba(182, 54, 121, 255), Color::from_rgba(251, 136, 97, 255), t)
            } else {
                let t = (val - 0.75) / 0.25;
                lerp_color(Color::from_rgba(251, 136, 97, 255), Color::from_rgba(252, 253, 191, 255), t)
            }
        }
        ColorMap::ElectricPurple => {
            if val < 0.35 {
                let t = val / 0.35;
                lerp_color(Color::from_rgba(0, 240, 255, 255), Color::from_rgba(125, 0, 255, 255), t)
            } else if val < 0.75 {
                let t = (val - 0.35) / 0.4;
                lerp_color(Color::from_rgba(125, 0, 255, 255), Color::from_rgba(255, 0, 160, 255), t)
            } else {
                let t = (val - 0.75) / 0.25;
                lerp_color(Color::from_rgba(255, 0, 160, 255), Color::from_rgba(255, 250, 250, 255), t)
            }
        }
        ColorMap::Ocean => {
            if val < 0.3 {
                let t = val / 0.3;
                lerp_color(Color::from_rgba(2, 16, 44, 255), Color::from_rgba(0, 110, 160, 255), t)
            } else if val < 0.65 {
                let t = (val - 0.3) / 0.35;
                lerp_color(Color::from_rgba(0, 110, 160, 255), Color::from_rgba(0, 210, 190, 255), t)
            } else if val < 0.9 {
                let t = (val - 0.65) / 0.25;
                lerp_color(Color::from_rgba(0, 210, 190, 255), Color::from_rgba(120, 245, 170, 255), t)
            } else {
                let t = (val - 0.9) / 0.1;
                lerp_color(Color::from_rgba(120, 245, 170, 255), Color::from_rgba(230, 255, 245, 255), t)
            }
        }
        ColorMap::Viridis => {
            if val < 0.25 {
                let t = val / 0.25;
                lerp_color(Color::from_rgba(68, 1, 84, 255), Color::from_rgba(59, 82, 139, 255), t)
            } else if val < 0.5 {
                let t = (val - 0.25) / 0.25;
                lerp_color(Color::from_rgba(59, 82, 139, 255), Color::from_rgba(33, 145, 140, 255), t)
            } else if val < 0.75 {
                let t = (val - 0.5) / 0.25;
                lerp_color(Color::from_rgba(33, 145, 140, 255), Color::from_rgba(94, 201, 98, 255), t)
            } else {
                let t = (val - 0.75) / 0.25;
                lerp_color(Color::from_rgba(94, 201, 98, 255), Color::from_rgba(253, 231, 37, 255), t)
            }
        }
    }
}

// Generate an individual particle spread out over the domain
fn create_particle(aspect: f32) -> Particle {
    let x = (macroquad::rand::gen_range(0.0, 1.0) * 2.0 - 1.0) * aspect;
    let y = macroquad::rand::gen_range(0.0, 1.0) * 2.0 - 1.0;
    let max_lifetime = macroquad::rand::gen_range(2.0, 6.0);
    
    let mut history = VecDeque::new();
    history.push_back(Vec2::new(x, y));
    
    Particle {
        pos: Vec2::new(x, y),
        vel: Vec2::ZERO,
        history,
        age: 0.0,
        lifetime: max_lifetime,
    }
}

// Map simulation world space [-aspect, aspect]x[-1, 1] to screen pixels
fn world_to_screen(w: Vec2, aspect: f32, width: f32, height: f32) -> Vec2 {
    let x = (w.x / aspect + 1.0) * 0.5 * width;
    let y = (w.y + 1.0) * 0.5 * height;
    Vec2::new(x, y)
}

// Sample base vector field equations (without mouse interactors)
fn sample_field(pos: Vec2, time: f32, config: &SimConfig, gravity_bodies: &[GravityBody]) -> Vec2 {
    let mut base_vec = match config.preset {
        FieldPreset::PerlinCurl => {
            let eps = 0.01;
            let scale = config.field_scale;
            let t = time * config.time_evolution_speed;
            
            // FBM values at offset positions to compute numerical derivatives
            let fbm_y_plus = noise::fbm2d(Vec2::new(pos.x * scale, (pos.y + eps) * scale) + Vec2::splat(t), config.noise_octaves);
            let fbm_y_minus = noise::fbm2d(Vec2::new(pos.x * scale, (pos.y - eps) * scale) + Vec2::splat(t), config.noise_octaves);
            let fbm_x_plus = noise::fbm2d(Vec2::new((pos.x + eps) * scale, pos.y * scale) + Vec2::splat(t), config.noise_octaves);
            let fbm_x_minus = noise::fbm2d(Vec2::new((pos.x - eps) * scale, pos.y * scale) + Vec2::splat(t), config.noise_octaves);
            
            let dx = (fbm_x_plus - fbm_x_minus) / (2.0 * eps);
            let dy = (fbm_y_plus - fbm_y_minus) / (2.0 * eps);
            
            // Curl noise in 2D: (dPsi/dy, -dPsi/dx)
            Vec2::new(dy, -dx)
        }
        FieldPreset::PerlinGradient => {
            let eps = 0.01;
            let scale = config.field_scale;
            let t = time * config.time_evolution_speed;
            
            let fbm_y_plus = noise::fbm2d(Vec2::new(pos.x * scale, (pos.y + eps) * scale) + Vec2::splat(t), config.noise_octaves);
            let fbm_y_minus = noise::fbm2d(Vec2::new(pos.x * scale, (pos.y - eps) * scale) + Vec2::splat(t), config.noise_octaves);
            let fbm_x_plus = noise::fbm2d(Vec2::new((pos.x + eps) * scale, pos.y * scale) + Vec2::splat(t), config.noise_octaves);
            let fbm_x_minus = noise::fbm2d(Vec2::new((pos.x - eps) * scale, pos.y * scale) + Vec2::splat(t), config.noise_octaves);
            
            let dx = (fbm_x_plus - fbm_x_minus) / (2.0 * eps);
            let dy = (fbm_y_plus - fbm_y_minus) / (2.0 * eps);
            
            // Gradient field: (dPsi/dx, dPsi/dy)
            Vec2::new(dx, dy)
        }
        FieldPreset::Vortex => {
            let len = pos.length();
            if len < 1e-4 {
                Vec2::ZERO
            } else {
                Vec2::new(-pos.y, pos.x) / (len * 0.5 + 0.15)
            }
        }
        FieldPreset::Dipole => {
            // Positive charge (source) at left, negative charge (sink) at right
            let src = Vec2::new(-0.55, 0.0);
            let snk = Vec2::new(0.55, 0.0);
            
            let to_src = pos - src;
            let r_src = to_src.length();
            let force_src = to_src / (r_src * r_src * r_src + 0.03); // 1/r^2 force
            
            let to_snk = pos - snk;
            let r_snk = to_snk.length();
            let force_snk = -to_snk / (r_snk * r_snk * r_snk + 0.03);
            
            force_src + force_snk
        }
        FieldPreset::Saddle => {
            // Saddle flow
            Vec2::new(pos.x, -pos.y)
        }
        FieldPreset::TrigFlow => {
            let tp = &config.trig_params;
            let t = time * config.time_evolution_speed;
            
            let vx = tp.a * (tp.b * pos.y + t).sin() + tp.c * (tp.d * pos.x - t).cos();
            let vy = tp.e * (tp.f * pos.x - t).sin() + tp.g * (tp.h * pos.y + t).cos();
            
            Vec2::new(vx, vy)
        }
        FieldPreset::GravityNBody => {
            let mut force = Vec2::ZERO;
            for body in gravity_bodies {
                let to_body = body.pos - pos;
                let r2 = to_body.length_squared() + 0.02; // soft factor to prevent infinite gravity
                let r = r2.sqrt();
                force += (body.mass * 1.5 * to_body) / (r * r2);
            }
            force
        }
    };
    
    base_vec * config.field_strength
}

// Sample field equations and apply temporary mouse interactive forces
fn sample_field_with_mouse(
    pos: Vec2,
    time: f32,
    config: &SimConfig,
    gravity_bodies: &[GravityBody],
    mouse_down: bool,
    mouse_world: Vec2,
) -> Vec2 {
    let mut field = sample_field(pos, time, config, gravity_bodies);
    
    if mouse_down {
        let to_mouse = mouse_world - pos;
        let dist = to_mouse.length();
        let radius = 0.55; // Radius of influence in world coordinates
        
        if dist < radius {
            let falloff = (1.0 - dist / radius).powi(2);
            let force_strength = 2.8 * config.field_strength;
            
            match config.mouse_mode {
                MouseMode::Attract => {
                    field += to_mouse.normalize_or_zero() * force_strength * falloff;
                }
                MouseMode::Repel => {
                    field -= to_mouse.normalize_or_zero() * force_strength * falloff;
                }
                MouseMode::VortexCw => {
                    let perp = Vec2::new(-to_mouse.y, to_mouse.x).normalize_or_zero();
                    field += perp * force_strength * falloff;
                }
                MouseMode::VortexCcw => {
                    let perp = Vec2::new(to_mouse.y, -to_mouse.x).normalize_or_zero();
                    field += perp * force_strength * falloff;
                }
                _ => {}
            }
        }
    }
    
    field
}

// Get the correct color for a particle depending on selected mapping
fn get_particle_color(p: &Particle, config: &SimConfig) -> Color {
    let u = match config.color_mode {
        ColorMode::Speed => {
            let speed = p.vel.length();
            let scale_factor = if config.physics_mode == PhysicsMode::VelocityFlow {
                0.15 * config.field_strength * config.particle_speed + 0.02
            } else {
                // In force field mode, velocities can build up larger
                1.4
            };
            (speed / scale_factor).clamp(0.0, 1.0)
        }
        ColorMode::Angle => {
            let angle = p.vel.y.atan2(p.vel.x);
            (angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI)
        }
        ColorMode::Age => {
            (p.age / p.lifetime).clamp(0.0, 1.0)
        }
    };
    color_map(u, config.color_map)
}

// Update particle physics, age, boundary rules
fn update_particle(
    p: &mut Particle,
    dt: f32,
    aspect: f32,
    time: f32,
    config: &SimConfig,
    gravity_bodies: &[GravityBody],
    mouse_down: bool,
    mouse_world: Vec2,
) {
    p.age += dt;
    if p.age >= p.lifetime {
        *p = create_particle(aspect);
        return;
    }
    
    // Safety check for Gravity N-Body mode: if particle falls directly on top of a gravity star, respawn it
    if config.preset == FieldPreset::GravityNBody {
        for body in gravity_bodies {
            if (p.pos - body.pos).length_squared() < 0.0016 { // radius of 0.04
                *p = create_particle(aspect);
                return;
            }
        }
    }
    
    // Sample the vector field
    let force = sample_field_with_mouse(p.pos, time, config, gravity_bodies, mouse_down, mouse_world);
    
    match config.physics_mode {
        PhysicsMode::VelocityFlow => {
            p.vel = force * config.particle_speed * 0.14;
            p.pos += p.vel * dt;
        }
        PhysicsMode::ForceField => {
            let accel = force / config.particle_mass;
            p.vel += accel * dt * config.particle_speed * 1.5;
            p.vel *= 1.0 - config.damping; // Apply air resistance/drag
            
            // Clamp speed to prevent particles from launching into hyperspace
            let max_speed = 4.0;
            if p.vel.length_squared() > max_speed * max_speed {
                p.vel = p.vel.normalize() * max_speed;
            }
            p.pos += p.vel * dt;
        }
    }
    
    // Add position to history trail
    p.history.push_back(p.pos);
    if p.history.len() > config.trail_length + 1 {
        p.history.pop_front();
    }
    
    // Boundaries
    let margin = 0.05;
    let out_x = p.pos.x < -aspect - margin || p.pos.x > aspect + margin;
    let out_y = p.pos.y < -1.0 - margin || p.pos.y > 1.0 + margin;
    
    if out_x || out_y {
        match config.boundary {
            BoundaryBehavior::Respawn => {
                *p = create_particle(aspect);
            }
            BoundaryBehavior::Wrap => {
                if p.pos.x < -aspect {
                    p.pos.x += 2.0 * aspect;
                    p.history.clear();
                    p.history.push_back(p.pos);
                } else if p.pos.x > aspect {
                    p.pos.x -= 2.0 * aspect;
                    p.history.clear();
                    p.history.push_back(p.pos);
                }
                
                if p.pos.y < -1.0 {
                    p.pos.y += 2.0;
                    p.history.clear();
                    p.history.push_back(p.pos);
                } else if p.pos.y > 1.0 {
                    p.pos.y -= 2.0;
                    p.history.clear();
                    p.history.push_back(p.pos);
                }
            }
            BoundaryBehavior::Bounce => {
                if p.pos.x < -aspect {
                    p.pos.x = -aspect;
                    p.vel.x = -p.vel.x * 0.85;
                } else if p.pos.x > aspect {
                    p.pos.x = aspect;
                    p.vel.x = -p.vel.x * 0.85;
                }
                
                if p.pos.y < -1.0 {
                    p.pos.y = -1.0;
                    p.vel.y = -p.vel.y * 0.85;
                } else if p.pos.y > 1.0 {
                    p.pos.y = 1.0;
                    p.vel.y = -p.vel.y * 0.85;
                }
            }
        }
    }
}

// Draw a beautiful vector arrow in screen space
fn draw_arrow_custom(start: Vec2, dir: Vec2, length: f32, color: Color, thickness: f32) {
    let end = start + dir * length;
    draw_line(start.x, start.y, end.x, end.y, thickness, color);
    
    // Arrowhead lines
    let head_size = (length * 0.28).clamp(3.0, 9.0);
    let angle = 150.0f32.to_radians();
    
    let cos_a = angle.cos();
    let sin_a = angle.sin();
    
    let head1 = end + Vec2::new(
        dir.x * cos_a - dir.y * sin_a,
        dir.x * sin_a + dir.y * cos_a,
    ) * head_size;
    
    let head2 = end + Vec2::new(
        dir.x * cos_a + dir.y * sin_a,
        -dir.x * sin_a + dir.y * cos_a,
    ) * head_size;
    
    draw_line(end.x, end.y, head1.x, head1.y, thickness, color);
    draw_line(end.x, end.y, head2.x, head2.y, thickness, color);
}

// Generate binary-star configurations
fn default_gravity_bodies() -> Vec<GravityBody> {
    vec![
        GravityBody { pos: Vec2::new(-0.45, 0.0), mass: 1.6 },
        GravityBody { pos: Vec2::new(0.45, 0.0), mass: 1.6 },
    ]
}

#[macroquad::main("Vector Field Sandbox")]
async fn main() {
    // Generate beautiful random particles
    let mut config = SimConfig::default();
    let mut gravity_bodies = default_gravity_bodies();
    
    let mut aspect = screen_width() / screen_height();
    let mut particles: Vec<Particle> = (0..config.num_particles)
        .map(|_| create_particle(aspect))
        .collect();
        
    let mut time = 0.0;
    
    // Dragging state for Sandbox N-Body mode
    let mut dragged_body_idx: Option<usize> = None;
    
    loop {
        // Recalculate screen sizes
        let w_w = screen_width();
        let w_h = screen_height();
        let current_aspect = w_w / w_h;
        
        // If aspect ratio changes, recreate or adjust particle domain positions
        if (current_aspect - aspect).abs() > 0.01 {
            aspect = current_aspect;
            // Respawn particles out of new bounds
            for p in &mut particles {
                if p.pos.x.abs() > aspect {
                    *p = create_particle(aspect);
                }
            }
        }
        
        let dt = get_frame_time().min(0.033); // Cap dt to prevent massive jumps (e.g. during frame stutters)
        time += dt;
        
        // Obtain mouse inputs and map them
        let mut egui_wants_pointer = false;
        egui_macroquad::cfg(|ctx| {
            egui_wants_pointer = ctx.wants_pointer_input() || ctx.wants_keyboard_input();
        });
            
        let mouse_pos = mouse_position();
        let mouse_world = Vec2::new(
            (mouse_pos.0 - w_w / 2.0) / (w_h / 2.0),
            (mouse_pos.1 - w_h / 2.0) / (w_h / 2.0),
        );
        let mouse_down = is_mouse_button_down(MouseButton::Left) && !egui_wants_pointer;
        
        // Handle dragging or placing in Gravity N-Body Sandbox mode
        if config.preset == FieldPreset::GravityNBody && !egui_wants_pointer {
            if is_mouse_button_pressed(MouseButton::Left) {
                // Check if user clicked close to a gravity well to drag
                let mut found_well = None;
                for (idx, body) in gravity_bodies.iter().enumerate() {
                    if (body.pos - mouse_world).length() < 0.12 {
                        found_well = Some(idx);
                        break;
                    }
                }
                
                if let Some(idx) = found_well {
                    dragged_body_idx = Some(idx);
                } else if config.mouse_mode == MouseMode::AddGravityBody {
                    // Place new gravity body
                    gravity_bodies.push(GravityBody {
                        pos: mouse_world,
                        mass: 1.0,
                    });
                }
            }
            
            if is_mouse_button_pressed(MouseButton::Right) {
                // Delete gravity body on right click
                let mut clicked_idx = None;
                for (idx, body) in gravity_bodies.iter().enumerate() {
                    if (body.pos - mouse_world).length() < 0.12 {
                        clicked_idx = Some(idx);
                        break;
                    }
                }
                if let Some(idx) = clicked_idx {
                    gravity_bodies.remove(idx);
                }
            }
            
            // Drag action
            if let Some(idx) = dragged_body_idx {
                if is_mouse_button_down(MouseButton::Left) {
                    if idx < gravity_bodies.len() {
                        gravity_bodies[idx].pos = mouse_world;
                    }
                } else {
                    dragged_body_idx = None;
                }
            }
        } else {
            dragged_body_idx = None;
        }
        
        // Update particles
        for p in &mut particles {
            update_particle(p, dt, aspect, time, &config, &gravity_bodies, mouse_down, mouse_world);
        }
        
        // --- DRAWING ---
        clear_background(Color::from_rgba(7, 7, 10, 255)); // Beautiful deep obsidian
        
        // Draw grid vector arrows
        if config.show_arrows {
            let rows = config.arrow_density;
            let cols = ((rows as f32) * aspect) as usize;
            for r in 0..rows {
                let y_w = -1.0 + 2.0 * (r as f32 + 0.5) / (rows as f32);
                for c in 0..cols {
                    let x_w = -aspect + 2.0 * aspect * (c as f32 + 0.5) / (cols as f32);
                    let pos_w = Vec2::new(x_w, y_w);
                    
                    let vec = sample_field_with_mouse(pos_w, time, &config, &gravity_bodies, mouse_down, mouse_world);
                    let mag = vec.length();
                    if mag < 1e-4 { continue; }
                    
                    let dir = vec / mag;
                    let start_s = world_to_screen(pos_w, aspect, w_w, w_h);
                    
                    let spacing_px = (2.0 * aspect / cols as f32) * (w_h / 2.0);
                    let max_len = spacing_px * config.arrow_scale * 0.7;
                    
                    let norm_mag = (mag / (config.field_strength * 1.5 + 0.01)).clamp(0.0, 1.0);
                    let arrow_len = max_len * (0.2 + 0.8 * norm_mag);
                    
                    let u = match config.color_mode {
                        ColorMode::Speed => norm_mag,
                        ColorMode::Angle => {
                            let angle = dir.y.atan2(dir.x);
                            (angle + std::f32::consts::PI) / (2.0 * std::f32::consts::PI)
                        }
                        ColorMode::Age => norm_mag,
                    };
                    
                    let mut col = color_map(u, config.color_map);
                    col.a = 0.32; // Dim the vector fields to keep them in the background
                    
                    draw_arrow_custom(start_s, dir, arrow_len, col, config.arrow_thickness);
                }
            }
        }
        
        // Draw gravity well hubs in sandbox mode
        if config.preset == FieldPreset::GravityNBody {
            for (i, body) in gravity_bodies.iter().enumerate() {
                let pos_s = world_to_screen(body.pos, aspect, w_w, w_h);
                let pulse = 1.0 + 0.08 * (time * 4.0 + i as f32).sin();
                let base_r = 7.0 + body.mass * 4.5;
                let r = base_r * pulse;
                
                // Draw multiple concentric circles for a nice celestial glowing flare
                draw_circle(pos_s.x, pos_s.y, r * 1.8, Color::new(0.6, 0.2, 1.0, 0.12));
                draw_circle(pos_s.x, pos_s.y, r * 1.2, Color::new(0.8, 0.3, 0.9, 0.25));
                draw_circle(pos_s.x, pos_s.y, r * 0.6, Color::new(1.0, 0.9, 1.0, 0.9));
                
                // Draw labels for the masses
                draw_text(&format!("M={:.1}", body.mass), pos_s.x - 18.0, pos_s.y - r - 6.0, 12.0, GRAY);
            }
        }
        
        // Draw particles and trails
        for p in &particles {
            let p_color = get_particle_color(p, &config);
            
            // Render trailing line strips
            if config.trail_length > 0 && p.history.len() > 1 {
                let len = p.history.len() - 1;
                for i in 0..len {
                    let start_w = p.history[i];
                    let end_w = p.history[i + 1];
                    
                    // Don't draw connections across boundary wraps
                    if (end_w - start_w).length_squared() > 0.45 {
                        continue;
                    }
                    
                    let start_s = world_to_screen(start_w, aspect, w_w, w_h);
                    let end_s = world_to_screen(end_w, aspect, w_w, w_h);
                    
                    let t = i as f32 / len as f32;
                    let mut seg_col = p_color;
                    seg_col.a = config.trail_alpha * t;
                    
                    let th = config.particle_size * (0.35 + 0.65 * t);
                    draw_line(start_s.x, start_s.y, end_s.x, end_s.y, th, seg_col);
                }
            }
            
            // Draw particle lead head
            if config.show_particles {
                let head_s = world_to_screen(p.pos, aspect, w_w, w_h);
                let mut head_col = p_color;
                head_col.a = config.particle_alpha;
                
                if config.neon_glow {
                    let mut glow_col = p_color;
                    glow_col.a = config.particle_alpha * 0.25;
                    draw_circle(head_s.x, head_s.y, config.particle_size * 2.3, glow_col);
                }
                
                draw_circle(head_s.x, head_s.y, config.particle_size, head_col);
            }
        }
        
        // --- DRAW INTERACTOR HUD MOUSE RING ---
        if mouse_down && config.mouse_mode != MouseMode::None && config.mouse_mode != MouseMode::AddGravityBody {
            let pulse = 1.0 + 0.05 * (time * 10.0).sin();
            let radius_px = 0.55 * (w_h / 2.0) * pulse; // 0.55 is field of mouse influence
            let mut hud_color = match config.mouse_mode {
                MouseMode::Attract => Color::new(0.0, 1.0, 0.5, 0.2),
                MouseMode::Repel => Color::new(1.0, 0.2, 0.2, 0.2),
                MouseMode::VortexCw | MouseMode::VortexCcw => Color::new(0.2, 0.6, 1.0, 0.2),
                _ => Color::new(1.0, 1.0, 1.0, 0.1),
            };
            
            // Draw subtle ring indicating interaction area
            draw_circle_lines(mouse_pos.0, mouse_pos.1, radius_px, 1.5, hud_color);
            hud_color.a = 0.04;
            draw_circle(mouse_pos.0, mouse_pos.1, radius_px, hud_color);
        }
        
        // --- UI PANEL ---
        egui_macroquad::ui(|ctx| {
            // Apply beautiful dark styling to egui window
            let mut egui_style = (*ctx.style()).clone();
            egui_style.visuals.window_corner_radius = egui::CornerRadius::same(8);
            egui_style.visuals.widgets.active.corner_radius = egui::CornerRadius::same(4);
            egui_style.visuals.widgets.hovered.corner_radius = egui::CornerRadius::same(4);
            egui_style.visuals.widgets.inactive.corner_radius = egui::CornerRadius::same(4);
            ctx.set_style(egui_style);

            egui::Window::new("Vector Field Sandbox")
                .default_width(330.0)
                .show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("VECTOR FIELD SIMULATION");
                    });
                    ui.separator();
                    
                    // 1. Preset Selector
                    ui.label("Field Equation Preset:");
                    let prev_preset = config.preset;
                    egui::ComboBox::from_id_source("preset_combo")
                        .selected_text(config.preset.name())
                        .show_ui(ui, |ui| {
                            for pr in &[
                                FieldPreset::PerlinCurl,
                                FieldPreset::PerlinGradient,
                                FieldPreset::Vortex,
                                FieldPreset::Dipole,
                                FieldPreset::Saddle,
                                FieldPreset::TrigFlow,
                                FieldPreset::GravityNBody,
                            ] {
                                ui.selectable_value(&mut config.preset, *pr, pr.name());
                            }
                        });
                    
                    // If preset changed to Gravity N-Body, reset gravity body coordinates
                    if config.preset != prev_preset && config.preset == FieldPreset::GravityNBody {
                        gravity_bodies = default_gravity_bodies();
                        config.physics_mode = PhysicsMode::ForceField; // Gravity is best with Force physics
                    }
                    
                    ui.add_space(6.0);
                    
                    // Collapsible Physics Settings
                    ui.collapsing("Simulation & Physics", |ui| {
                        ui.label("Physics Model:");
                        egui::ComboBox::from_id_source("phys_combo")
                            .selected_text(config.physics_mode.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config.physics_mode, PhysicsMode::VelocityFlow, PhysicsMode::VelocityFlow.name());
                                ui.selectable_value(&mut config.physics_mode, PhysicsMode::ForceField, PhysicsMode::ForceField.name());
                            });
                        
                        ui.add(egui::Slider::new(&mut config.field_strength, 0.1..=4.0).text("Field Strength"));
                        
                        if config.preset == FieldPreset::PerlinCurl || config.preset == FieldPreset::PerlinGradient {
                            ui.add(egui::Slider::new(&mut config.field_scale, 0.5..=6.0).text("Noise Spatial Scale"));
                            ui.add(egui::Slider::new(&mut config.noise_octaves, 1..=5).text("Noise Octaves"));
                        }
                        
                        ui.add(egui::Slider::new(&mut config.time_evolution_speed, 0.0..=2.0).text("Evolution Speed"));
                        
                        if config.physics_mode == PhysicsMode::ForceField {
                            ui.add(egui::Slider::new(&mut config.particle_mass, 0.2..=5.0).text("Particle Inertia / Mass"));
                            ui.add(egui::Slider::new(&mut config.damping, 0.0..=0.15).text("Air Friction (Damping)"));
                        }
                    });
                    
                    // Trigonometric sub-menu
                    if config.preset == FieldPreset::TrigFlow {
                        ui.add_space(4.0);
                        ui.collapsing("Trig Equation Coefficients", |ui| {
                            ui.label("Vx = A*sin(B*y + t) + C*cos(D*x - t)");
                            ui.add(egui::Slider::new(&mut config.trig_params.a, -2.5..=2.5).text("A"));
                            ui.add(egui::Slider::new(&mut config.trig_params.b, 0.5..=6.0).text("B"));
                            ui.add(egui::Slider::new(&mut config.trig_params.c, -2.5..=2.5).text("C"));
                            ui.add(egui::Slider::new(&mut config.trig_params.d, 0.5..=6.0).text("D"));
                            
                            ui.label("Vy = E*sin(F*x - t) + G*cos(H*y + t)");
                            ui.add(egui::Slider::new(&mut config.trig_params.e, -2.5..=2.5).text("E"));
                            ui.add(egui::Slider::new(&mut config.trig_params.f, 0.5..=6.0).text("F"));
                            ui.add(egui::Slider::new(&mut config.trig_params.g, -2.5..=2.5).text("G"));
                            ui.add(egui::Slider::new(&mut config.trig_params.h, 0.5..=6.0).text("H"));
                        });
                    }
                    
                    ui.add_space(6.0);
                    
                    // 2. Particle Appearance
                    ui.collapsing("Particle Customization", |ui| {
                        let mut val = config.num_particles;
                        if ui.add(egui::Slider::new(&mut val, 100..=9000).text("Count")).changed() {
                            config.num_particles = val;
                            if particles.len() < val {
                                while particles.len() < val {
                                    particles.push(create_particle(aspect));
                                }
                            } else {
                                particles.truncate(val);
                            }
                        }
                        
                        ui.add(egui::Slider::new(&mut config.particle_speed, 0.1..=4.0).text("Flow Velocity"));
                        ui.add(egui::Slider::new(&mut config.particle_size, 0.5..=4.5).text("Size"));
                        
                        let mut len = config.trail_length;
                        if ui.add(egui::Slider::new(&mut len, 0..=30).text("Trail Length")).changed() {
                            config.trail_length = len;
                        }
                        
                        ui.add(egui::Slider::new(&mut config.trail_alpha, 0.05..=1.0).text("Trail Opacity"));
                        
                        ui.label("Color Theme Palette:");
                        egui::ComboBox::from_id_source("colormap_combo")
                            .selected_text(config.color_map.name())
                            .show_ui(ui, |ui| {
                                for col in &[
                                    ColorMap::ElectricPurple,
                                    ColorMap::Magma,
                                    ColorMap::Ocean,
                                    ColorMap::Rainbow,
                                    ColorMap::Viridis,
                                ] {
                                    ui.selectable_value(&mut config.color_map, *col, col.name());
                                }
                            });
                        
                        ui.label("Color Mapping Variable:");
                        egui::ComboBox::from_id_source("colormode_combo")
                            .selected_text(config.color_mode.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config.color_mode, ColorMode::Speed, ColorMode::Speed.name());
                                ui.selectable_value(&mut config.color_mode, ColorMode::Angle, ColorMode::Angle.name());
                                ui.selectable_value(&mut config.color_mode, ColorMode::Age, ColorMode::Age.name());
                            });
                        
                        ui.checkbox(&mut config.neon_glow, "Neon Core Glow Flare");
                    });
                    
                    ui.add_space(6.0);
                    
                    // 3. Vector Field Layout
                    ui.collapsing("Visual Overlay Grid", |ui| {
                        ui.checkbox(&mut config.show_arrows, "Show Field Arrow Grid");
                        if config.show_arrows {
                            ui.add(egui::Slider::new(&mut config.arrow_density, 15..=50).text("Grid Resolution"));
                            ui.add(egui::Slider::new(&mut config.arrow_scale, 0.2..=1.5).text("Arrow Length Scale"));
                            ui.add(egui::Slider::new(&mut config.arrow_thickness, 0.5..=3.0).text("Arrow Stroke"));
                        }
                        
                        ui.label("Border Boundary Action:");
                        egui::ComboBox::from_id_source("boundary_combo")
                            .selected_text(config.boundary.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config.boundary, BoundaryBehavior::Respawn, BoundaryBehavior::Respawn.name());
                                ui.selectable_value(&mut config.boundary, BoundaryBehavior::Wrap, BoundaryBehavior::Wrap.name());
                                ui.selectable_value(&mut config.boundary, BoundaryBehavior::Bounce, BoundaryBehavior::Bounce.name());
                            });
                    });
                    
                    ui.add_space(6.0);
                    
                    // 4. Interactive Mouse Tool
                    ui.collapsing("Mouse Interaction Tool", |ui| {
                        ui.label("Click and drag mouse in viewport:");
                        egui::ComboBox::from_id_source("mouse_combo")
                            .selected_text(config.mouse_mode.name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(&mut config.mouse_mode, MouseMode::None, MouseMode::None.name());
                                ui.selectable_value(&mut config.mouse_mode, MouseMode::Attract, MouseMode::Attract.name());
                                ui.selectable_value(&mut config.mouse_mode, MouseMode::Repel, MouseMode::Repel.name());
                                ui.selectable_value(&mut config.mouse_mode, MouseMode::VortexCw, MouseMode::VortexCw.name());
                                ui.selectable_value(&mut config.mouse_mode, MouseMode::VortexCcw, MouseMode::VortexCcw.name());
                                if config.preset == FieldPreset::GravityNBody {
                                    ui.selectable_value(&mut config.mouse_mode, MouseMode::AddGravityBody, MouseMode::AddGravityBody.name());
                                }
                            });
                        
                        if config.preset == FieldPreset::GravityNBody {
                            ui.add_space(5.0);
                            ui.horizontal(|ui| {
                                if ui.button("Clear Stars").clicked() {
                                    gravity_bodies.clear();
                                }
                                if ui.button("Add 2 Stars").clicked() {
                                    gravity_bodies = default_gravity_bodies();
                                }
                            });
                        }
                    });
                    
                    ui.add_space(10.0);
                    
                    // 5. Actions / Reset
                    ui.horizontal(|ui| {
                        if ui.button("Reset Settings").clicked() {
                            config = SimConfig::default();
                            gravity_bodies = default_gravity_bodies();
                            particles = (0..config.num_particles)
                                .map(|_| create_particle(aspect))
                                .collect();
                        }
                        if ui.button("Clear Field").clicked() {
                            for p in &mut particles {
                                *p = create_particle(aspect);
                            }
                        }
                    });
                    
                    ui.add_space(8.0);
                    
                    // Short instruction labels
                    ui.separator();
                    ui.small("Tips:");
                    if config.preset == FieldPreset::GravityNBody {
                        ui.small("• Left-Click & Drag to move gravity stars.");
                        ui.small("• Right-Click on a star to delete it.");
                        ui.small("• Set mouse to 'Add Gravity Body' to place new ones.");
                    } else {
                        ui.small("• Choose Attract/Repel mouse tool for physics warping.");
                    }
                    ui.small(&format!("• Performance: {:.0} FPS ({} particles)", get_fps(), config.num_particles));
                });
        });
        
        egui_macroquad::draw();
        
        next_frame().await
    }
}
