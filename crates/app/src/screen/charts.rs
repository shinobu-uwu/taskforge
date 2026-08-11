use circular_buffer::FixedCircularBuffer;
use iced::{
    Element, Fill,
    widget::{column, text},
};
use plotters::prelude::{
    AreaSeries, ChartBuilder, Color, DrawingBackend, IntoFont, RGBColor, ShapeStyle,
};
use plotters_iced::{Chart, ChartWidget};

const HISTORY_LENGTH: i32 = 128;
const BYTES_PER_GIBIBYTE: f64 = 1024.0 * 1024.0 * 1024.0;
const CPU_COLOR: RGBColor = RGBColor(52, 152, 219);
const MEMORY_COLOR: RGBColor = RGBColor(46, 204, 113);
const GRID_COLOR: RGBColor = RGBColor(120, 120, 120);
const LABEL_COLOR: RGBColor = RGBColor(160, 160, 160);

#[derive(Debug, Default)]
pub(crate) struct ChartsScreen;

#[derive(Debug, Clone)]
pub(crate) enum Message {}

impl ChartsScreen {
    pub(crate) fn update(&mut self, message: Message) {
        match message {}
    }

    pub(crate) fn view<'a>(
        &self,
        cpu_history: &'a FixedCircularBuffer<f32, 128>,
        memory_history: &'a FixedCircularBuffer<u64, 128>,
        total_memory: u64,
    ) -> Element<'a, Message> {
        column![]
            .spacing(12)
            .padding(16)
            .width(Fill)
            .height(Fill)
            .into()
    }
}

struct CpuChart<'a> {
    history: &'a FixedCircularBuffer<f32, 128>,
}

impl Chart<Message> for CpuChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let Ok(mut chart) = builder
            .margin(12)
            .x_label_area_size(0)
            .y_label_area_size(42)
            .build_cartesian_2d(0..HISTORY_LENGTH, 0.0_f32..100.0_f32)
        else {
            return;
        };

        let _ = chart
            .configure_mesh()
            .disable_x_mesh()
            .x_labels(0)
            .y_labels(5)
            .bold_line_style(GRID_COLOR.mix(0.25))
            .light_line_style(GRID_COLOR.mix(0.1))
            .axis_style(ShapeStyle::from(GRID_COLOR.mix(0.5)))
            .label_style(("sans-serif", 12).into_font().color(&LABEL_COLOR))
            .y_label_formatter(&|value| format!("{value:.0}%"))
            .draw();

        let _ = chart.draw_series(
            AreaSeries::new(history_points(self.history), 0.0, CPU_COLOR.mix(0.18))
                .border_style(ShapeStyle::from(CPU_COLOR).stroke_width(2)),
        );
    }
}

struct MemoryChart<'a> {
    history: &'a FixedCircularBuffer<u64, 128>,
    total_memory: u64,
}

impl Chart<Message> for MemoryChart<'_> {
    type State = ();

    fn build_chart<DB: DrawingBackend>(&self, _state: &Self::State, mut builder: ChartBuilder<DB>) {
        let upper_bound = (self.total_memory as f64 / BYTES_PER_GIBIBYTE).max(1.0);
        let Ok(mut chart) = builder
            .margin(12)
            .x_label_area_size(0)
            .y_label_area_size(64)
            .build_cartesian_2d(0..HISTORY_LENGTH, 0.0_f64..upper_bound)
        else {
            return;
        };

        let _ = chart
            .configure_mesh()
            .disable_x_mesh()
            .x_labels(0)
            .y_labels(5)
            .bold_line_style(GRID_COLOR.mix(0.25))
            .light_line_style(GRID_COLOR.mix(0.1))
            .axis_style(ShapeStyle::from(GRID_COLOR.mix(0.5)))
            .label_style(("sans-serif", 12).into_font().color(&LABEL_COLOR))
            .y_label_formatter(&|value| format!("{value:.1} GiB"))
            .draw();

        let _ = chart.draw_series(
            AreaSeries::new(
                memory_history_points(self.history),
                0.0,
                MEMORY_COLOR.mix(0.18),
            )
            .border_style(ShapeStyle::from(MEMORY_COLOR).stroke_width(2)),
        );
    }
}

fn history_points<T>(history: &FixedCircularBuffer<T, 128>) -> impl Iterator<Item = (i32, T)> + '_
where
    T: Copy,
{
    let start = HISTORY_LENGTH - history.len() as i32;

    history
        .iter()
        .copied()
        .enumerate()
        .map(move |(index, value)| (start + index as i32, value))
}

fn memory_history_points(
    history: &FixedCircularBuffer<u64, 128>,
) -> impl Iterator<Item = (i32, f64)> + '_ {
    let start = HISTORY_LENGTH - history.len() as i32;

    history
        .iter()
        .copied()
        .enumerate()
        .map(move |(index, bytes)| (start + index as i32, bytes as f64 / BYTES_PER_GIBIBYTE))
}

fn format_memory(bytes: u64) -> String {
    format!("{:.1} GiB", bytes as f64 / BYTES_PER_GIBIBYTE)
}
