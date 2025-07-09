use crate::state::constants::graphics::{ART_HEIGHT, ART_WIDTH};
use crate::state::structs::GameState;

/// Returns a bitmap pattern for a given character as an array of 8 bytes.
///
/// Each byte in the array represents a row of a 6x8 pixel grid, where the bits
/// in the byte correspond to whether a pixel is "on" (1) or "off" (0).
///
/// # Arguments
///
/// * `ch` - A `char` representing the character for which the pattern is needed.
///
/// # Returns
///
/// An array of 8 bytes (`[u8; 8]`) where each byte represents a row of the character's
/// bitmap pattern. If the character is not supported, an array of zeros is returned.
/// ```
pub fn get_character_pattern(ch: char) -> [u8; 8] {
    match ch {
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
    }
}

/// Draws a character on the screen at the specified position.
///
/// # Parameters
/// - `ch`: The character to draw.
/// - `x`: The x-coordinate of the top-left corner where the character will be drawn.
/// - `y`: The y-coordinate of the top-left corner where the character will be drawn.
/// - `game_state`: A mutable reference to the `GameState` containing the window buffer and other game data.
pub fn draw_character(ch: char, x: usize, y: usize, game_state: &mut GameState) {
    let pattern = get_character_pattern(ch);

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