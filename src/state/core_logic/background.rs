use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;

pub struct AlternateGlobeSpriteFrame;

impl CoreLogic for AlternateGlobeSpriteFrame {
    fn execute(&self, game_state: &mut GameState, _sink: &mut rodio::Sink) {
        if game_state.background_last_sprite_frame_update_time.elapsed().as_millis() >= 1000 {
            game_state.background_sprite_frame_index = (game_state.background_sprite_frame_index + 1) % 6   ;
            game_state.background_last_sprite_frame_update_time = std::time::Instant::now();
        }
    }
}

pub struct AlternateStarsSpriteFrame;

impl CoreLogic for AlternateStarsSpriteFrame {
    fn execute(&self, game_state: &mut GameState, _sink: &mut rodio::Sink) {
        if game_state.stars_last_sprite_frame_update_time.elapsed().as_millis() >= 250 {
            game_state.stars_sprite_frame_index = (game_state.stars_sprite_frame_index + 1) % 6;
            game_state.stars_last_sprite_frame_update_time = std::time::Instant::now();
        }
    }
}



