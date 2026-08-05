use gpui::{
    App, AppContext, Context, Entity, IntoElement, ParentElement, Render, Styled, Window, div,
    relative,
};
use gpui_component::{
    ActiveTheme, IndexPath, StyledExt,
    label::Label,
    list::{List, ListDelegate, ListItem, ListState},
    white,
};
use system::monitor::SystemSnapshot;

use crate::{
    history::History,
    widgets::{
        chart::{cpu_chart, memory_chart},
        icon::FontAwesomeIconName,
    },
};

#[derive(Debug)]
pub struct ProcessView {
    cpu_history: Entity<History<f32>>,
    memory_history: Entity<History<u64>>,
    list_state: Entity<ListState<ProcessListDelegate>>,
}

#[derive(Debug)]
pub struct ProcessListDelegate {
    snapshot: Entity<SystemSnapshot>,
    selected_index: Option<IndexPath>,
}

impl ProcessView {
    pub fn new(
        snapshot: Entity<SystemSnapshot>,
        cpu_history: Entity<History<f32>>,
        memory_history: Entity<History<u64>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let delegate = ProcessListDelegate {
            snapshot: snapshot.clone(),
            selected_index: None,
        };
        let list_state = cx.new(|cx| ListState::new(delegate, window, cx));

        Self {
            list_state,
            cpu_history,
            memory_history,
        }
    }
}

impl Render for ProcessView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .h_flex()
            .size_full()
            .child(
                div()
                    .flex_basis(relative(3. / 4.))
                    .size_full()
                    .child(List::new(&self.list_state)),
            )
            .child(
                div()
                    .v_flex()
                    .h_full()
                    .flex_basis(relative(1. / 4.))
                    .gap_4()
                    .p_4()
                    .border_l_1()
                    .border_color(white())
                    .child(Label::new("Performance").text_lg())
                    .child(
                        div()
                            .w_full()
                            .h_40()
                            .child(cpu_chart(&self.cpu_history, cx)),
                    )
                    .child(
                        div()
                            .w_full()
                            .h_40()
                            .child(memory_chart(&self.memory_history, cx)),
                    ),
            )
    }
}

impl ListDelegate for ProcessListDelegate {
    type Item = ListItem;

    fn items_count(&self, _section: usize, cx: &App) -> usize {
        self.snapshot.read(cx).processes.len()
    }

    fn render_item(
        &mut self,
        ix: IndexPath,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) -> Option<Self::Item> {
        let name = self.snapshot.read(cx).processes[ix.row].name.clone();

        Some(
            ListItem::new(ix)
                .border_b_1()
                .border_color(cx.theme().border.opacity(0.5))
                .py_4()
                .gap_2()
                .selected(Some(ix) == self.selected_index)
                .child(
                    div()
                        .h_flex()
                        .gap_2()
                        .child(FontAwesomeIconName::RegularHdd)
                        .child(Label::new(name)),
                ),
        )
    }

    fn set_selected_index(
        &mut self,
        ix: Option<IndexPath>,
        _window: &mut Window,
        cx: &mut Context<ListState<Self>>,
    ) {
        self.selected_index = ix;
        cx.notify();
    }
}
