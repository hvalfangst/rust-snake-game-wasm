use std::process::exit;
use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;
use rodio::Sink;

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.game_over {
            println!("Game Over! Snake collided with itself or went out of bounds.");
            exit(1); // Exit the game immediately
        }
    }
}