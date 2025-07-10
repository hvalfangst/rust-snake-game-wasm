use minifb::{Key, KeyRepeat};
use crate::graphics::render_graphics::render_pixel_buffer;
use crate::graphics::update_graphics::draw_choose_perk_screen_with_highlight;
use crate::state::core_logic::CoreLogic;

pub struct CheckNewPerk;



impl CoreLogic for CheckNewPerk {
    fn execute(&self, game_state: &mut crate::state::structs::GameState, _sink: &mut rodio::Sink) {
        if game_state.perk_available {
            game_state.selected_perk = None;
            let mut highlighted_perk: Option<usize> = None;
            let mut perk_selected = false;

            let key_perk_map = [
                (Key::A, -1), // Move left
                (Key::D, 1),  // Move right
            ];

            loop {

                for (key, direction) in key_perk_map.iter() {
                    if game_state.window.is_key_down(*key) {
                        if let Some(current) = highlighted_perk {
                            let new_perk = (current as isize + direction).clamp(1, 2) as usize;
                            highlighted_perk = Some(new_perk);
                        } else {
                            highlighted_perk = Some(1); // Default to the first perk if none is highlighted
                        }
                    }

                    if game_state.window.is_key_pressed(Key::Space, KeyRepeat::No) {
                        if let Some(perk) = highlighted_perk {
                            game_state.selected_perk = Some(perk);
                            perk_selected = true;
                            break;
                        }
                    }
                }

                draw_choose_perk_screen_with_highlight(game_state, highlighted_perk);
                render_pixel_buffer(game_state);

                if perk_selected {
                    game_state.perk_available = false;

                    match game_state.selected_perk {
                        Some(1) => {
                            // Example perk: 10 % Speed Boost
                            game_state.player.move_interval -= 0.01;
                        },
                        Some(2) => {
                            // Example perk: Double Points
                            game_state.score *= 2;
                        }
                        _ => {}
                    }

                    std::thread::sleep(std::time::Duration::from_millis(200));
                    break;
                }


            }
        }
    }
}