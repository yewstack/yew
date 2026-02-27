use std::collections::HashMap;
use std::io::Cursor;
use std::sync::{LazyLock, Mutex};

use image::{ImageBuffer, Rgb};

static CACHE: LazyLock<Mutex<HashMap<(u32, u32, u32), String>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

// Cosine palette (Inigo Quilez style)
// color(t) = a + b * cos(2pi(c*t + d))
struct Palette {
    a: [f64; 3],
    b: [f64; 3],
    c: [f64; 3],
    d: [f64; 3],
}

const PALETTES: &[Palette] = &[
    // sunset
    Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [1.0, 1.0, 1.0],
        d: [0.00, 0.33, 0.67],
    },
    // electric
    Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [1.0, 1.0, 0.5],
        d: [0.80, 0.90, 0.30],
    },
    // forest
    Palette {
        a: [0.5, 0.5, 0.5],
        b: [0.5, 0.5, 0.5],
        c: [2.0, 1.0, 0.0],
        d: [0.50, 0.20, 0.25],
    },
    // candy
    Palette {
        a: [0.8, 0.5, 0.4],
        b: [0.2, 0.4, 0.2],
        c: [2.0, 1.0, 1.0],
        d: [0.00, 0.25, 0.25],
    },
    // ocean
    Palette {
        a: [0.0, 0.5, 0.5],
        b: [0.0, 0.5, 0.5],
        c: [0.0, 0.5, 0.3],
        d: [0.0, 0.0, 0.2],
    },
];

fn palette_color(pal: &Palette, t: f64) -> [u8; 3] {
    let tau = std::f64::consts::TAU;
    let mut rgb = [0u8; 3];
    for i in 0..3 {
        let v = pal.a[i] + pal.b[i] * (tau * (pal.c[i] * t + pal.d[i])).cos();
        rgb[i] = (v.clamp(0.0, 1.0) * 255.0) as u8;
    }
    rgb
}

fn hash(mut x: i32, mut y: i32, seed: u32) -> u32 {
    let mut h = seed;
    x = x.wrapping_mul(374761393);
    y = y.wrapping_mul(668265263);
    h = h.wrapping_add(x as u32).wrapping_mul(2654435761);
    h = h.wrapping_add(y as u32).wrapping_mul(2246822519);
    h ^= h >> 13;
    h = h.wrapping_mul(3266489917);
    h ^= h >> 16;
    h
}

fn grad(hash: u32, dx: f64, dy: f64) -> f64 {
    match hash & 3 {
        0 => dx + dy,
        1 => -dx + dy,
        2 => dx - dy,
        _ => -dx - dy,
    }
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + t * (b - a)
}

fn fade(t: f64) -> f64 {
    t * t * t * (t * (t * 6.0 - 15.0) + 10.0)
}

fn noise2d(x: f64, y: f64, seed: u32) -> f64 {
    let xi = x.floor() as i32;
    let yi = y.floor() as i32;
    let xf = x - x.floor();
    let yf = y - y.floor();
    let u = fade(xf);
    let v = fade(yf);

    let n00 = grad(hash(xi, yi, seed), xf, yf);
    let n10 = grad(hash(xi + 1, yi, seed), xf - 1.0, yf);
    let n01 = grad(hash(xi, yi + 1, seed), xf, yf - 1.0);
    let n11 = grad(hash(xi + 1, yi + 1, seed), xf - 1.0, yf - 1.0);

    lerp(lerp(n00, n10, u), lerp(n01, n11, u), v)
}

fn fbm(x: f64, y: f64, seed: u32, octaves: u32) -> f64 {
    let mut value = 0.0;
    let mut amp = 0.5;
    let mut freq = 1.0;
    for _ in 0..octaves {
        value += amp * noise2d(x * freq, y * freq, seed);
        freq *= 2.0;
        amp *= 0.5;
    }
    value
}

fn warped_fbm(x: f64, y: f64, seed: u32, warp: f64, octaves: u32) -> f64 {
    let qx = fbm(x, y, seed, octaves);
    let qy = fbm(x + 5.2, y + 1.3, seed.wrapping_add(1), octaves);

    fbm(x + warp * qx, y + warp * qy, seed.wrapping_add(2), octaves)
}

/// Generate a procedural image as an `ImageBuffer` with the given dimensions and seed.
/// The seed selects the color palette and drives the noise pattern.
pub fn generate_image(width: u32, height: u32, seed: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let pal_idx = (seed as usize) % PALETTES.len();
    let pal = &PALETTES[pal_idx];

    let scale = 3.5;
    let warp = 2.0;
    let octaves = 3;

    ImageBuffer::from_fn(width, height, |px, py| {
        let x = (px as f64 / width as f64) * scale;
        let y = (py as f64 / height as f64) * scale;

        let v = warped_fbm(x, y, seed, warp, octaves);
        let t = (v * 0.5 + 0.5).clamp(0.0, 1.0);
        let [r, g, b] = palette_color(pal, t);
        Rgb([r, g, b])
    })
}

/// Generate a procedural image and return it as PNG bytes.
pub fn generate_png(width: u32, height: u32, seed: u32) -> Vec<u8> {
    let img = generate_image(width, height, seed);
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), image::ImageFormat::Png)
        .expect("PNG encoding failed");
    bytes
}

/// Generate a procedural image and return it as a base64 `data:` URI suitable for `<img src=...>`.
/// Results are cached — repeated calls with the same arguments are free.
pub fn generate_data_url(width: u32, height: u32, seed: u32) -> String {
    let key = (width, height, seed);
    if let Some(cached) = CACHE.lock().unwrap().get(&key) {
        return cached.clone();
    }

    use image::codecs::png::{CompressionType, FilterType, PngEncoder};
    use image::ImageEncoder;

    let img = generate_image(width, height, seed);
    let mut bytes: Vec<u8> = Vec::new();
    let encoder = PngEncoder::new_with_quality(
        Cursor::new(&mut bytes),
        CompressionType::Fast,
        FilterType::Sub,
    );
    encoder
        .write_image(img.as_raw(), width, height, image::ExtendedColorType::Rgb8)
        .expect("PNG encoding failed");

    let b64 = base64_encode(&bytes);
    let url = format!("data:image/png;base64,{b64}");
    CACHE.lock().unwrap().insert(key, url.clone());
    url
}

fn base64_encode(data: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}
