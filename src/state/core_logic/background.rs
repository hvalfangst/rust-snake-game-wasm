use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;

pub struct AlternateBackgroundSpriteFrame;

impl CoreLogic for AlternateBackgroundSpriteFrame {
    fn execute(&self, game_state: &mut GameState, _sink: &mut rodio::Sink) {
        // Alternate the background sprite frame every 700 ms
        if game_state.background_last_sprite_frame_update_time.elapsed().as_millis() >= 700 {
            game_state.background_sprite_frame_index = (game_state.background_sprite_frame_index + 1) % 2   ;
            game_state.background_last_sprite_frame_update_time = std::time::Instant::now();
        }
    }
}