use std::time::Instant;
use crate::graphics::sprites::SpriteMaps;
use minifb::{Key, Window};
use crate::state::constants::graphics::{SNAKE_BODY_HEIGHT, SNAKE_BODY_WIDTH};

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
    pub body_sprite_frame_index: usize, // Index of the current sprite frame for the snake
    pub body_sprite_timer: f32, // Timer for sprite animation
    pub body_last_sprite_frame_index_update_time: Instant,
    pub head_sprite_frame_index: usize, // Index of the current sprite frame for the snake's head
    pub head_sprite_timer: f32, // Timer for head sprite animation
    pub head_last_sprite_frame_index_update_time: Instant, // Track the last update time for head sprite
    pub proximity_to_food: bool, // Flag to indicate if the snake is close to food
}

impl Snake {
    pub fn new(x: f32, y: f32, initial_direction: Direction) -> Self {


        let body = match initial_direction {
            Direction::Right => vec![
                Vector2D { x, y },
                Vector2D { x: x - SNAKE_BODY_WIDTH * 2.0, y },
                Vector2D { x: x - SNAKE_BODY_WIDTH * 3.0, y },
            ],
            Direction::Left => vec![
                Vector2D { x, y },
                Vector2D { x: x + SNAKE_BODY_WIDTH * 2.0, y },
                Vector2D { x: x + SNAKE_BODY_WIDTH * 3.0, y },
            ],
            Direction::Down => vec![
                Vector2D { x, y },
                Vector2D { x, y: y - SNAKE_BODY_HEIGHT * 2.0 },
                Vector2D { x, y: y - SNAKE_BODY_HEIGHT * 3.0 },
            ],
            Direction::Up => vec![
                Vector2D { x, y },
                Vector2D { x, y: y + SNAKE_BODY_HEIGHT * 2.0 },
                Vector2D { x, y: y + SNAKE_BODY_HEIGHT * 3.0 },
            ],
        };

        Snake {
            direction: initial_direction,
            last_key: None,
            body,
            move_timer: 0.0,
            move_interval: 0.1, // Move every 0.2 seconds (5 moves per second)
            body_sprite_frame_index: 0, // Start with the first sprite frame
            body_sprite_timer: 0.0, // Initialize sprite timer
            body_last_sprite_frame_index_update_time: Instant::now(), // Initialize sprite frame update time
            head_sprite_frame_index: 0, // Start with the first sprite frame for the head
            head_sprite_timer: 0.0, // Initialize head sprite timer
            head_last_sprite_frame_index_update_time: Instant::now(), // Initialize head sprite frame update time
            proximity_to_food: false, // Initially not close to food
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
    pub sprite_frame_index: usize, // Index of the current sprite for the food
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
    pub game_over: bool,
    pub score: u32, // Score of the game
    pub background_sprite_frame_index: usize,
    pub background_last_sprite_frame_update_time: Instant,
    pub background_offset_x: usize,
    pub stars_last_sprite_frame_update_time: Instant,
    pub stars_sprite_frame_index: usize,
}

impl<'a> GameState<'a> {
    pub fn new(
        player: Snake,
        sprites: SpriteMaps,
        window_buffer: &'a mut Vec<u32>,
        window_width: usize,
        window_height: usize,
        window: &'a mut Window,
        scaled_buffer: &'a mut Vec<u32>,
    ) -> Self {
        GameState {
            player,
            sprites,
            window_buffer,
            window_width,
            window_height,
            window,
            scaled_buffer,
            delta_time: 0.0,
            last_frame_time: None,
            game_over: false,
            score: 0,
            background_sprite_frame_index: 0,
            food: Food {
                position: Vector2D { x: 100.0, y: 100.0 },
                is_active: false,
                sprite_frame_index: 0,
                last_sprite_frame_index_update_time: Instant::now(),
            },
            background_last_sprite_frame_update_time: Instant::now(),
            background_offset_x: 0, // Initialize background offset
            stars_last_sprite_frame_update_time: Instant::now(),
            stars_sprite_frame_index: 0, // Initialize stars sprite frame index
        }
    }

    pub fn restart_level(&mut self) {
        println!("\n * * * * |Restarting Level| * * * * \n");

        self.player = Snake::new(40.0, 150.0, Direction::Right);
        self.food = Food {
            position: Vector2D { x: 100.0, y: 100.0 },
            is_active: false,
            sprite_frame_index: 0,
            last_sprite_frame_index_update_time: Instant::now(),
        };
        self.score = 0;
        self.game_over = false;
        self.background_sprite_frame_index = 0;
        self.background_last_sprite_frame_update_time = Instant::now();
    }
}

