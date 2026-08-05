use gpui::{Entity, ParentElement, Render, Styled, div, prelude::IntoElement};
use gpui_component::{StyledExt, white};

use crate::{
    history::History,
    widgets::chart::{cpu_chart, memory_chart},
};

pub struct ChartsView {
    cpu_history: Entity<History<f32>>,
    memory_history: Entity<History<u64>>,
}

impl ChartsView {
    pub fn new(cpu_history: Entity<History<f32>>, memory_history: Entity<History<u64>>) -> Self {
        Self {
            cpu_history,
            memory_history,
        }
    }
}

impl Render for ChartsView {
    fn render(
        &mut self,
        _window: &mut gpui::Window,
        cx: &mut gpui::prelude::Context<Self>,
    ) -> impl IntoElement {
        div()
            .flex()
            .flex_1()
            .v_flex()
            .p_4()
            .gap_4()
            .size_full()
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .size_48()
                    .border_1()
                    .border_color(white())
                    .rounded_sm()
                    .child(cpu_chart(&self.cpu_history, cx)),
            )
            .child(
                div()
                    .v_flex()
                    .gap_1()
                    .size_48()
                    .border_1()
                    .border_color(white())
                    .rounded_sm()
                    .child(memory_chart(&self.memory_history, cx)),
            )
    }
}
