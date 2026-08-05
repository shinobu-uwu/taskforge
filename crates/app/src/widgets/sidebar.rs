use gpui::{Context, EventEmitter, IntoElement, ParentElement, Render, Styled, Window};
use gpui_component::{
    Icon, Sizable,
    label::Label,
    sidebar::{Sidebar, SidebarHeader, SidebarMenu, SidebarMenuItem},
};

use crate::{state::CurrentView, widgets::icon::FontAwesomeIconName};

#[derive(Debug)]
pub struct SideBar {
    selected: CurrentView,
}

pub enum SideBarEvent {
    Selected(CurrentView),
}

impl EventEmitter<SideBarEvent> for SideBar {}

impl SideBar {
    pub fn new() -> Self {
        Self {
            selected: CurrentView::Processes,
        }
    }

    fn select(&mut self, view: CurrentView, cx: &mut Context<Self>) {
        self.selected = view;
        cx.emit(SideBarEvent::Selected(view));
        cx.notify();
    }
}

impl Render for SideBar {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("root-sidebar")
            .header(
                SidebarHeader::new()
                    .child(Icon::new(FontAwesomeIconName::SolidTasks).large())
                    .child(Label::new("taskforge").text_lg()),
            )
            .child(
                SidebarMenu::new()
                    .child(
                        SidebarMenuItem::new("Processes")
                            .icon(FontAwesomeIconName::RegularMoon)
                            .active(self.selected == CurrentView::Processes)
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.select(CurrentView::Processes, cx)
                            })),
                    )
                    .child(
                        SidebarMenuItem::new("Charts")
                            .icon(FontAwesomeIconName::RegularBarChart)
                            .active(self.selected == CurrentView::Charts)
                            .on_click(
                                cx.listener(|this, _, _, cx| this.select(CurrentView::Charts, cx)),
                            ),
                    ),
            )
    }
}
