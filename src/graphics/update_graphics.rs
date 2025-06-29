use crate::graphics::sprites::draw_sprite;
use crate::state::constants::graphics::{ART_HEIGHT, ART_WIDTH};
use crate::state::structs::{Direction, GameState};

pub fn update_pixel_buffer(game_state: &mut GameState) {
    draw_game_world(game_state);
    draw_food(game_state);
    draw_player(game_state);
    draw_score(game_state);

}

fn draw_score(game_state: &mut GameState) {
    let score_text = game_state.score.to_string();
    let char_width = 6;
    let char_spacing = 2;

    let total_width = (score_text.len() * char_width) + ((score_text.len() - 1) * char_spacing);
    let start_x = (ART_WIDTH / 2) - (total_width / 2);
    let start_y = 10;

    for (i, ch) in score_text.chars().enumerate() {
        let x_pos = start_x + (i * (char_width + char_spacing));
        draw_character(ch, x_pos, start_y, game_state);
    }
}

fn draw_character(ch: char, x: usize, y: usize, game_state: &mut GameState) {


    let pattern = match ch {
        '0' => [
            0b011110,
            0b100001,
            0b100011,
            0b101101,
            0b110001,
            0b100001,
            0b011110,
            0b000000,
        ],
        '1' => [
            0b001100,
            0b011100,
            0b001100,
            0b001100,
            0b001100,
            0b001100,
            0b111111,
            0b000000,
        ],
        '2' => [
            0b011110,
            0b100001,
            0b000001,
            0b000110,
            0b011000,
            0b100000,
            0b111111,
            0b000000,
        ],
        '3' => [
            0b011110,
            0b100001,
            0b000001,
            0b001110,
            0b000001,
            0b100001,
            0b011110,
            0b000000,
        ],
        '4' => [
            0b000110,
            0b001110,
            0b010110,
            0b100110,
            0b111111,
            0b000110,
            0b000110,
            0b000000,
        ],
        '5' => [
            0b111111,
            0b100000,
            0b111110,
            0b000001,
            0b000001,
            0b100001,
            0b011110,
            0b000000,
        ],
        '6' => [
            0b011110,
            0b100000,
            0b100000,
            0b111110,
            0b100001,
            0b100001,
            0b011110,
            0b000000,
        ],
        '7' => [
            0b111111,
            0b000001,
            0b000010,
            0b000100,
            0b001000,
            0b010000,
            0b100000,
            0b000000,
        ],
        '8' => [
            0b011110,
            0b100001,
            0b100001,
            0b011110,
            0b100001,
            0b100001,
            0b011110,
            0b000000,
        ],
        '9' => [
            0b011110,
            0b100001,
            0b100001,
            0b011111,
            0b000001,
            0b000001,
            0b011110,
            0b000000,
        ],
        _ => [0; 8],
    };

    for (row, &pattern_row) in pattern.iter().enumerate() {
        for col in 0..6 {
            if (pattern_row >> (5 - col)) & 1 == 1 {
                let pixel_x = x + col;
                let pixel_y = y + row;

                if pixel_x < ART_WIDTH && pixel_y < ART_HEIGHT {
                    let index = pixel_y * ART_WIDTH + pixel_x;
                    if index < game_state.window_buffer.len() {
                        game_state.window_buffer[index] = 0xFFFFFF; // White pixel
                    }
                }
            }
        }
    }
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

