use std::time::{self, Duration};

use anyhow::anyhow;
use palette::{FromColor, IntoColor, Lch};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::palette::base_palette::{BasePalette, PaletteColor};

pub fn optimize_base_palette(
    best_palette: &mut BasePalette,
    change_palette_element: &[PaletteColor],
    time_limit: u64,
    rng: &mut ThreadRng,
) -> anyhow::Result<()> {
    let time_limit = Duration::from_millis(time_limit);
    let start_temp = 15000.0f32;
    let end_temp = 0.0;

    let start = time::Instant::now();

    let mut count = 0;

    let mut now_palette = best_palette.clone();

    while start.elapsed() < time_limit {
        let next_palette = generate_base_palette(&now_palette, &change_palette_element, rng)?;
        let temp = start_temp
            + (end_temp - start_temp)
                * (start.elapsed().as_micros() as f32 / time_limit.as_micros() as f32);
        let diff = next_palette.score - now_palette.score;
        let r: f32 = rng.gen();
        if r < (diff / temp).exp() || now_palette.score < next_palette.score {
            now_palette = next_palette;
        }
        if best_palette.score < now_palette.score {
            *best_palette = now_palette.clone();
        }
        count += 1;
    }
    println!("loop count: {}, score: {}", count, best_palette.score);
    Ok(())
}

enum ChangeType {
    IncL,
    DecL,
    IncC,
    DecC,
    IncH,
    DecH,
}

impl ChangeType {
    fn random(rng: &mut ThreadRng) -> anyhow::Result<Self> {
        let n = rng.gen_range(0..6);
        match n {
            0 => Ok(Self::IncL),
            1 => Ok(Self::DecL),
            2 => Ok(Self::IncC),
            3 => Ok(Self::DecC),
            4 => Ok(Self::IncH),
            5 => Ok(Self::DecH),
            _ => Err(anyhow!("Not found change type")),
        }
    }
}

pub fn generate_base_palette(
    palette: &BasePalette,
    change_palette_element: &[PaletteColor],
    rng: &mut ThreadRng,
) -> anyhow::Result<BasePalette> {
    let select = change_palette_element
        .choose(rng)
        .ok_or(anyhow!("Failed to select a color to change"))?;
    let change_type = ChangeType::random(rng)?;
    let rgb = palette.get_color(*select);
    let mut lch = Lch::from_color(rgb);
    match change_type {
        ChangeType::IncL => lch.l = (lch.l + 2.0).clamp(0.0, 100.0),
        ChangeType::DecL => lch.l = (lch.l - 2.0).clamp(0.0, 100.0),
        ChangeType::IncC => lch.chroma = (lch.chroma + 2.0).clamp(0.0, 100.0),
        ChangeType::DecC => lch.chroma = (lch.chroma - 2.0).clamp(0.0, 100.0),
        ChangeType::IncH => lch.hue += 3.0,
        ChangeType::DecH => lch.hue -= 3.0,
    }
    let mut palette = palette.clone();
    palette.update_color(*select, lch.into_color());
    Ok(palette)
}
