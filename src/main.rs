use minifb::{Window, WindowOptions};
use std::io::{BufRead, Read};
use winit::event_loop::EventLoop;
use winit::monitor::MonitorHandle;

use crate::state::constants::graphics::{ART_WIDTH, SCALED_WINDOW_HEIGHT, SCALED_WINDOW_WIDTH};

use crate::state::structs::{Direction, GameState, Snake};
use crate::{
    graphics::sprites::SpriteMaps,
    state::core_logic::initialize_core_logic_map,
    state::event_loop::start_event_loop,
};

use rodio::{OutputStream, Sink};
mod state;
mod graphics;
mod input;

fn main() {
    // Initialize the audio output stream and sink
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let mut sink = Sink::try_new(&stream_handle).unwrap();
    let sprites = SpriteMaps::new();

    let mut player = Snake::new(40.0, 150.0, Direction::Right);
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
        "Space Wurm",
        window_width,
        window_height,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });


    // Initialize window and scaled buffer
    let mut window_buffer = vec![0; ART_WIDTH * ART_WIDTH];
    let mut scaled_buffer = vec![0; window_width * window_height];

    let game_state = GameState::new(
        player,
        sprites,
        &mut window_buffer,
        window_width,
        window_height,
        &mut window,
        &mut scaled_buffer,
    );

    start_event_loop(game_state, core_logic, &mut sink);
}