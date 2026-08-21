use iced::{
    Alignment::Center,
    Element, Fill, Font, border,
    widget::{Text, column, container, grid, pick_list, row, rule, space, svg, text, toggler},
};
use iced_fonts::lucide;

use crate::config::{Config, ConfigChange, ProcessCpuDisplayMode, RefreshInterval, Theme};

#[derive(Debug, Default)]
pub struct SettingsScreen;

#[derive(Debug, Clone)]
pub enum Message {
    ConfigChanged(ConfigChange),
}

impl SettingsScreen {
    pub fn view<'a>(&'a self, config: &'a Config) -> Element<'a, Message> {
        container(
            grid![
                self.appearance_section(config),
                self.process_section(config),
                self.about_section(),
            ]
            .columns(2)
            .height(Fill)
            .spacing(8),
        )
        .padding(8)
        .width(Fill)
        .height(Fill)
        .into()
    }

    fn appearance_section(&self, config: &Config) -> Element<'_, Message> {
        self.section(
            column![
                self.title(lucide::brush(), "Appearance"),
                self.entry(
                    "Expanded sidebar:",
                    toggler(config.expanded_sidebar)
                        .on_toggle(|expanded| Message::ConfigChanged(
                            ConfigChange::ExpandedSidebar(expanded)
                        ))
                        .size(24)
                        .into()
                ),
                rule::horizontal(1),
                self.entry(
                    "Theme:",
                    pick_list(Theme::ALL, Some(config.theme.clone()), |t| {
                        Message::ConfigChanged(ConfigChange::Theme(t))
                    })
                    .into()
                ),
            ]
            .spacing(8)
            .padding(8)
            .into(),
        )
    }

    fn process_section<'a>(&'a self, config: &Config) -> Element<'a, Message> {
        self.section(
            column![
                self.title(lucide::trending_up(), "Process monitoring"),
                self.entry(
                    "Process CPU usage:",
                    pick_list(
                        ProcessCpuDisplayMode::ALL,
                        Some(config.process_cpu_display_mode),
                        |mode| Message::ConfigChanged(ConfigChange::ProcessCpuDisplayMode(mode)),
                    )
                    .into()
                ),
                rule::horizontal(1),
                self.entry(
                    "Refresh Interval",
                    pick_list(RefreshInterval::ALL, Some(config.refresh_rate), |r| {
                        Message::ConfigChanged(ConfigChange::RefreshRate(r))
                    })
                    .into()
                ),
            ]
            .spacing(8)
            .padding(8)
            .into(),
        )
    }

    fn about_section(&self) -> Element<'_, Message> {
        let (platform_name, platform_icon) = platform();

        self.section(
            column![
                self.title(lucide::info(), "Taskforge"),
                self.entry("Version", text(env!("CARGO_PKG_VERSION")).into()),
                rule::horizontal(1),
                self.entry(
                    "Platform",
                    row![platform_icon, text(platform_name)]
                        .spacing(8)
                        .align_y(Center)
                        .into()
                ),
            ]
            .spacing(8)
            .padding(8)
            .into(),
        )
    }

    fn title<'a>(&'a self, icon: Text<'a>, label: &'static str) -> Element<'a, Message> {
        row![
            icon.style(|theme: &iced::Theme| text::Style {
                color: Some(theme.palette().primary)
            })
            .size(28),
            text(label)
                .style(|theme: &iced::Theme| text::Style {
                    color: Some(theme.palette().primary)
                })
                .font(Font {
                    weight: iced::font::Weight::Semibold,
                    ..Default::default()
                })
                .size(20)
        ]
        .spacing(8)
        .padding(4)
        .width(Fill)
        .align_y(Center)
        .into()
    }

    fn entry<'a>(
        &'a self,
        label: &'static str,
        element: Element<'a, Message>,
    ) -> Element<'a, Message> {
        row![text(label), space::horizontal(), element]
            .align_y(Center)
            .padding(8)
            .into()
    }

    fn section<'a>(&'a self, content: Element<'a, Message>) -> Element<'a, Message> {
        container(content)
            .padding(8)
            .style(|theme| container::Style {
                border: border::rounded(8)
                    .color(theme.extended_palette().background.stronger.color)
                    .width(1),
                ..Default::default()
            })
            .into()
    }
}

#[cfg(target_os = "windows")]
fn platform() -> (&'static str, Element<'static, Message>) {
    platform_svg(
        "Windows",
        include_bytes!("../../assets/windows-brands-solid-full.svg"),
    )
}

#[cfg(target_os = "linux")]
fn platform() -> (&'static str, Element<'static, Message>) {
    platform_svg(
        "Linux",
        include_bytes!("../../assets/linux-brands-solid-full.svg"),
    )
}

#[cfg(target_os = "macos")]
fn platform() -> (&'static str, Element<'static, Message>) {
    platform_svg(
        "macOS",
        include_bytes!("../../assets/apple-brands-solid-full.svg"),
    )
}

#[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
fn platform() -> (&'static str, Element<'static, Message>) {
    (std::env::consts::OS, lucide::monitor().into())
}

fn platform_svg(
    name: &'static str,
    bytes: &'static [u8],
) -> (&'static str, Element<'static, Message>) {
    let icon = svg(svg::Handle::from_memory(bytes))
        .width(24)
        .height(24)
        .style(|theme: &iced::Theme, _status| svg::Style {
            color: Some(theme.palette().text),
        });

    (name, icon.into())
}
