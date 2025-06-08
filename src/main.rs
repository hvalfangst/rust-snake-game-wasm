use minifb::{Window, WindowOptions};
use std::io::{BufRead, Read};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::state::constants::graphics::{SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};

use crate::state::structs::{GameState, Player};
use crate::{
    graphics::sprites::SpriteMaps,
    state::core_logic::initialize_core_logic_map,
    state::event_loop::start_event_loop,
};
use input::handler::initialize_input_logic_map;
use rodio::{OutputStream, Sink};

mod state;
mod graphics;

mod input;

fn main() {
    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    let sprites = SpriteMaps::new();
    let mut player = Player::new(100.0, 176.0);
    let input_logic = initialize_input_logic_map();
    let core_logic = initialize_core_logic_map();
    let fullscreen = false;

    // Determine window size based on fullscreen flag
    let (window_width, window_height) = if fullscreen {
        let primary_monitor: MonitorHandle =  EventLoop::new().primary_monitor().expect("Failed to get primary monitor");
        let screen_size = primary_monitor.size();
        (screen_size.width as usize, screen_size.height as usize)
    } else {
        (SCALED_WINDOW_WIDTH, SCALED_WINDOW_HEIGHT)
    };

    // Create a window with the dimensions of the primary monitor
    let mut window = Window::new(
        "Snake",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Initialize window and scaled buffer
    let mut window_buffer = vec![0; 256 * 224];
    let mut scaled_buffer = vec![0; window_width * window_height];

    let game_state = GameState {
        player,
        sprites,
        window_buffer: &mut window_buffer,
        window_width,
        window_height,
        window: &mut window,
        scaled_buffer: &mut scaled_buffer,
        art_width: 256, // Assuming the art width is 256 pixels
        art_height: 224, // Assuming the art height is 224 pixels
    };

    start_event_loop(game_state, input_logic, core_logic, &mut sink);
}