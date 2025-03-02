use gif::{Encoder, Frame, Repeat};
use std::io::Cursor;
use worker::*;

#[event(fetch)]
async fn fetch(
    _req: HttpRequest,
    _env: Env,
    _ctx: Context,
) -> Result<Response> {
    console_error_panic_hook::set_once();
    
    // Get current timestamp using JavaScript Date
    let timestamp = js_sys::Date::now() / 1000.0;
    
    // Generate the countdown GIF
    let gif_data = generate_countdown_gif(timestamp as u64);
    let mut resp = Response::from_bytes(gif_data).unwrap();
    // Set the Content-Type header to image/gif
    resp.headers_mut().set("Content-Type", "image/gif").unwrap();

    Ok(resp)
}

fn generate_countdown_gif(current_timestamp: u64) -> Vec<u8> {
    // Set up GIF parameters
    let width: u32 = 600;
    let height: u32 = 200;
    let color_map = &[0xFF, 0xFF, 0xFF, 0, 0, 0];  // White and black
    
    // Create a buffer to hold the GIF data
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);
    
    // Create GIF encoder
    let mut encoder = Encoder::new(&mut cursor, width as u16, height as u16, color_map).unwrap();
    encoder.set_repeat(Repeat::Infinite).unwrap();
    
    // Create frames for countdown from 60 to 0
    for i in 0..=60 {
        // Create a new frame for this number
        let mut frame_data = vec![0; (width * height) as usize];
        
        // Calculate time using the provided timestamp
        let total_seconds = current_timestamp + i;
        let hours = ((total_seconds / 3600) % 24) as u32;
        let minutes = ((total_seconds / 60) % 60) as u32;
        let seconds = (total_seconds % 60) as u32;
        
        // In a real implementation, you would use a proper text rendering library
        draw_number(&mut frame_data, width, height, hours, minutes, seconds);
        
        // Create and add the frame
        let mut frame = Frame::default();
        frame.width = width as u16;
        frame.height = height as u16;
        frame.buffer = Into::into(frame_data);
        frame.delay = 100;  // 1 second delay (in 1/100ths of a second)
        
        encoder.write_frame(&frame).unwrap();
    }
    
    // Return the generated GIF data
    drop(encoder);
    buffer
}

fn draw_number(buffer: &mut [u8], width: u32, height: u32, hours: u32, minutes: u32, seconds: u32) {
    // Clear the buffer (set to white)
    for pixel in buffer.iter_mut() {
        *pixel = 0;  // White background
    }
    
    // Define a simple 5x7 bitmap font for digits
    let digits = [
        // 0
        [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        // 1
        [
            [0, 0, 1, 0, 0],
            [0, 1, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 1, 1, 0],
        ],
        // 2
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
        ],
        // 3
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        // 4
        [
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
        ],
        // 5
        [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        // 6
        [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 0],
            [1, 0, 0, 0, 0],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        // 7
        [
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 1, 0],
            [0, 0, 1, 0, 0],
            [0, 1, 0, 0, 0],
            [1, 0, 0, 0, 0],
        ],
        // 8
        [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        // 9
        [
            [1, 1, 1, 1, 1],
            [1, 0, 0, 0, 1],
            [1, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
            [0, 0, 0, 0, 1],
            [0, 0, 0, 0, 1],
            [1, 1, 1, 1, 1],
        ],
        // : (colon)
        [
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 1, 0, 0],
            [0, 0, 0, 0, 0],
            [0, 0, 0, 0, 0],
        ],
    ];

    // Calculate digit size with scaling
    let scale = 6; // Scale the font to make it larger
    let digit_width = 5 * scale;
    let digit_height = 7 * scale;
    let spacing = 2 * scale; // Space between digits

    let time_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    
    // Calculate starting position to center the number
    let total_width = time_str.len() as usize * (digit_width + spacing) - spacing;
    let x_start = ((width as usize) - total_width) / 2;
    let y_start = ((height as usize) - digit_height) / 2;

    // Draw each digit
    for (i, ch) in time_str.chars().enumerate() {
        let mut digit: usize = 0;
        if ch >= '0' && ch <= '9' {
            digit = ch.to_digit(10).unwrap() as usize;
        } else if ch == ':' {
            digit = 10;
        }
         
        let x_pos = x_start + i * (digit_width + spacing);
        
        // Draw the digit bitmap
        for y in 0..7 {
            for x in 0..5 {
                if digits[digit][y][x] == 1 {
                    // Draw a scaled pixel (block of pixels)
                    for sy in 0..scale {
                        for sx in 0..scale {
                            let buf_x = x_pos + x * scale + sx;
                            let buf_y = y_start + y * scale + sy;
                            
                            if buf_x < width as usize && buf_y < height as usize {
                                let index = buf_y * width as usize + buf_x;
                                if index < buffer.len() {
                                    buffer[index] = 1; // Set to black
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}