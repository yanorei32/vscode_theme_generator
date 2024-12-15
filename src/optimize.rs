use std::time::{self, Duration};

use itertools::Itertools;
use palette::{FromColor, IntoColor, Lch};
use rand::seq::SliceRandom;

use crate::{
    model::{Color, ColorMap},
    palette::BasePalette,
    util::{ColorMapExt, SrgbExt},
};

pub trait OptimizerExt {
    fn optimize<R: rand::Rng>(self, targets: &[Color], rng: &mut R) -> Self;
}

// TODO: BasePaletteに依存しない操作
impl OptimizerExt for BasePalette {
    fn optimize<R: rand::Rng>(self, targets: &[Color], rng: &mut R) -> Self {
        let (actual_mode, color_map) = self.take();

        let color_map = optimize_color_map(&color_map, targets, 100, rng);

        Self::new(actual_mode, color_map)
    }
}

trait OptimizerColorMapExt {
    fn calc_score(&self) -> f32;
}

impl OptimizerColorMapExt for ColorMap {
    fn calc_score(&self) -> f32 {
        // TODO: これが期待通りに動いているか確認する
        let base_score: f32 = enum_iterator::all::<Color>()
            .tuple_combinations()
            .map(|(a, b)| {
                let diff = self[a].compare(&self[b]);
                let p = if a.is_bg_color() { 42.0 } else { 21.0 };

                (diff / p).log10().min(0.0) * 1000000.0
            })
            .sum();

        let (l_ave, chroma_ave) = self.fg_color_avg_luminouse_chroma();

        let (l_point, chroma_point) = self
            .iter()
            .filter_map(|(k, v)| (!k.is_bg_color()).then_some(v))
            .map(|c| Lch::from_color(*c))
            .map(|c| (c.l, c.chroma))
            // absolute differencial
            .map(|(l, chroma)| ((l - l_ave).abs(), (chroma - chroma_ave).abs()))
            // scoring
            .map(|(l, chroma)| {
                (
                    (l - 5.0).max(0.0).powf(2.0),
                    (chroma - 5.0).max(0.0).powf(2.0),
                )
            })
            // sum
            .fold((0.0, 0.0), |(l_acc, chroma_acc), (l, chroma)| {
                (l_acc + l, chroma_acc + chroma)
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

pub fn optimize_color_map<R: rand::Rng>(
    color_map: &ColorMap,
    candidates: &[Color],
    time_limit_ms: u64,
    rng: &mut R,
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

fn random_edit_one_color_of<R: rand::Rng>(
    color_map: &ScoredColorMap,
    candidates: &[Color],
    rng: &mut R,
) -> ScoredColorMap {
    let color_map = color_map.clone();

    // if candidates is empty, do nothing
    let Some(target_choice) = candidates.choose(rng) else {
        return color_map;
    };

    let mut color_map = color_map.take();

    color_map[target_choice] = Op::choice(rng).apply(color_map[target_choice]);

    ScoredColorMap::from(color_map)
}

enum Op {
    IncL,
    DecL,
    IncC,
    DecC,
    IncH,
    DecH,
}

impl Op {
    fn choice<R: rand::Rng>(rng: &mut R) -> Self {
        let n = rng.gen_range(0..6);
        match n {
            0 => Self::IncL,
            1 => Self::DecL,
            2 => Self::IncC,
            3 => Self::DecC,
            4 => Self::IncH,
            5 => Self::DecH,
            _ => unreachable!(),
        }
    }

    fn apply<I: IntoColor<Lch>, O: FromColor<Lch>>(&self, i: I) -> O {
        let mut lch: Lch = i.into_color();

        match self {
            Self::IncL => lch.l = (lch.l + 2.0).clamp(0.0, 100.0),
            Self::DecL => lch.l = (lch.l - 2.0).clamp(0.0, 100.0),
            Self::IncC => lch.chroma = (lch.chroma + 2.0).clamp(0.0, 100.0),
            Self::DecC => lch.chroma = (lch.chroma - 2.0).clamp(0.0, 100.0),
            Self::IncH => lch.hue += 3.0,
            Self::DecH => lch.hue -= 3.0,
        };

        lch.into_color()
    }
}
