use std::time::Instant;
use crate::graphics::sprites::SpriteMaps;
use minifb::{Key, Window};


#[derive(Debug, Clone, Copy)] // Add Clone and Copy
pub struct Vector2D {
    pub x: f32,
    pub y: f32,
}

pub struct Snake {
    pub direction: Direction, // Defaults to Right
    pub last_key: Option<Key>, // Last key pressed by the player, defaults to None
    pub body: Vec<(Vector2D)>, // Body segments of the player
    pub move_timer: f32,     // Tracks time since last move
    pub move_interval: f32,  // How often to move (in seconds)
}

impl Snake {
    pub fn new(x: f32, y: f32, initial_direction: Direction) -> Self {
        const SPRITE_WIDTH: f32 = 6.0;
        const SPRITE_HEIGHT: f32 = 8.0;

        let body = match initial_direction {
            Direction::Right => vec![
                Vector2D { x, y },
                Vector2D { x: x - SPRITE_WIDTH * 2.0, y },
                Vector2D { x: x - SPRITE_WIDTH * 3.0, y },
            ],
            Direction::Left => vec![
                Vector2D { x, y },
                Vector2D { x: x + SPRITE_WIDTH * 2.0, y },
                Vector2D { x: x + SPRITE_WIDTH * 3.0, y },
            ],
            Direction::Down => vec![
                Vector2D { x, y },
                Vector2D { x, y: y - SPRITE_HEIGHT * 2.0 },
                Vector2D { x, y: y - SPRITE_HEIGHT * 3.0 },
            ],
            Direction::Up => vec![
                Vector2D { x, y },
                Vector2D { x, y: y + SPRITE_HEIGHT * 2.0 },
                Vector2D { x, y: y + SPRITE_HEIGHT * 3.0 },
            ],
        };

        Snake {
            direction: initial_direction,
            last_key: None,
            body,
            move_timer: 0.0,
            move_interval: 0.1, // Move every 0.2 seconds (5 moves per second)
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
    pub delta_time: f32,
    pub last_frame_time: Option<Instant>,
}

