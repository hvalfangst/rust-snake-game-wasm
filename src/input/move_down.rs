use crate::input::handler::InputLogic;

use crate::state::structs::Direction::Down;
use crate::state::structs::GameState;
use minifb::Key;
use rodio::Sink;
pub struct MoveDown;
impl InputLogic for MoveDown {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Update velocity


        // Update direction
        game_state.player.last_key = Some(Key::S);
        game_state.player.direction = Down;

        // Cycle through the sprite map for walking down
        Self::advance_crawling_animation(game_state);
    }
}

impl MoveDown {
    fn advance_crawling_animation(game_state: &mut GameState) {

    }
}