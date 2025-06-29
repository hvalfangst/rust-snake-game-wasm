use crate::state::core_logic::CoreLogic;
use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::structs::{Direction, Food, GameState, Vector2D};
use rodio::Sink;
use rand::Rng;

pub struct SpawnFood;

impl CoreLogic for SpawnFood {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.food.is_active {
            return;
        }

        game_state.food = Food {
            position: Vector2D {
                x: rand::rng().random_range(LOWER_BOUND_X + 10.0..UPPER_BOUND_X - 10.0),
                y: rand::rng().random_range(LOWER_BOUND_Y + 10.0..UPPER_BOUND_Y - 10.0)
            },
            is_active: true,
            current_sprite_frame_index: 0,
            last_sprite_frame_index_update_time: std::time::Instant::now(),
        };
    }
}

pub struct CheckIfFoodWasEaten;

impl CoreLogic for CheckIfFoodWasEaten {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if !game_state.food.is_active {
            return;
        }

        let head_position = &game_state.player.body[0];
        let food_position = &game_state.food.position;

        if (head_position.x - food_position.x).abs() < 16.0 && (head_position.y - food_position.y).abs() < 16.0 {
            game_state.food.is_active = false;
            game_state.score += 100;

            let tail_position = game_state.player.body.last().unwrap();

            const SPRITE_WIDTH: f32 = 6.0;
            const SPRITE_HEIGHT: f32 = 8.0;

            let new_segment = match game_state.player.direction {
                Direction::Left => Vector2D {
                    x: tail_position.x + SPRITE_WIDTH,
                    y: tail_position.y,
                },
                Direction::Right => Vector2D {
                    x: tail_position.x - SPRITE_WIDTH,
                    y: tail_position.y,
                },
                Direction::Up => Vector2D {
                    x: tail_position.x,
                    y: tail_position.y + SPRITE_HEIGHT,
                },
                Direction::Down => Vector2D {
                    x: tail_position.x,
                    y: tail_position.y - SPRITE_HEIGHT,
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
            return;
        }

        if game_state.food.last_sprite_frame_index_update_time.elapsed().as_millis() >= 500 {
            game_state.food.current_sprite_frame_index = 1 - game_state.food.current_sprite_frame_index;
            game_state.food.last_sprite_frame_index_update_time = std::time::Instant::now();
        }
    }
}