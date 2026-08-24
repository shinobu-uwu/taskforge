use iced::{Color as IcedColor, Element, Fill, Length, Theme};
use plotters::chart::MeshStyle;
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use system::{disk::DiskUsage, memory::Memory};

use crate::state::history::{History, TimedSample};

const HISTORY_WINDOW_SECONDS: f32 = 60.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChartSettings {
    pub width: Length,
    pub height: Length,
    pub axes: bool,
    pub x_mesh: bool,
    pub y_mesh: bool,
    pub margin: u32,
    pub x_label_area_size: u32,
    pub y_label_area_size: u32,
    pub x_labels: usize,
    pub y_labels: usize,
    pub series_width: u32,
    pub area_opacity: f64,
    pub light_grid_opacity: f64,
    pub bold_grid_opacity: f64,
}

impl ChartSettings {
    /// A small chart without axes and with only horizontal guide lines.
    pub const fn compact() -> Self {
        Self {
            width: Fill,
            height: Fill,
            axes: false,
            x_mesh: false,
            y_mesh: true,
            margin: 12,
            x_label_area_size: 0,
            y_label_area_size: 0,
            x_labels: 0,
            y_labels: 0,
            series_width: 2,
            area_opacity: 0.18,
            light_grid_opacity: 0.1,
            bold_grid_opacity: 0.25,
        }
    }

    /// A large chart with axes, labels, and a grid in both directions.
    pub const fn detailed() -> Self {
        Self {
            axes: true,
            x_mesh: true,
            y_mesh: true,
            margin: 12,
            x_label_area_size: 32,
            y_label_area_size: 48,
            x_labels: 8,
            y_labels: 5,
            ..Self::compact()
        }
    }
}

impl Default for ChartSettings {
    fn default() -> Self {
        Self::detailed()
    }
}

pub fn cpu_chart<'a, Message: 'a>(
    history: &'a History<TimedSample<f32>>,
    theme: &Theme,
    settings: ChartSettings,
) -> Element<'a, Message> {
    ChartWidget::new(CpuChart {
        history,
        colors: ChartColors::cpu(theme),
        settings,
    })
    .width(settings.width)
    .height(settings.height)
    .into()
}

pub fn memory_chart<'a, Message: 'a>(
    history: &'a History<TimedSample<Memory>>,
    total_memory: Memory,
    theme: &Theme,
    settings: ChartSettings,
) -> Element<'a, Message> {
    ChartWidget::new(MemoryChart {
        history,
        colors: ChartColors::memory(theme),
        total_memory,
        settings,
    })
    .width(settings.width)
    .height(settings.height)
    .into()
}

pub fn disk_chart<'a, Message: 'a>(
    history: &'a History<TimedSample<DiskUsage>>,
    theme: &Theme,
    settings: ChartSettings,
) -> Element<'a, Message> {
    ChartWidget::new(DiskChart {
        history,
        colors: DiskChartColors::from_theme(theme),
        settings,
    })
    .width(settings.width)
    .height(settings.height)
    .into()
}

#[derive(Clone, Copy)]
struct ChartColors {
    series: RGBColor,
    grid: RGBColor,
}

#[derive(Clone, Copy)]
struct DiskChartColors {
    read: RGBColor,
    written: RGBColor,
    grid: RGBColor,
}

impl DiskChartColors {
    fn from_theme(theme: &Theme) -> Self {
        let palette = theme.palette();
        let text = rgb(palette.text);

        Self {
            read: rgb(palette.primary),
            written: rgb(palette.warning),
            grid: text,
        }
    }
}

impl ChartColors {
    fn cpu(theme: &Theme) -> Self {
        Self::from_theme(theme, theme.palette().primary)
    }

    fn memory(theme: &Theme) -> Self {
        Self::from_theme(theme, theme.palette().success)
    }

    fn from_theme(theme: &Theme, series: IcedColor) -> Self {
        let text = rgb(theme.palette().text);

        Self {
            series: rgb(series),
            grid: text,
        }
    }
}

fn rgb(color: IcedColor) -> RGBColor {
    let [red, green, blue, _alpha] = color.into_rgba8();
    RGBColor(red, green, blue)
}

struct CpuChart<'a> {
    history: &'a History<TimedSample<f32>>,
    colors: ChartColors,
    settings: ChartSettings,
}

impl<Message> Chart<Message> for CpuChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let Ok(mut chart) = builder
            .margin(self.settings.margin)
            .x_label_area_size(self.settings.x_label_area_size)
            .y_label_area_size(self.settings.y_label_area_size)
            .build_cartesian_2d(-HISTORY_WINDOW_SECONDS..0.0_f32, 0.0_f32..100.0_f32)
        else {
            return;
        };

        let mut mesh = chart.configure_mesh();
        configure_mesh(&mut mesh, self.settings);
        let _ = mesh
            .label_style((FontFamily::SansSerif, 16, &self.colors.grid))
            .x_label_formatter(&format_seconds)
            .y_label_formatter(&format_percent)
            .axis_style(self.colors.grid.mix(0.0))
            .light_line_style(self.colors.grid.mix(self.settings.light_grid_opacity))
            .bold_line_style(self.colors.grid.mix(self.settings.bold_grid_opacity))
            .draw();

        let points = chart_samples(self.history)
            .into_iter()
            .map(|(x, sample)| (x, sample.sample));

        let _ = chart.draw_series(
            AreaSeries::new(
                points,
                0.0,
                self.colors.series.mix(self.settings.area_opacity),
            )
            .border_style(self.colors.series.stroke_width(self.settings.series_width)),
        );
    }
}

