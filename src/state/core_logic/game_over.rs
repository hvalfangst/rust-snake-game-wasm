use crate::graphics::update_graphics::draw_game_over_screen;
use crate::state::core_logic::CoreLogic;
use crate::state::structs::GameState;
use rodio::Sink;
use crate::graphics::render_graphics::render_pixel_buffer;

pub struct CheckGameOver;

impl CoreLogic for CheckGameOver {
    fn execute(&self, game_state: &mut GameState, _sink: &mut Sink) {

        if game_state.game_over {
            let mut frame = 0;

            while frame < 8 {
                draw_game_over_screen(game_state, frame);
                render_pixel_buffer(game_state);

                // Sleep for 200 ms
                std::thread::sleep(std::time::Duration::from_millis(200));

                frame += 1;
            }

            game_state.restart_level();
        }
        }
}