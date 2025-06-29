use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::structs::{Direction, Food, GameState, Vector2D};
use rodio::Sink;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::time::Instant;
use rand::Rng;

pub fn execute_core_logic(game_state: &mut GameState, core_logic_operations: &HashMap<String, Rc<RefCell<dyn CoreLogic>>>, sink: &mut Sink) {
    for (_, core_logic_operation) in core_logic_operations.iter() {
        core_logic_operation.borrow().execute(game_state, sink);
    }
}

pub trait CoreLogic {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink);
}

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let player_width = game_state.sprites.body[0].width as f32;

        // Check if the player is out of vertical bounds
        if game_state.player.body[0].y + player_width <= LOWER_BOUND_Y {
            game_state.player.body[0].y = (game_state.player.body[0].y + player_width).abs();
        } else if game_state.player.body[0].y + player_width >= UPPER_BOUND_Y {
            game_state.player.body[0].y = (game_state.player.body[0].y - player_width).abs(); // Adjust y to fit within bounds

        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let player_width = game_state.sprites.body[0].width as f32;

        // Check if the player is out of horizontal bounds
        if game_state.player.body[0].x + player_width <= LOWER_BOUND_X {
            game_state.player.body[0].x = (game_state.player.body[0].x + player_width).abs(); // Adjust x to fit within bounds

        } else if game_state.player.body[0].x + player_width >= UPPER_BOUND_X {
            game_state.player.body[0].x = (game_state.player.body[0].x - player_width).abs(); // Adjust x to fit within bounds

        }
    }
}

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {}
}


pub struct ModifyPosition;

impl CoreLogic for ModifyPosition {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        match game_state.player.direction {
            Direction::Left => {

                game_state.player.body[0].x -= 1.0; // Move the head of the snake left

                // let new_x = game_state.player.ledger.last().unwrap().x - 1.0;
                // game_state.player.ledger.push(Vector2D { x: new_x, y: game_state.player.ledger[0].y });
                // game_state.player.ledger.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

            }
            Direction::Right => {

                game_state.player.body[0].x += 1.0; // Move the head of the snake right

                // let new_x = game_state.player.ledger.last().unwrap().x + 1.0;
                // game_state.player.ledger.push(Vector2D { x: new_x, y: game_state.player.ledger[0].y });
                // game_state.player.ledger.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

            }
            Direction::Up => {
                game_state.player.body[0].y -= 1.0; // Move the head of the snake up
                // let new_y = game_state.player.ledger.last().unwrap().y - 1.0;
                // game_state.player.ledger.push(Vector2D { x: game_state.player.ledger[0].x, y: new_y });
                // game_state.player.ledger.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
            }
            Direction::Down => {
                game_state.player.body[0].y += 1.0; // Move the head of the snake down
                // let new_y = game_state.player.ledger.last().unwrap().y + 1.0;
                // game_state.player.ledger.push(Vector2D { x: game_state.player.ledger[0].x, y: new_y });
                // game_state.player.ledger.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

            }
        }
    }
}

pub struct SpawnFood;

impl CoreLogic for SpawnFood {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.food.is_active {
            return; // Food is already active, no need to spawn new food
        }

        game_state.food = Food {
            position: Vector2D {
                x: rand::rng().random_range(LOWER_BOUND_X + 10.0..UPPER_BOUND_X - 10.0),
                y: rand::rng().random_range(LOWER_BOUND_Y + 10.0..UPPER_BOUND_Y - 10.0)
            },
            is_active: true,
            current_sprite_frame_index: 0, // Start with the first sprite frame
            last_sprite_frame_index_update_time: std::time::Instant::now(), // Initialize the last update time
        };
    }
}

pub struct CheckIfFoodWasEaten;

impl CoreLogic for CheckIfFoodWasEaten {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if !game_state.food.is_active {
            return; // Food is not active, no need to check
        }

        let head_position = &game_state.player.body[0]; // Use body[0] as head
        let food_position = &game_state.food.position;

