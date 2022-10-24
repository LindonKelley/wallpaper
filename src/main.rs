use image::RgbImage;
use palette::{LinLuma, Srgb, FromColor, Hsv, Hsl};

const IMAGE_WIDTH: u32 = 1920;
const IMAGE_HEIGHT: u32 = 1080;
const SEED: u64 = 1031;
const CHUNK_WIDTH: u32 = 40;
const CHUNK_HEIGHT: u32 = 40;

const CHUNKS_WIDTH: u32 = IMAGE_WIDTH / CHUNK_WIDTH + 1;
const CHUNKS_HEIGHT: u32 = IMAGE_HEIGHT / CHUNK_HEIGHT + 1;

fn main() {
    let mut image = RgbImage::new(IMAGE_WIDTH, IMAGE_HEIGHT);
    for cy in 0..CHUNKS_HEIGHT {
        let cy_min = cy * CHUNK_HEIGHT;
        let cy_max = cy_min + 1;
        for cx in 0..CHUNKS_WIDTH {
            let cx_min = cx * CHUNK_HEIGHT;
            let cx_max = cx_min + 1;

            let noise_min_min = generate_2d(cx_min as u64 * cx as u64 * 9487098521 + 6051548827, cy_min as u64 * cy as u64 * 9243528143 + 2188797433);
            let noise_min_max = generate_2d(cx_min as u64 * cx as u64 * 8177198077 + 9666026099, cy_max as u64 * cy as u64 * 5415044891 + 5883733717);
            let noise_max_min = generate_2d(cx_max as u64 * cx as u64 * 4828519567 + 3052300007, cy_min as u64 * cy as u64 * 3486176611 + 2809378237);
            let noise_max_max = generate_2d(cx_max as u64 * cx as u64 * 3221596519 + 5573303501, cy_max as u64 * cy as u64 * 1564159609 + 5800857247);

            let min_y_values = (noise_min_min, noise_min_max);
            let max_y_values = (noise_max_min, noise_max_max);

            for y_off in 0..CHUNK_HEIGHT {
                let y = cy_min + y_off;
                let v_y = y_off as f64 / CHUNK_HEIGHT as f64;
                for x_off in 0..CHUNK_WIDTH {
                    let x = cx_min + x_off;
                    if let Some(pixel) = image.get_pixel_mut_checked(x, y) {
                        let x1_value = interpolate(min_y_values, v_y);
                        let x2_value = interpolate(max_y_values, v_y);
                        let x_values = (x1_value, x2_value);
                        let v_x = x_off as f64 / CHUNK_WIDTH as f64;
                        let value = interpolate(x_values, v_x);
                        let hsv = Hsv::new((x as f64 / IMAGE_WIDTH as f64) * 360.0, 0.8, value);
                        let (r, g, b) = Srgb::from_color(hsv).into_components();
                        pixel.0 = [r, g, b].map(|f| (f * 255.0) as u8);
                    }
                }
            }
        }
    }
    image.save("output/wallpaper.png").unwrap();
}

fn interpolate((min, max): (f64, f64), v: f64) -> f64 {
    v * (max - min) + min
}

fn generate_2d(x: u64, y: u64) -> f64 {
    ((generate(x) ^ generate(y)) % 1024) as f64 / 1024.0
}

/// pseudo-randomly generates a u64 from a u64
fn generate(v: u64) -> u64 {
    4152460847 * (v ^ SEED) + 5577138713
}
