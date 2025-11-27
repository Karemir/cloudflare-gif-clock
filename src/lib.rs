use fontdue::{
    layout::{CoordinateSystem, Layout, LayoutSettings, TextStyle},
    Font, FontSettings,
};
use gif::{Encoder, Frame, Repeat};
use once_cell::sync::Lazy;
use std::io::Cursor;
use worker::*;

const GRADIENT_LEVELS: u8 = 16;

static FONT_DATA: &[u8] = include_bytes!("fonts/Lato-Regular.ttf");
static FONT: Lazy<Font> = Lazy::new(|| {
    Font::from_bytes(FONT_DATA, FontSettings::default()).expect("Failed to load font")
});

#[event(fetch)]
async fn fetch(_req: HttpRequest, _env: Env, _ctx: Context) -> Result<Response> {
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

fn build_grayscale_palette(levels: u8) -> Vec<u8> {
    let mut palette = Vec::with_capacity((levels as usize + 1) * 3);
    palette.extend_from_slice(&[0xFF, 0xFF, 0xFF]); // background color (white)
    for step in 0..levels {
        let t = (step + 1) as f32 / levels as f32;
        let value = (255.0 * (1.0 - t)).round() as u8;
        palette.extend_from_slice(&[value, value, value]);
    }
    palette
}

fn generate_countdown_gif(current_timestamp: u64) -> Vec<u8> {
    // Set up GIF parameters
    let width: u32 = 600;
    let height: u32 = 200;
    let color_map = build_grayscale_palette(GRADIENT_LEVELS);

    // Create a buffer to hold the GIF data
    let mut buffer = Vec::new();
    let mut cursor = Cursor::new(&mut buffer);

    // Create GIF encoder
    let mut encoder = Encoder::new(&mut cursor, width as u16, height as u16, &color_map).unwrap();
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
        frame.delay = 100; // 1 second delay (in 1/100ths of a second)

        encoder.write_frame(&frame).unwrap();
    }

    // Return the generated GIF data
    drop(encoder);
    buffer
}

fn draw_number(buffer: &mut [u8], width: u32, height: u32, hours: u32, minutes: u32, seconds: u32) {
    buffer.fill(0);

    let time_str = format!("{:02}:{:02}:{:02}", hours, minutes, seconds);
    let font_size = 120.0;

    let mut layout = Layout::new(CoordinateSystem::PositiveYDown);
    layout.reset(&LayoutSettings {
        ..LayoutSettings::default()
    });
    layout.append(&[&*FONT], &TextStyle::new(&time_str, font_size, 0));

    let glyphs = layout.glyphs();
    if glyphs.is_empty() {
        return;
    }

    let mut min_x = f32::MAX;
    let mut min_y = f32::MAX;
    let mut max_x = f32::MIN;
    let mut max_y = f32::MIN;
    let mut has_area = false;

    for glyph in glyphs {
        if glyph.width == 0 || glyph.height == 0 {
            continue;
        }
        has_area = true;
        min_x = min_x.min(glyph.x);
        min_y = min_y.min(glyph.y);
        max_x = max_x.max(glyph.x + glyph.width as f32);
        max_y = max_y.max(glyph.y + glyph.height as f32);
    }

    if !has_area {
        return;
    }

    let text_width = max_x - min_x;
    let text_height = max_y - min_y;
    let offset_x = ((width as f32 - text_width) / 2.0) - min_x;
    let offset_y = ((height as f32 - text_height) / 2.0) - min_y;
    let buf_width = width as i32;
    let buf_height = height as i32;

    for glyph in glyphs {
        if glyph.width == 0 || glyph.height == 0 {
            continue;
        }

        let (metrics, bitmap) = FONT.rasterize_config(glyph.key);
        let glyph_x = glyph.x + offset_x;
        let glyph_y = glyph.y + offset_y;

        for y in 0..metrics.height {
            let py = (glyph_y + y as f32).floor() as i32;
            if py < 0 || py >= buf_height {
                continue;
            }

            for x in 0..metrics.width {
                let px = (glyph_x + x as f32).floor() as i32;
                if px < 0 || px >= buf_width {
                    continue;
                }

                let coverage = bitmap[y * metrics.width + x];
                let level = ((coverage as u16 * GRADIENT_LEVELS as u16) / 255) as u8;
                if level > 0 {
                    let index = py as usize * width as usize + px as usize;
                    if index < buffer.len() {
                        buffer[index] = level.min(GRADIENT_LEVELS);
                    }
                }
            }
        }
    }
}