        // Check if the head of the snake is roughly at the same position as the food
        if (head_position.x - food_position.x).abs() < 16.0 && (head_position.y - food_position.y).abs() < 16.0 {
            game_state.food.is_active = false; // Deactivate food

            // Get the tail position (last body segment)
            let tail_position = game_state.player.body.last().unwrap(); // Safe since we know body has at least head

            // Add new segment behind the tail, offset by sprite dimensions
            const SPRITE_WIDTH: f32 = 6.0;  // Replace with your actual sprite width
            const SPRITE_HEIGHT: f32 = 8.0; // Replace with your actual sprite height

            let new_segment = match game_state.player.direction {
                Direction::Left => Vector2D {
                    x: tail_position.x + SPRITE_WIDTH,  // Behind means to the right
                    y: tail_position.y,
                },
                Direction::Right => Vector2D {
                    x: tail_position.x - SPRITE_WIDTH,  // Behind means to the left
                    y: tail_position.y,
                },
                Direction::Up => Vector2D {
                    x: tail_position.x,
                    y: tail_position.y + SPRITE_HEIGHT, // Behind means below
                },
                Direction::Down => Vector2D {
                    x: tail_position.x,
                    y: tail_position.y - SPRITE_HEIGHT, // Behind means above
                },
            };

            game_state.player.body.push(new_segment);
            println!("Food eaten! Snake grew. New tail at: ({}, {})", new_segment.x, new_segment.y);
        }
    }
}

pub struct AlternateBetweenFoodSpriteFrames;

impl CoreLogic for AlternateBetweenFoodSpriteFrames {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if !game_state.food.is_active {
            return; // Food is not active, no need to alternate sprite frames
        }

        // Alternate the food sprite frame index every 500 ms
        if game_state.food.last_sprite_frame_index_update_time.elapsed().as_millis() >= 500 {
            game_state.food.current_sprite_frame_index = 1 - game_state.food.current_sprite_frame_index;
            game_state.food.last_sprite_frame_index_update_time = std::time::Instant::now();
        }
    }
}


pub struct ModifyCoordinatesOfBodyParts;

impl CoreLogic for ModifyCoordinatesOfBodyParts {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Update timer using stored delta time
        game_state.player.move_timer += game_state.delta_time;

        // Only move when timer exceeds interval
        if game_state.player.move_timer >= game_state.player.move_interval {
            game_state.player.move_timer = 0.0; // Reset timer

            let body_size = game_state.player.body.len();
            if body_size > 0 {
                // Move body segments
                for i in (1..body_size).rev() {
                    game_state.player.body[i] = game_state.player.body[i - 1].clone();
                }

                // Move head by sprite dimensions
                const SPRITE_WIDTH: f32 = 6.0;
                const SPRITE_HEIGHT: f32 = 8.0;

                match game_state.player.direction {
                    Direction::Left => game_state.player.body[0].x -= SPRITE_WIDTH,
                    Direction::Right => game_state.player.body[0].x += SPRITE_WIDTH,
                    Direction::Up => game_state.player.body[0].y -= SPRITE_HEIGHT,
                    Direction::Down => game_state.player.body[0].y += SPRITE_HEIGHT,
                }
            }
        }
    }
}

pub struct UpdateDeltaTime;

impl CoreLogic for UpdateDeltaTime {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let current_time = Instant::now();

        // Calculate delta time
        game_state.delta_time = if let Some(last_time) = game_state.last_frame_time {
            current_time.duration_since(last_time).as_secs_f32()
        } else {
            // First frame, use a default delta time
            1.0 / 60.0 // Assume 60 FPS for first frame
        };

        // Update last frame time for next frame
        game_state.last_frame_time = Some(current_time);
    }
}


pub fn initialize_core_logic_map() -> HashMap<String, Rc<RefCell<dyn CoreLogic>>> {
    let mut logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>> = HashMap::new();

    logic_map.insert("UpdateDeltaTime".to_string(), Rc::new(RefCell::new(UpdateDeltaTime)));
    logic_map.insert("ModifyCoordinatesOfBodyParts".to_string(), Rc::new(RefCell::new(ModifyCoordinatesOfBodyParts)));
    logic_map.insert("VerticalBounds".to_string(), Rc::new(RefCell::new(VerticalBounds)));
    logic_map.insert("HorizontalBounds".to_string(), Rc::new(RefCell::new(HorizontalBounds)));
    // logic_map.insert("ModifyPosition".to_string(), Rc::new(RefCell::new(ModifyPosition)));
    logic_map.insert("SpawnFood".to_string(), Rc::new(RefCell::new(SpawnFood)));
    logic_map.insert("CheckIfFoodWasEaten".to_string(), Rc::new(RefCell::new(CheckIfFoodWasEaten)));
    logic_map.insert("AlternateBetweenFoodSpriteFrames".to_string(), Rc::new(RefCell::new(AlternateBetweenFoodSpriteFrames)));


    logic_map
}
