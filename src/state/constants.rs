pub mod graphics {
    use std::time::Duration;

    pub const FRAME_DURATION: Duration = Duration::from_nanos(16666667); // 16.6666667 ms = 60 FPS
    pub const BACKGROUND_CHANGE_INTERVAL: Duration = Duration::from_secs(1);

    pub const SCALED_WINDOW_WIDTH: usize = 960;
    pub const SCALED_WINDOW_HEIGHT: usize = 540;
    pub const TILE_WIDTH: usize = 16;
    pub const TILE_HEIGHT: usize = 16;
}

pub mod physics {
    pub const GRAVITY: f32 = 0.5;
    pub const JUMP_VELOCITY: f32 = -5.0;
    pub const MAX_VELOCITY: f32 = 2.0;
    pub const ACCELERATION: f32 = 0.1;
    pub const FRICTION: f32 = 0.2;

    pub const LOWER_BOUND_X: f32 = 10.0;
    pub const UPPER_BOUND_X: f32 = 256.0;
    pub const LOWER_BOUND_Y: f32 = 10.0;
    pub const UPPER_BOUND_Y: f32 = 224.0;
}

