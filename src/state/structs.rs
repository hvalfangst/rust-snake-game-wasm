use crate::graphics::sprites::SpriteMaps;
use minifb::{Key, Window};




pub struct Player {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub direction: Direction,
    pub last_key: Option<Key>,
}

impl Player {
    pub fn new(x: f32, y: f32) -> Self {
        Player {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            direction: Direction::Right, // Default direction
            last_key: None,
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

pub struct GameState<'a> {
    pub player: Player, // Player object
    pub sprites: SpriteMaps, // Sprite maps
    pub window_buffer: &'a mut Vec<u32>, // Window buffer
    pub window_width: usize, // Width of the window
    pub window_height: usize, // Height of the window
    pub window: &'a mut Window, // Window object
    pub scaled_buffer: &'a mut Vec<u32>, // Scaled buffer
    pub art_width: usize, // Width of the art
    pub art_height: usize, // Height of the art
}