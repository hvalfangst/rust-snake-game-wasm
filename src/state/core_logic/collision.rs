use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;
use rodio::Sink;

pub struct CheckSelfCollision;

impl CoreLogic for CheckSelfCollision {
    fn execute(&self, game_state: &mut GameState, sink: &mut Sink) {
        if game_state.player.body.len() <= 1 {
            return; // Can't collide with self if only head exists
        }

        let head_position = &game_state.player.body[0];

        // Check if head collides with any body segment (starting from index 1)
        for i in 1..game_state.player.body.len() {
            let body_segment = &game_state.player.body[i];

            // Check if head position overlaps with this body segment
            if Self::positions_overlap(head_position, body_segment) {
                println!("Game Over! Snake collided with itself at segment {}", i);
                game_state.game_over = true;
                return;
            }
        }
    }
}

impl CheckSelfCollision {
    fn positions_overlap(pos1: &crate::state::structs::Vector2D, pos2: &crate::state::structs::Vector2D) -> bool {
        // Since we're using exact grid movement, check for exact position match
        const COLLISION_TOLERANCE: f32 = 1.0; // Small tolerance for floating point precision

        (pos1.x - pos2.x).abs() < COLLISION_TOLERANCE && (pos1.y - pos2.y).abs() < COLLISION_TOLERANCE
    }
}