pub mod graphics {

    pub const SCALED_WINDOW_WIDTH: usize = 960;
    pub const SCALED_WINDOW_HEIGHT: usize = 540;
    pub const ART_WIDTH: usize = 256;
    pub const ART_HEIGHT: usize = 224;
    pub const SNAKE_BODY_WIDTH: f32 = 6.0;
    pub const SNAKE_BODY_HEIGHT: f32 = 8.0;
}

pub mod physics {
    pub const LOWER_BOUND_X: f32 = 10.0;
    pub const UPPER_BOUND_X: f32 = 256.0;
    pub const LOWER_BOUND_Y: f32 = 10.0;
    pub const UPPER_BOUND_Y: f32 = 224.0;
    pub const COLLISION_TOLERANCE: f32 = 1.0;
}

pub mod text {
    pub const SCORE: &str = "Score: ";
    pub const SELECT_PERK: &str = "Select Perk";
    pub const PERK_NEED_4_SPEED: (&str, &str) = ("Need 4 Speed", "+25% movement speed");
    pub const PERK_HUNGRY_WORM: (&str, &str) = ("Hungry Worm", "2x score from food");
    pub const PERK_CURSE_OF_GLOSSY: (&str, &str) = ("Curse of Glossy", "Death by shiny things");
}