struct MemoryChart<'a> {
    history: &'a History<TimedSample<Memory>>,
    total_memory: Memory,
    colors: ChartColors,
    settings: ChartSettings,
}

impl<Message> Chart<Message> for MemoryChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let Ok(mut chart) = builder
            .margin(self.settings.margin)
            .x_label_area_size(self.settings.x_label_area_size)
            .y_label_area_size(self.settings.y_label_area_size)
            .build_cartesian_2d(
                -HISTORY_WINDOW_SECONDS..0.0_f32,
                0.0_f64..self.total_memory.as_gib_f64(),
            )
        else {
            return;
        };

        let mut mesh = chart.configure_mesh();
        configure_mesh(&mut mesh, self.settings);
        let _ = mesh
            .label_style((FontFamily::SansSerif, 16, &self.colors.grid))
            .x_label_formatter(&format_seconds)
            .axis_style(self.colors.grid.mix(0.0))
            .light_line_style(self.colors.grid.mix(self.settings.light_grid_opacity))
            .bold_line_style(self.colors.grid.mix(self.settings.bold_grid_opacity))
            .draw();

        let points = chart_samples(self.history)
            .into_iter()
            .map(|(x, sample)| (x, sample.sample.as_gib_f64()));

        let _ = chart.draw_series(
            AreaSeries::new(
                points,
                0.0,
                self.colors.series.mix(self.settings.area_opacity),
            )
            .border_style(self.colors.series.stroke_width(self.settings.series_width)),
        );
    }
}

struct DiskChart<'a> {
    history: &'a History<TimedSample<DiskUsage>>,
    colors: DiskChartColors,
    settings: ChartSettings,
}

impl<Message> Chart<Message> for DiskChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let upper_bound = self
            .history
            .iter()
            .flat_map(|sample| [sample.sample.read_bytes, sample.sample.written_bytes])
            .max()
            .map_or(1.0, |maximum| (maximum as f64 * 1.05).max(1.0));

        let Ok(mut chart) = builder
            .margin(self.settings.margin)
            .x_label_area_size(self.settings.x_label_area_size)
            .y_label_area_size(self.settings.y_label_area_size)
            .build_cartesian_2d(-HISTORY_WINDOW_SECONDS..0.0_f32, 0.0_f64..upper_bound)
        else {
            return;
        };

        let mut mesh = chart.configure_mesh();
        configure_mesh(&mut mesh, self.settings);
        let _ = mesh
            .label_style((FontFamily::SansSerif, 16, &self.colors.grid))
            .x_label_formatter(&format_seconds)
            .axis_style(self.colors.grid.mix(0.0))
            .light_line_style(self.colors.grid.mix(self.settings.light_grid_opacity))
            .bold_line_style(self.colors.grid.mix(self.settings.bold_grid_opacity))
            .draw();

        let samples = chart_samples(self.history);
        let read_points = samples
            .iter()
            .map(|(x, sample)| (*x, sample.sample.read_bytes as f64));
        let written_points = samples
            .iter()
            .map(|(x, sample)| (*x, sample.sample.written_bytes as f64));

        let read_color = self.colors.read;
        let _ = chart.draw_series(LineSeries::new(
            read_points,
            read_color.stroke_width(self.settings.series_width),
        ));

        let written_color = self.colors.written;
        let _ = chart.draw_series(LineSeries::new(
            written_points,
            written_color.stroke_width(self.settings.series_width),
        ));
    }
}

fn chart_samples<T>(history: &History<TimedSample<T>>) -> Vec<(f32, &TimedSample<T>)> {
    let Some(latest) = history.iter().last().map(|sample| sample.captured_at) else {
        return Vec::new();
    };

    let mut anchor = None;
    let mut visible = Vec::with_capacity(history.len());

    for sample in history.iter() {
        let seconds_ago = latest
            .saturating_duration_since(sample.captured_at)
            .as_secs_f32();
        let x = -seconds_ago;

        if seconds_ago > HISTORY_WINDOW_SECONDS {
            anchor = Some((x, sample));
            continue;
        }

        if visible.is_empty()
            && let Some(anchor) = anchor.take()
        {
            visible.push(anchor);
        }

        visible.push((x, sample));
    }

    visible
}

fn format_seconds(seconds: &f32) -> String {
    format!("{:.0}s", seconds.abs())
}

fn format_percent(percent: &f32) -> String {
    format!("{percent:.0}%")
}

