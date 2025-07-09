use image::GenericImageView;

pub struct SpriteFrame {
    pub width: u32,  // Width of the sprite in pixels
    pub height: u32, // Height of the sprite in pixels
    data: Vec<u32> // Pixel data of the sprite, typically in ARGB or RGBA format
}

impl SpriteFrame {
    fn new(width: u32, height: u32, data: Vec<u32>) -> Self {
        Self { width, height, data }
    }
}

pub struct SpriteMaps {
    pub body: Vec<SpriteFrame>,
    pub background: Vec<SpriteFrame>,
    pub food: Vec<SpriteFrame>,
    pub head: Vec<SpriteFrame>,
    pub tail: Vec<SpriteFrame>,
    pub game_over_screen: Vec<SpriteFrame>,
    pub layer_0: Vec<SpriteFrame>,
    pub layer_1: Vec<SpriteFrame>,
    pub blue_strip: Vec<SpriteFrame>,
}

impl SpriteMaps {
    pub fn new() -> Self {
        Self {
            background: load_sprites_from_map("assets/sprites/background.png", 256, 224),
            body: load_sprites_from_map("assets/sprites/body.png", 6, 6),
            food: load_sprites_from_map("assets/sprites/food.png", 16, 16),
            head: load_sprites_from_map("assets/sprites/head.png", 16, 16),
            tail: load_sprites_from_map("assets/sprites/tail.png", 6, 6),
            game_over_screen: load_sprites_from_map("assets/sprites/game_over.png", 256, 224),
            layer_0: load_sprites_from_map("assets/sprites/layer_0.png", 256, 224),
            layer_1: load_sprites_from_map("assets/sprites/layer_1.png", 256, 224),
            blue_strip: load_sprites_from_map("assets/sprites/blue_strip.png", 256, 224),
        }
    }
}

/// Loads sprites from a sprite map image file into memory.
///
/// Opens the image file specified by `sprite_map_path`, extracts individual
/// sprites based on `sprite_width` and `sprite_height`, and stores each sprite in a buffer.
///
/// # Parameters
/// - `sprite_map_path`: A string slice containing the path to the sprite map image file.
/// - `sprite_width`: The width of each individual sprite in pixels.
/// - `sprite_height`: The height of each individual sprite in pixels.
///
/// # Returns
/// A vector containing tuples of sprite dimensions and pixel data.
pub fn load_sprites_from_map(sprite_map_path: &str, sprite_width: u32, sprite_height: u32) -> Vec<SpriteFrame> {
    // Load the sprite map image
    let sprite_map = image::open(sprite_map_path).expect(&format!("Failed to open sprite map at {}", sprite_map_path));
    let (map_width, map_height) = sprite_map.dimensions();

    println!("Sprite map loaded from {}", sprite_map_path);
    println!("Sprite map dimensions: {}x{}", map_width, map_height);

    // Calculate the number of sprites in each dimension
    let sprites_x = map_width / sprite_width;
    let sprites_y = map_height / sprite_height;

    println!("Sprites x: {}", sprites_x);
    println!("Sprites y: {}", sprites_y);

    // Extract individual sprites and store them in a buffer
    let mut sprites = Vec::new();
    for y in 0..sprites_y {
        for x in 0..sprites_x {
            println!("Extracting sprite at ({}, {})", x, y);
            let sprite = sprite_map.crop_imm(x * sprite_width, y * sprite_height, sprite_width, sprite_height);
            let buffer = img_to_buffer(&sprite);
            println!("Sprite extracted: {}x{}, buffer length: {}", sprite_width, sprite_height, buffer.len());
            let new_sprite = SpriteFrame::new(sprite_width, sprite_height, buffer);
            sprites.push(new_sprite);
        }
    }

    println!("Total sprites extracted: {}\n", sprites.len());

    // Return the vector of sprites
    sprites
}

/// Converts an image to a buffer of u32 pixels in ARGB format.
///
/// Each pixel in the buffer is represented as ARGB (Alpha, Red, Green, Blue).
///
/// # Parameters
/// - `img`: A reference to the `DynamicImage` to be converted.
///
/// # Returns
/// A vector of u32 pixels representing the image in ARGB format.
pub fn img_to_buffer(img: &image::DynamicImage) -> Vec<u32> {
    img.to_rgba8().pixels().map(|p| {
        let channels = p.0;
        ((channels[3] as u32) << 24) // Alpha channel
            | ((channels[0] as u32) << 16) // Red channel
            | ((channels[1] as u32) << 8)  // Green channel
            | (channels[2] as u32)         // Blue channel
    }).collect()
}

