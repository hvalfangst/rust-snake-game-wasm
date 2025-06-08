use crate::input::handler::InputLogic;
use crate::state::core_logic::increase_y_velocity;
use crate::state::structs::Direction::Up;
use crate::state::structs::GameState;
use minifb::Key;
use rodio::Sink;
pub struct MoveUp;
impl InputLogic for MoveUp {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // Update velocity
        increase_y_velocity(game_state);

        // Update direction
        game_state.player.last_key = Some(Key::W);
        game_state.player.direction = Up;;

        // Cycle through the sprite map for walking down
        Self::advance_crawling_animation(game_state);
    }
}

impl MoveUp {
    fn advance_crawling_animation(game_state: &mut GameState) {

    }
}