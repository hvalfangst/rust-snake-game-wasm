use rodio::Sink;
use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let head = &mut game_state.player.body[0];
        let sprite_height = game_state.sprites.body[0].height as f32;

        // Wrap vertically with sprite height consideration
        if head.y < LOWER_BOUND_Y {
            head.y = UPPER_BOUND_Y - sprite_height; // Appear at top
        } else if head.y > UPPER_BOUND_Y {
            head.y = LOWER_BOUND_Y + sprite_height; // Appear at bottom
        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let head = &mut game_state.player.body[0];
        let sprite_width = game_state.sprites.body[0].width as f32;

        // Wrap horizontally with sprite width consideration
        if head.x < LOWER_BOUND_X {
            head.x = UPPER_BOUND_X - sprite_width; // Appear at right
        } else if head.x > UPPER_BOUND_X {
            head.x = LOWER_BOUND_X + sprite_width; // Appear at left
        }
    }
}