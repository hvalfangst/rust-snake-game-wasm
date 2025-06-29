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

    // Append chunks to the back of the head
    for i in 1..game_state.player.body.len() {
        draw_sprite(
            game_state.player.body[i].x  as usize, // X position of the body segment
            game_state.player.body[i].y as usize, // Y position of the body segment
            &game_state.sprites.body[1], // Body segment sprite
            game_state.window_buffer, // Buffer to draw on
            ART_WIDTH // Width of art
        );
    }

    // Tail must be drawn with different sprite frame
    if game_state.player.body.len() > 1 {
        let tail_position = &game_state.player.body[game_state.player.body.len() - 1];
        draw_sprite(
            tail_position.x as usize, // X position of the tail
            tail_position.y as usize, // Y position of the tail
            &game_state.sprites.body[0], // Tail sprite
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

