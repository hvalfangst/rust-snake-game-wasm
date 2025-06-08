use crate::input::handler::InputLogic;
use crate::state::core_logic::{increase_x_velocity};
use crate::state::structs::Direction::Left;
use crate::state::structs::GameState;
use minifb::Key;
use rodio::Sink;

pub struct MoveLeft;
impl InputLogic for MoveLeft {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {

        // Update velocity
        increase_x_velocity(game_state);

        // Update direction
        game_state.player.last_key = Some(Key::A);
        game_state.player.direction = Left;



        // Cycle through the sprite map for walking left
        Self::advance_crawling_animation(game_state);
    }
}

impl MoveLeft {
    fn advance_crawling_animation(game_state: &mut GameState) {

        }
    }