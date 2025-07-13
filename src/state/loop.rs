use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::{thread, time};
use minifb::Key;

use crate::graphics::render::render_pixel_buffer;
use crate::graphics::update::update_pixel_buffer;
use crate::input::handler::handle_user_input;
use crate::state::constants::audio::MUSIC_0_FILE;
use crate::state::constants::state::FRAME_RATE_SLEEP_DURATION;
use crate::state::core::{execute_core_logic, CoreLogic};
use crate::state::structs::GameState;

pub fn start_event_loop(mut game_state: GameState, core_logic_map: HashMap<String, Rc<RefCell<dyn CoreLogic>>>) {

    // Main event loop: runs as long as the window is open and the Escape key is not pressed
    while game_state.window.is_open() && !game_state.window.is_key_down(Key::Escape) {

        if !game_state.audio_manager.is_music_playing() && !game_state.music_disabled {
            println!("Playing background music...");
            game_state.audio_manager.play_music(MUSIC_0_FILE).unwrap_or_else(|e| {
                eprintln!("Failed to play music: {}", e);
            });
        }

        // Handle basic user input, which influence player direction
        handle_user_input(&mut game_state);

        // Process game logic such as obstacle detection
        execute_core_logic(&mut game_state, &core_logic_map);

        // Update the pixel buffer with the current game state
        update_pixel_buffer(&mut game_state);

        // Render the updated buffer
        render_pixel_buffer(&mut game_state);

        // Sleep for x ms to maintain a frame rate of approximately y FPS
        thread::sleep(time::Duration::from_millis(FRAME_RATE_SLEEP_DURATION));
    }
}