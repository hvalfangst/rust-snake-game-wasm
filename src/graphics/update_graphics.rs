use crate::graphics::sprites::draw_sprite;
use crate::graphics::symbols::draw_character;
use crate::state::constants::graphics::{ART_HEIGHT, ART_WIDTH};
use crate::state::structs::{Direction, GameState};

pub fn update_pixel_buffer(game_state: &mut GameState) {
    draw_background(game_state);
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

fn draw_food(game_state: &mut GameState) {
    let food_x = game_state.food.position.x;
    let darkness = match food_x {
        x if x > ART_WIDTH as f32 / 1.7 => Some(0.6),
        x if x > ART_WIDTH as f32 / 1.8 => Some(0.7),
        x if x > ART_WIDTH as f32 / 1.9 => Some(0.8),
        _ => None,
    };

    // Draw the food sprite at the food's position
    draw_sprite(
        food_x as usize,
        game_state.food.position.y as usize,
        &game_state.sprites.food[*&game_state.food.sprite_frame_index],
        game_state.window_buffer,
        ART_WIDTH,
        darkness
    );
}

fn draw_player(game_state: &mut GameState) {

    let head_position = &game_state.player.body[0];
    let darkness = match head_position.x {
        x if x > ART_WIDTH as f32 / 1.7 => Some(0.6),
        x if x > ART_WIDTH as f32 / 1.8 => Some(0.7),
        x if x > ART_WIDTH as f32 / 1.9 => Some(0.8),
        _ => None,
    };

    // Magic number offset based on direction
    let offset: f32 = match game_state.player.direction {
        Direction::Right => 0.0,
        Direction::Left => 10.0,
        Direction::Up => 7.0,
        Direction::Down => 0.0,
    };

    // Draw head first
    draw_sprite(
        (head_position.x - offset) as usize,
        (head_position.y - offset) as usize,
        &game_state.sprites.head[game_state.player.head_sprite_frame_index],
        game_state.window_buffer,
        ART_WIDTH,
        darkness
    );


    // Draw the body segments from neck to buttocks
    for i in 1..game_state.player.body.len() -1 {
        let body_x = game_state.player.body[i].x;
        let darkness = match body_x {
            x if x > ART_WIDTH as f32 / 1.7 => Some(0.6),
            x if x > ART_WIDTH as f32 / 1.8 => Some(0.7),
            x if x > ART_WIDTH as f32 / 1.9 => Some(0.8),
            _ => None,
        };

        draw_sprite(
            body_x as usize,
            game_state.player.body[i].y as usize,
            &game_state.sprites.body[game_state.player.body_sprite_frame_index],
            game_state.window_buffer,
            ART_WIDTH,
            darkness
        );
    }

    // For right and up we draw the first tail sprite frame, left and down we draw the second tail sprite frame
    let tail_sprite_index = if game_state.player.direction == Direction::Right || game_state.player.direction == Direction::Up {
        0
    } else {
        1
    };

    let tail_index = game_state.player.body.len();
    if tail_index > 0 {
        let tail_position = &game_state.player.body[tail_index - 1];
        let darkness = match tail_position.x {
            x if x > ART_WIDTH as f32 / 1.7 => Some(0.6),
            x if x > ART_WIDTH as f32 / 1.8 => Some(0.7),
            x if x > ART_WIDTH as f32 / 1.9 => Some(0.8),
            _ => None,
        };
        draw_sprite(
            tail_position.x as usize,
            tail_position.y as usize,
            &game_state.sprites.tail[tail_sprite_index],
            game_state.window_buffer,
            ART_WIDTH,
            darkness
        );
    }
}

pub fn draw_game_over_screen(game_state: &mut GameState, index: usize) {
    draw_sprite(
        0,
        0,
        &game_state.sprites.game_over_screen[index],
        game_state.window_buffer,
        ART_WIDTH,
        None
    );

    // Draw the score underneath the "Game Over" screen
    let score_text = format!("Score: {}", game_state.score);
    let x_position = (ART_WIDTH / 3) - (score_text.len() * 6 / 2); // Center the text
    let y_position = ART_HEIGHT - 20; // Position near the bottom
    for (i, ch) in score_text.chars().enumerate() {
        draw_character(ch, x_position + i * 8, y_position, game_state);
    }
}


pub fn draw_background(state: &mut GameState) {

    // Always first draw a subset (the top 200 pixels) of our background in order to mitigate void spots
    draw_sprite(
        0,
        0,
        &state.sprites.blue_strip[0],
        state.window_buffer,
        ART_WIDTH,
        None
    );

    // Loop through the layers and draw them based on the player's position
    for (i, divisor) in [12, 1].iter().enumerate() {
        // Calculate offsets for parallax effect
        let (offset_x, offset_y) = if i == 0 {
            // Layer 0 (stars) - moves towards an x position
            (
                state.background_offset_x / divisor, // Use the incrementing x offset
                0
            )
        } else {
            // Layer 1 (space station) - static, no movement
            (0, 0)
        };

        // Increment the x offset for layer 0
        if i == 0 {
            state.background_offset_x = (state.background_offset_x + 1);
        }

        // Select the appropriate layer based on the index
        let layer = match i {
            0 => &state.sprites.layer_0[0],
            1 => &state.sprites.layer_1[0],
            _ => unreachable!(),
        };

        // Draw the sprite with calculated offsets
        draw_sprite(
            offset_x,
            offset_y,
            layer,
            state.window_buffer,
            ART_WIDTH,
            None,
        );
    }
}