/// Draws a sprite onto the window buffer at the specified coordinates, with alpha blending.
///
/// # Parameters
/// - `x`: The x-coordinate where the sprite will be drawn.
/// - `y`: The y-coordinate where the sprite will be drawn.
/// - `sprite`: A tuple containing the sprite's width, height, and pixel data. The pixel data is a vector of `u32` values representing RGBA colors.
/// - `window_buffer`: A mutable slice of `u32` representing the pixels of the window buffer. Each `u32` value represents an RGBA color.
/// - `window_width`: The width of the window in pixels.
///
/// This function uses alpha blending to combine the sprite's pixels with the corresponding pixels in the window buffer. Only non-transparent pixels in the sprite are drawn.
///
/// # Alpha Blending
/// Alpha blending is a process used in computer graphics to combine a foreground image with a background image, resulting in a composite image.
/// The alpha value determines the transparency level of the pixel:
/// - An alpha value of 0 means the pixel is completely transparent.
/// - An alpha value of 255 (0xFF) means the pixel is completely opaque.
///
/// The formula for alpha blending is:
/// ```
/// blended_color = (foreground_color * alpha + background_color * (255 - alpha)) / 255
/// ```
///
/// # ARGB Color Palette
/// Each `u32` value in the pixel data represents a color in ARGB format:
/// - The highest 8 bits represent the alpha (transparency) channel.
/// - The next 8 bits represent the red channel.
/// - The next 8 bits represent the green channel.
/// - The lowest 8 bits represent the blue channel.
///
/// For example, a color represented as `0x80FF00FF` means:
/// - Alpha: 0x80 (128 in decimal, semi-transparent)
/// - Red: 0xFF (255 in decimal, full intensity)
/// - Green: 0x00 (0 in decimal, no intensity)
/// - Blue: 0xFF (255 in decimal, full intensity)
///
/// # Example Usage
/// ```rust
/// let sprite = (16, 16, vec![0x80FF00FF; 256]); // A semi-transparent magenta 16x16 sprite
/// let mut window_buffer = vec![0xFFFFFFFF; 800 * 600]; // A white 800x600 window buffer
/// draw_sprite(10, 10, &sprite, &mut window_buffer, 800);
/// ```
pub fn draw_sprite(
    x: usize,
    y: usize,
    sprite: &SpriteFrame,
    window_buffer: &mut [u32],
    window_width: usize,
    darkness_factor: Option<f32> // None = no darkening, Some(0.5) = 50% darker
) {
    for row in 0..sprite.height as usize {
        for col in 0..sprite.width as usize {
            let sprite_pixel_index = row * (sprite.width as usize) + col;
            let window_pixel_index = (y + row) * window_width + (x + col);

            if window_pixel_index < window_buffer.len() {
                let mut sprite_pixel = sprite.data[sprite_pixel_index];

                // Apply darkening if specified
                if let Some(factor) = darkness_factor {
                    let alpha = (sprite_pixel >> 24) & 0xFF;
                    let r = ((sprite_pixel >> 16) & 0xFF) as f32 * factor;
                    let g = ((sprite_pixel >> 8) & 0xFF) as f32 * factor;
                    let b = (sprite_pixel & 0xFF) as f32 * factor;

                    sprite_pixel = (alpha << 24) |
                        ((r as u32).min(255) << 16) |
                        ((g as u32).min(255) << 8) |
                        (b as u32).min(255);
                }

                let sprite_alpha = (sprite_pixel >> 24) & 0xFF;
                let sprite_rgb = sprite_pixel & 0x00FFFFFF;

                if sprite_alpha > 0 {
                    let window_pixel = window_buffer[window_pixel_index];
                    let window_rgb = window_pixel & 0x00FFFFFF;

                    let blended_r = ((sprite_rgb >> 16) & 0xFF) * sprite_alpha / 255 + ((window_rgb >> 16) & 0xFF) * (255 - sprite_alpha) / 255;
                    let blended_g = ((sprite_rgb >> 8) & 0xFF) * sprite_alpha / 255 + ((window_rgb >> 8) & 0xFF) * (255 - sprite_alpha) / 255;
                    let blended_b = (sprite_rgb & 0xFF) * sprite_alpha / 255 + (window_rgb & 0xFF) * (255 - sprite_alpha) / 255;

                    let blended_pixel = 0xFF000000 | (blended_r & 0xFF) << 16 | (blended_g & 0xFF) << 8 | (blended_b & 0xFF);
                    window_buffer[window_pixel_index] = blended_pixel;
                }
            }
        }
    }
}

/// Sets a pixel in the window buffer at the specified coordinates.
///
/// # Parameters
/// - `x`: The x-coordinate where the pixel will be set.
/// - `y`: The y-coordinate where the pixel will be set.
/// - `color`: The color of the pixel in ARGB format.
/// - `window_buffer`: A mutable slice of `u32` representing the pixels of the window buffer.
/// - `window_width`: The width of the window in pixels.
pub fn set_pixel(x: usize, y: usize, color: u32, window_buffer: &mut [u32], window_width: usize) {
    let window_pixel_index = y * window_width + x;
    if window_pixel_index < window_buffer.len() {
        window_buffer[window_pixel_index] = color;
    }
}


