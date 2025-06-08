use crate::input::handler::InputLogic;
use crate::state::core_logic::{increase_x_velocity};
use crate::state::structs::Direction::Right;
use crate::state::structs::GameState;
use minifb::Key;
use rodio::Sink;

pub struct MoveRight;

impl InputLogic for MoveRight {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity
        increase_x_velocity(game_state);

        // Update direction
        game_state.player.last_key = Some(Key::D);
        game_state.player.direction = Right;

        // Cycle through the sprite map for walking right
        Self::advance_crawling_animation(game_state);
    }
}

impl MoveRight {
    fn advance_crawling_animation(game_state: &mut GameState) {

    }
}