fn configure_mesh<DB, X, Y>(mesh: &mut MeshStyle<'_, '_, X, Y, DB>, settings: ChartSettings)
where
    DB: DrawingBackend,
    X: Ranged,
    Y: Ranged,
{
    if !settings.axes {
        mesh.disable_axes();
    } else {
        mesh.x_labels(settings.x_labels)
            .y_labels(settings.y_labels)
            // Plotters places labels at twice the tick size. The transparent
            // axis style hides the ticks while preserving this 4 px gap.
            .set_all_tick_mark_size(2);
    }

    if !settings.x_mesh {
        mesh.disable_x_mesh();
    }

    if !settings.y_mesh {
        mesh.disable_y_mesh();
    }
}

#[cfg(test)]
mod tests {
    use std::time::{Duration, Instant};

    use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea};
    use plotters_iced::Chart;

    use iced::Theme;
    use system::{disk::DiskUsage, memory::Memory};

    use super::{
        ChartColors, ChartSettings, CpuChart, DiskChart, DiskChartColors, MemoryChart,
        chart_samples, format_seconds,
    };
    use crate::state::history::{History, TimedSample};

    #[test]
    fn charts_render_with_compact_and_detailed_settings() {
        let now = Instant::now();
        let mut cpu_history = History::new();
        cpu_history.push_back(TimedSample::new(now - Duration::from_secs(30), 25.0));
        cpu_history.push_back(TimedSample::new(now, 50.0));

        let mut memory_history = History::new();
        memory_history.push_back(TimedSample::new(
            now,
            Memory::from_bytes(8 * 1024 * 1024 * 1024),
        ));

        let mut disk_history = History::new();
        disk_history.push_back(TimedSample::new(
            now,
            DiskUsage {
                read_bytes: 1024,
                written_bytes: 2048,
            },
        ));
        let total_memory = Memory::from_bytes(16 * 1024 * 1024 * 1024);

        let mut cpu_buffer = vec![0; 640 * 480 * 3];
        let cpu_root = BitMapBackend::with_buffer(&mut cpu_buffer, (640, 480)).into_drawing_area();
        <CpuChart<'_> as Chart<()>>::build_chart(
            &CpuChart {
                history: &cpu_history,
                colors: ChartColors::cpu(&Theme::Dark),
                settings: ChartSettings::compact(),
            },
            &(),
            ChartBuilder::on(&cpu_root),
        );

        let mut detailed_cpu_buffer = vec![0; 640 * 480 * 3];
        let detailed_cpu_root =
            BitMapBackend::with_buffer(&mut detailed_cpu_buffer, (640, 480)).into_drawing_area();
        <CpuChart<'_> as Chart<()>>::build_chart(
            &CpuChart {
                history: &cpu_history,
                colors: ChartColors::cpu(&Theme::Dark),
                settings: ChartSettings::detailed(),
            },
            &(),
            ChartBuilder::on(&detailed_cpu_root),
        );

        let mut memory_buffer = vec![0; 640 * 480 * 3];
        let memory_root =
            BitMapBackend::with_buffer(&mut memory_buffer, (640, 480)).into_drawing_area();
        <MemoryChart<'_> as Chart<()>>::build_chart(
            &MemoryChart {
                history: &memory_history,
                colors: ChartColors::memory(&Theme::Dark),
                total_memory,
                settings: ChartSettings::compact(),
            },
            &(),
            ChartBuilder::on(&memory_root),
        );

        let mut disk_buffer = vec![0; 640 * 480 * 3];
        let disk_root =
            BitMapBackend::with_buffer(&mut disk_buffer, (640, 480)).into_drawing_area();
        <DiskChart<'_> as Chart<()>>::build_chart(
            &DiskChart {
                history: &disk_history,
                colors: DiskChartColors::from_theme(&Theme::Dark),
                settings: ChartSettings::compact(),
            },
            &(),
            ChartBuilder::on(&disk_root),
        );
    }

    #[test]
    fn time_axis_keeps_one_sample_before_the_window() {
        let now = Instant::now();
        let mut history = History::new();
        history.push_back(TimedSample::new(now - Duration::from_secs(90), 1.0));
        history.push_back(TimedSample::new(now - Duration::from_secs(61), 2.0));
        history.push_back(TimedSample::new(now - Duration::from_secs(30), 3.0));
        history.push_back(TimedSample::new(now, 4.0));

        let points = chart_samples(&history)
            .into_iter()
            .map(|(x, sample)| (x, sample.sample))
            .collect::<Vec<_>>();

        assert_eq!(points, [(-61.0, 2.0), (-30.0, 3.0), (0.0, 4.0)]);

        let mut startup_history = History::new();
        startup_history.push_back(TimedSample::new(now - Duration::from_secs(30), 1.0));
        startup_history.push_back(TimedSample::new(now, 2.0));
        let startup_points = chart_samples(&startup_history)
            .into_iter()
            .map(|(x, sample)| (x, sample.sample))
            .collect::<Vec<_>>();

        assert_eq!(startup_points, [(-30.0, 1.0), (0.0, 2.0)]);
        assert_eq!(format_seconds(&-30.0), "30s");
        assert_eq!(format_seconds(&0.0), "0s");
    }
}
