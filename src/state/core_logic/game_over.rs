use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;
use rodio::Sink;

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        // TODO: Implement game over logic
    }
}