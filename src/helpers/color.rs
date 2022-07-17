use anyhow::Error;
use anyhow::Result;
use rand::Rng;

pub fn parse_color(color: String) -> Result<i32> {
    match color.as_str().to_lowercase().trim() {
        "random" => Ok(-1),
        "teal" => Ok(1752220),
        "green" => Ok(3066993),
        "blue" => Ok(3447003),
        "purple" => Ok(10181046),
        "magenta" => Ok(15277667),
        "gold" => Ok(15844367),
        "orange" => Ok(15105570),
        "red" => Ok(15158332),
        "yellow" => Ok(16705372),
        "og blurple" => Ok(7506394),
        "blurple" => Ok(5793266),
        "dark theme" => Ok(3553599),
        _ => parse_hex_number(color),
    }
}

fn parse_hex_number(color: String) -> Result<i32> {
    if color.starts_with('#') {
        let hex = color.trim_start_matches('#');
        let int_value = i32::from_str_radix(hex, 16)?;
        return Ok(int_value);
    }
    Err(Error::msg("Failed to parse color input"))
}

pub fn gen_color() -> i32 {
    let mut rng = rand::thread_rng();
    let r = rng.gen_range(0..255);
    let g = rng.gen_range(0..255);
    let b = rng.gen_range(0..255);
    (r << 16) + (g << 8) + b
}
