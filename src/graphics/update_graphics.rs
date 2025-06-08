use crate::graphics::sprites::draw_sprite;

use crate::state::structs::GameState;

pub fn update_pixel_buffer(game_state: &mut GameState) {
    draw_game_world(game_state);
    draw_player(game_state)
}

fn draw_player(game_state: &mut GameState) {

    // Determine the current direction and action of the player
    let direction = game_state.player.direction;

    // Determine the sprite to draw
    let sprite_to_draw = &game_state.sprites.player[0];


    // Draw the chosen player sprite
    draw_sprite(
        game_state.player.x as usize,
        game_state.player.y as usize,
        sprite_to_draw,
        game_state.window_buffer,
        game_state.art_width
    );
}

fn draw_game_world(game_state: &mut GameState) {
    draw_sprite(
        0, // X position of the sprite
        0, // Y position of the sprite
        &game_state.sprites.background[0], // Background sprite
        game_state.window_buffer, // Buffer to draw on
        game_state.art_width // Width of art
    );

}

