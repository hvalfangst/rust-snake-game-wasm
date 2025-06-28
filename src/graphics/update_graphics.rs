use crate::graphics::sprites::draw_sprite;
use crate::state::constants::graphics::ART_WIDTH;
use crate::state::structs::{Direction, GameState};

pub fn update_pixel_buffer(game_state: &mut GameState) {
    draw_game_world(game_state);
    draw_food(game_state);
    draw_player(game_state);

}

fn draw_food(game_state: &mut GameState) {
    // Draw the food sprite at the food's position
    draw_sprite(
        game_state.food.position.x as usize, // X position of the food
        game_state.food.position.y as usize, // Y position of the food
        &game_state.sprites.food[*&game_state.food.current_sprite_frame_index], // Food sprite
        game_state.window_buffer, // Buffer to draw on
        ART_WIDTH // Width of art
    );
}

fn draw_player(game_state: &mut GameState) {

    let head_position = &game_state.player.body[0];

    println!("Drawing player at position: ({}, {})", head_position.x, head_position.y);

    // TODO: Must handle each direction uniquely

    let offset: f32 = match game_state.player.direction {
        Direction::Right => 0.0,
        Direction::Left => 10.0,
        Direction::Up => 7.0,
        Direction::Down => 0.0,
    };

    // Draw head first
    draw_sprite(
        (head_position.x - offset) as usize, // X position of the head
        (head_position.y - offset) as usize, // Y position of the head
        &game_state.sprites.head[0], // Head sprite
        game_state.window_buffer, // Buffer to draw on
        ART_WIDTH // Width of art
    );

    // Draw the body segments of the snake based on the size of the body
    let body_size = game_state.player.body.len();
    println!("Drawing player body with size: {}", body_size);
    let segment_height = game_state.sprites.body[0].height; // Height of the body segment sprite
    let segment_width = game_state.sprites.body[0].width;  // Width of the body segment sprite

    // Append chunks to the back of the head
    for i in 1..body_size {
        // Calculate the position of the current body segment based on the head position and index
        let (x, y) = match game_state.player.direction {
            Direction::Right => ((head_position.x as usize).saturating_sub(i * segment_width as usize), head_position.y as usize),
            Direction::Left => (head_position.x as usize + i * segment_width as usize, head_position.y as usize),
            Direction::Up => (head_position.x as usize, (head_position.y as usize).saturating_add(i * segment_height as usize)),
            Direction::Down => (head_position.x as usize, (head_position.y as usize).saturating_sub(i * segment_height as usize)),
        };

        // let sprite = if i == body_size - 1 {
        //     &game_state.sprites.body[1] // Use the first body segment sprite for the last segment
        // } else {
        //     &game_state.sprites.body[1] // Use the second body segment sprite for all other segments
        // };

        draw_sprite(
            x, // X position of the body segment
            y, // Y position of the body segment
            &game_state.sprites.body[1], //Bo segment sprite
            game_state.window_buffer, // Buffer to draw on
            ART_WIDTH // Width of art
        );
    }
}

fn draw_game_world(game_state: &mut GameState) {
    draw_sprite(
        0, // X position of the sprite
        0, // Y position of the sprite
        &game_state.sprites.background[0], // Background sprite
        game_state.window_buffer, // Buffer to draw on
        ART_WIDTH // Width of art
    );

}

