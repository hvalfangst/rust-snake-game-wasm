use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;
use std::time::Instant;

pub struct AlternateBodySpriteFrameIndex;

impl CoreLogic for AlternateBodySpriteFrameIndex {
    fn execute(&self, game_state: &mut GameState, _sink: &mut rodio::Sink) {
        // Alternate the frame index for the snake's body sprites every 700 milliseconds
        if game_state.player.body_last_sprite_frame_index_update_time.elapsed().as_millis() >= 1500 {
            game_state.player.body_sprite_frame_index = (game_state.player.body_sprite_frame_index + 1) % 2;
            game_state.player.body_last_sprite_frame_index_update_time = Instant::now();
        }
    }
}


pub struct AlternateHeadSpriteFrameIndex;

impl CoreLogic for AlternateHeadSpriteFrameIndex {
    fn execute(&self, game_state: &mut GameState, _sink: &mut rodio::Sink) {
        if game_state.player.proximity_to_food {
            game_state.player.head_sprite_frame_index = 3;
        } else {
            let elapsed_time = game_state.player.head_last_sprite_frame_index_update_time.elapsed().as_millis();

            // Check if we should show sprite 2 (every 5000ms for 500ms duration)
            let cycle_position = elapsed_time % 5000;

            if cycle_position < 500 {
                // Show sprite 2 for the first 500ms of every 2000ms cycle
                game_state.player.head_sprite_frame_index = 2;
            } else {
                // For the remaining 1500ms, alternate between sprites 0 and 1 every 500ms
                let remaining_time = cycle_position - 500; // Time since sprite 2 ended
                let sub_cycle = remaining_time / 500;

                game_state.player.head_sprite_frame_index = if sub_cycle % 2 == 0 { 0 } else { 1 };
            }
        }
    }
}