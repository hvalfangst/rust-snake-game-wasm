use std::time::Instant;
use crate::graphics::sprites::SpriteMaps;
use minifb::{Key, Window};


pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

pub struct Snake {
    pub direction: Direction, // Defaults to Right
    pub last_key: Option<Key>, // Last key pressed by the player, defaults to None
    pub body: Vec<(Vector2D)>, // Body segments of the player
}

impl Snake {
    pub fn new(x: f32, y: f32) -> Self {
        Snake {
            direction: Direction::Right, // Default direction is Right
            last_key: None, // No key pressed initially
            body: vec![
                Vector2D { x, y },
                Vector2D { x: x - 1.0, y },
                Vector2D { x: x - 2.0, y },
                Vector2D { x: x - 2.0, y },
            ], // Initialize with three segments
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Direction {
    Right,
    Left,
    Up,
    Down,
}

pub struct Food {
    pub position: Vector2D, // Position of the food
    pub is_active: bool, // Whether the food is active or not
    pub current_sprite_frame_index: usize, // Index of the current sprite for the food
    pub last_sprite_frame_index_update_time: Instant, // Track the last update time
}

pub struct GameState<'a> {
    pub player: Snake, // Player object
    pub sprites: SpriteMaps, // Sprite maps
    pub window_buffer: &'a mut Vec<u32>, // Window buffer
    pub window_width: usize, // Width of the window
    pub window_height: usize, // Height of the window
    pub window: &'a mut Window, // Window object
    pub scaled_buffer: &'a mut Vec<u32>, // Scaled buffer
    pub food: Food, // Food object
}