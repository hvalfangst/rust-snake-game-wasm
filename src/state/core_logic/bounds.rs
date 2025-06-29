use crate::state::core_logic::CoreLogic;
use crate::state::constants::physics::{LOWER_BOUND_X, LOWER_BOUND_Y, UPPER_BOUND_X, UPPER_BOUND_Y};
use crate::state::structs::GameState;
use rodio::Sink;

pub struct VerticalBounds;

impl CoreLogic for VerticalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let player_width = game_state.sprites.body[0].width as f32;

        // Check if the player is out of vertical bounds
        if game_state.player.body[0].y + player_width <= LOWER_BOUND_Y {
            game_state.player.body[0].y = (game_state.player.body[0].y + player_width).abs();
        } else if game_state.player.body[0].y + player_width >= UPPER_BOUND_Y {
            game_state.player.body[0].y = (game_state.player.body[0].y - player_width).abs();
        }
    }
}

pub struct HorizontalBounds;

impl CoreLogic for HorizontalBounds {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        let player_width = game_state.sprites.body[0].width as f32;

        // Check if the player is out of horizontal bounds
        if game_state.player.body[0].x + player_width <= LOWER_BOUND_X {
            game_state.player.body[0].x = (game_state.player.body[0].x + player_width).abs();
        } else if game_state.player.body[0].x + player_width >= UPPER_BOUND_X {
            game_state.player.body[0].x = (game_state.player.body[0].x - player_width).abs();
        }
    }
}