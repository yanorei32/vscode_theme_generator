use std::time::{self, Duration};

use palette::{FromColor, IntoColor, Lch};
use rand::{rngs::ThreadRng, seq::SliceRandom, Rng};

use crate::{
    model::{Color, ColorMap},
    util::{ColorMapExt, SrgbExt},
};

trait OptimizerColorMapExt {
    fn calc_score(&self) -> f32;
}

impl OptimizerColorMapExt for ColorMap {
    fn calc_score(&self) -> f32 {
        // TODO: これが期待通りに動いているか確認する
        use itertools::Itertools;

        let base_score: f32 = enum_iterator::all::<Color>()
            .tuple_combinations()
            .map(|(a, b)| {
                let diff = self[a].compare(&self[b]);
                let p = if a.is_bg_color() { 42.0 } else { 21.0 };

                (diff / p).log10().min(0.0) * 1000000.0
            })
            .sum();

        let (l_ave, chroma_ave) = self.fg_color_avg_luminouse_chroma();

        let (l_point, chroma_point) =
            self.iter()
                .filter(|(k, _)| !k.is_bg_color())
                .fold((0.0, 0.0), |sum, (_, value)| {
                    let lch = Lch::from_color(*value);
                    (
                        sum.0 + ((lch.l - l_ave).abs() - 5.0).max(0.0).powf(2.0),
                        sum.1 + ((lch.chroma - chroma_ave).abs() - 5.0).max(0.0).powf(2.0),
                    )
                });

        base_score - l_point * 10000000.0 + chroma_point * 10000000.0
    }
}

#[derive(Debug, Clone)]
struct ScoredColorMap {
    color_map: ColorMap,
    score: f32,
}

impl ScoredColorMap {
    fn from(color_map: ColorMap) -> Self {
        Self {
            score: color_map.calc_score(),
            color_map,
        }
    }

    fn score(&self) -> f32 {
        self.score
    }

    fn take(self) -> ColorMap {
        self.color_map
    }
}

pub fn optimize_color_map(
    color_map: &ColorMap,
    candidates: &[Color],
    time_limit_ms: u64,
    rng: &mut ThreadRng,
) -> ColorMap {
    // if candidates is empty, do nothing
    if candidates.is_empty() {
        return color_map.clone();
    }

    let time_limit = Duration::from_millis(time_limit_ms);
    let start_temp = 15000.0f32;
    let end_temp = 0.0;

    let start = time::Instant::now();

    let mut count = 0;

    let color_map = ScoredColorMap::from(color_map.clone());
    let mut best = color_map.clone();
    let mut cursor = color_map.clone();

    while start.elapsed() < time_limit {
        let next = random_edit_one_color_of(&cursor, candidates, rng);

        let temp = start_temp
            + (end_temp - start_temp)
                * (start.elapsed().as_micros() as f32 / time_limit.as_micros() as f32);

        let diff = next.score() - cursor.score();
        let r: f32 = rng.gen();

        if r < (diff / temp).exp() || cursor.score() < next.score() {
            cursor = next;
        }

        if best.score() < cursor.score() {
            best = cursor.clone();
        }

        count += 1;
    }

    println!("loop count: {}, score: {}", count, best.score());
    best.take()
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
            _ => unreachable!(),
        }
    }
}

fn random_edit_one_color_of(
    color_map: &ScoredColorMap,
    candidates: &[Color],
    rng: &mut ThreadRng,
) -> ScoredColorMap {
    // if candidates is empty, do nothing
    let Some(target_color) = candidates.choose(rng) else {
        return color_map.clone();
    };

    let mut color_map = color_map.clone().take();

    let change_type = ChangeType::random(rng).unwrap();
    let rgb = color_map[*target_color];
    let mut lch = Lch::from_color(rgb);

    match change_type {
        ChangeType::IncL => lch.l = (lch.l + 2.0).clamp(0.0, 100.0),
        ChangeType::DecL => lch.l = (lch.l - 2.0).clamp(0.0, 100.0),
        ChangeType::IncC => lch.chroma = (lch.chroma + 2.0).clamp(0.0, 100.0),
        ChangeType::DecC => lch.chroma = (lch.chroma - 2.0).clamp(0.0, 100.0),
        ChangeType::IncH => lch.hue += 3.0,
        ChangeType::DecH => lch.hue -= 3.0,
    }

    color_map[target_color] = lch.into_color();

    ScoredColorMap::from(color_map)
}
