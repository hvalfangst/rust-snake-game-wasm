use crate::graphics::sprites::draw_sprite;
use crate::state::constants::graphics::ART_WIDTH;
use crate::state::structs::GameState;

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
        &game_state.sprites.food[0], // Food sprite
        game_state.window_buffer, // Buffer to draw on
        ART_WIDTH // Width of art
    );
}

fn draw_player(game_state: &mut GameState) {

    // Determine the current direction and action of the player
    let direction = game_state.player.direction;


    // // Draw the head of the snake
    // draw_sprite(
    //     game_state.player.ledger[0].x as usize,
    //     game_state.player.ledger[0].y as usize,
    //     &game_state.sprites.player[0],
    //     game_state.window_buffer,
    //     ART_WIDTH
    // );

    for (index, i) in game_state.player.ledger.iter().enumerate() {
        println!("Drawing snake segment index {} at ({}, {})",index,  i.x, i.y);
        // Draw the body of the snake
        draw_sprite(
            i.x as usize,
            i.y as usize,
            &game_state.sprites.player[0],
            game_state.window_buffer,
            ART_WIDTH
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

