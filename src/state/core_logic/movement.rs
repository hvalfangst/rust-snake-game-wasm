use crate::state::structs::{Direction, GameState};
use rodio::Sink;
use crate::state::core_logic::CoreLogic;

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
                for i in 1..body_size {
                    // Shift each body segment to the position of the previous segment
                    // The segment at index [body_size - i] (starting from the tail) gets the value of the segment at [body_size - i - 1]
                    // This ensures that the "neck" (second segment) gets the position of the "head" (first segment),
                    // and each subsequent segment follows the position of the segment before it.
                    game_state.player.body[body_size - i] = game_state.player.body[body_size - i - 1].clone();
                }
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