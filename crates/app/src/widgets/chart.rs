use crate::history::History;
use gpui::{App, Entity, prelude::IntoElement};
use gpui_component::{ActiveTheme, chart::AreaChart};

#[derive(Debug, Clone)]
struct DataPoint {
    x: String,
    y: f64,
}

pub fn cpu_chart(history: &Entity<History<f32>>, cx: &mut App) -> impl IntoElement {
    let data: Vec<DataPoint> = history
        .read(cx)
        .iter()
        .copied()
        .enumerate()
        .map(DataPoint::from)
        .collect();

    AreaChart::new(data)
        .x(|d| d.x.clone())
        .y(|d| d.y)
        .x_axis(false)
        .stroke(cx.theme().green)
        .fill(cx.theme().green_light)
}

pub fn memory_chart(history: &Entity<History<u64>>, cx: &mut App) -> impl IntoElement {
    let data: Vec<DataPoint> = history
        .read(cx)
        .iter()
        .copied()
        .enumerate()
        .map(DataPoint::from)
        .collect();

    AreaChart::new(data)
        .x(|d| d.x.clone())
        .y(|d| d.y)
        .x_axis(false)
        .stroke(cx.theme().ring)
        .fill(cx.theme().ring)
}

impl From<(usize, f32)> for DataPoint {
    fn from((i, u): (usize, f32)) -> Self {
        DataPoint {
            x: i.to_string(),
            y: u as f64,
        }
    }
}

impl From<(usize, u64)> for DataPoint {
    fn from((i, u): (usize, u64)) -> Self {
        DataPoint {
            x: i.to_string(),
            y: u as f64,
        }
    }
}
