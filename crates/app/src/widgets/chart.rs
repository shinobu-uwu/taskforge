use iced::{Color as IcedColor, Element, Fill, Theme};
use plotters::prelude::*;
use plotters_iced::{Chart, ChartBuilder, ChartWidget, DrawingBackend};
use system::{disk::DiskUsage, memory::Memory};

use crate::state::history::{HISTORY_LEN, History};

const HISTORY_END: i32 = HISTORY_LEN as i32;

pub fn cpu_chart<'a, Message: 'a>(
    history: &'a History<f32>,
    theme: &Theme,
) -> Element<'a, Message> {
    ChartWidget::new(CpuChart {
        history,
        colors: ChartColors::cpu(theme),
    })
    .width(Fill)
    .height(Fill)
    .into()
}

pub fn memory_chart<'a, Message: 'a>(
    history: &'a History<Memory>,
    total_memory: Memory,
    theme: &Theme,
) -> Element<'a, Message> {
    ChartWidget::new(MemoryChart {
        history,
        colors: ChartColors::memory(theme),
        total_memory,
    })
    .width(Fill)
    .height(Fill)
    .into()
}

pub fn disk_chart<'a, Message: 'a>(
    history: &'a History<DiskUsage>,
    theme: &Theme,
) -> Element<'a, Message> {
    ChartWidget::new(DiskChart {
        history,
        colors: DiskChartColors::from_theme(theme),
    })
    .width(Fill)
    .height(Fill)
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
    history: &'a History<f32>,
    colors: ChartColors,
}

impl<Message> Chart<Message> for CpuChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let Ok(mut chart) = builder
            .margin(12)
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(0..HISTORY_END, 0.0_f32..100.0_f32)
        else {
            return;
        };

        let _ = chart
            .configure_mesh()
            .disable_axes()
            .disable_x_mesh()
            .light_line_style(self.colors.grid.mix(0.1))
            .bold_line_style(self.colors.grid.mix(0.25))
            .draw();

        let offset = HISTORY_END - self.history.len() as i32;
        let points = self
            .history
            .iter()
            .enumerate()
            .map(|(index, value)| (offset + index as i32, *value));

        let _ = chart.draw_series(
            AreaSeries::new(points, 0.0, self.colors.series.mix(0.18))
                .border_style(self.colors.series.stroke_width(2)),
        );
    }
}

struct MemoryChart<'a> {
    history: &'a History<Memory>,
    total_memory: Memory,
    colors: ChartColors,
}

impl<Message> Chart<Message> for MemoryChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let Ok(mut chart) = builder
            .margin(12)
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(0..HISTORY_END, 0.0_f64..self.total_memory.as_gib_f64())
        else {
            return;
        };

        let _ = chart
            .configure_mesh()
            .disable_axes()
            .disable_x_mesh()
            .light_line_style(self.colors.grid.mix(0.1))
            .bold_line_style(self.colors.grid.mix(0.25))
            .draw();

        let offset = HISTORY_END - self.history.len() as i32;
        let points = self
            .history
            .iter()
            .enumerate()
            .map(|(index, memory)| (offset + index as i32, memory.as_gib_f64()));

        let _ = chart.draw_series(
            AreaSeries::new(points, 0.0, self.colors.series.mix(0.18))
                .border_style(self.colors.series.stroke_width(2)),
        );
    }
}

struct DiskChart<'a> {
    history: &'a History<DiskUsage>,
    colors: DiskChartColors,
}

impl<Message> Chart<Message> for DiskChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let upper_bound = self
            .history
            .iter()
            .flat_map(|usage| [usage.read_bytes, usage.written_bytes])
            .max()
            .map_or(1.0, |maximum| (maximum as f64 * 1.05).max(1.0));

        let Ok(mut chart) = builder
            .margin(12)
            .x_label_area_size(0)
            .y_label_area_size(0)
            .build_cartesian_2d(0..HISTORY_END, 0.0_f64..upper_bound)
        else {
            return;
        };

        let _ = chart
            .configure_mesh()
            .disable_axes()
            .disable_x_mesh()
            .light_line_style(self.colors.grid.mix(0.1))
            .bold_line_style(self.colors.grid.mix(0.25))
            .draw();

        let offset = HISTORY_END - self.history.len() as i32;
        let read_points = self
            .history
            .iter()
            .enumerate()
            .map(|(index, usage)| (offset + index as i32, usage.read_bytes as f64));
        let written_points = self
            .history
            .iter()
            .enumerate()
            .map(|(index, usage)| (offset + index as i32, usage.written_bytes as f64));

        let read_color = self.colors.read;
        let _ = chart.draw_series(LineSeries::new(read_points, read_color.stroke_width(2)));

        let written_color = self.colors.written;
        let _ = chart.draw_series(LineSeries::new(
            written_points,
            written_color.stroke_width(2),
        ));
    }
}

#[cfg(test)]
mod tests {
    use plotters::prelude::{BitMapBackend, ChartBuilder, IntoDrawingArea};
    use plotters_iced::Chart;

    use iced::Theme;
    use system::{disk::DiskUsage, memory::Memory};

    use super::{ChartColors, CpuChart, DiskChart, DiskChartColors, MemoryChart};
    use crate::state::history::History;

    #[test]
    fn charts_render_without_axes() {
        let cpu_history = History::new();
        let memory_history = History::new();
        let mut disk_history = History::new();
        disk_history.push_back(DiskUsage {
            read_bytes: 1024,
            written_bytes: 2048,
        });
        let total_memory = Memory::from_bytes(16 * 1024 * 1024 * 1024);

        let mut cpu_buffer = vec![0; 640 * 480 * 3];
        let cpu_root = BitMapBackend::with_buffer(&mut cpu_buffer, (640, 480)).into_drawing_area();
        <CpuChart<'_> as Chart<()>>::build_chart(
            &CpuChart {
                history: &cpu_history,
                colors: ChartColors::cpu(&Theme::Dark),
            },
            &(),
            ChartBuilder::on(&cpu_root),
        );

        let mut memory_buffer = vec![0; 640 * 480 * 3];
        let memory_root =
            BitMapBackend::with_buffer(&mut memory_buffer, (640, 480)).into_drawing_area();
        <MemoryChart<'_> as Chart<()>>::build_chart(
            &MemoryChart {
                history: &memory_history,
                colors: ChartColors::memory(&Theme::Dark),
                total_memory,
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
            },
            &(),
            ChartBuilder::on(&disk_root),
        );
    }
}
