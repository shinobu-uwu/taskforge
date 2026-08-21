use std::{fmt, time::Duration};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
#[serde(default)]
pub struct Config {
    pub theme: Theme,
    pub expanded_sidebar: bool,
    pub process_cpu_display_mode: ProcessCpuDisplayMode,
    pub refresh_rate: RefreshInterval,
}

#[derive(Debug, Clone)]
pub enum ConfigChange {
    Theme(Theme),
    ExpandedSidebar(bool),
    ProcessCpuDisplayMode(ProcessCpuDisplayMode),
    RefreshRate(RefreshInterval),
}

impl Config {
    pub fn apply(&mut self, change: ConfigChange) {
        match change {
            ConfigChange::Theme(theme) => self.theme = theme,
            ConfigChange::ExpandedSidebar(expanded) => self.expanded_sidebar = expanded,
            ConfigChange::ProcessCpuDisplayMode(mode) => self.process_cpu_display_mode = mode,
            ConfigChange::RefreshRate(duration) => self.refresh_rate = duration,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ProcessCpuDisplayMode {
    #[default]
    TotalCapacity,
    PerCore,
}

impl ProcessCpuDisplayMode {
    pub const ALL: [Self; 2] = [Self::TotalCapacity, Self::PerCore];
}

impl fmt::Display for ProcessCpuDisplayMode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::TotalCapacity => "System",
            Self::PerCore => "Per-core",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct RefreshInterval(Duration);

impl RefreshInterval {
    pub const ALL: [RefreshInterval; 4] = [
        RefreshInterval(Duration::from_millis(500)),
        RefreshInterval(Duration::from_secs(1)),
        RefreshInterval(Duration::from_secs(2)),
        RefreshInterval(Duration::from_secs(5)),
    ];

    pub fn duration(&self) -> Duration {
        self.0
    }
}

impl Default for RefreshInterval {
    fn default() -> Self {
        Self(Duration::from_millis(500))
    }
}

impl fmt::Display for RefreshInterval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs_f32();
        let suffix = match secs {
            0.0..2.0 => "second",
            _ => "seconds",
        };
        write!(f, "{secs} {suffix}",)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum Theme {
    #[default]
    Light,
    Dark,
    Dracula,
    Nord,
    SolarizedLight,
    SolarizedDark,
    GruvboxLight,
    GruvboxDark,
    CatppuccinLatte,
    CatppuccinFrappe,
    CatppuccinMacchiato,
    CatppuccinMocha,
    TokyoNight,
    TokyoNightStorm,
    TokyoNightLight,
    KanagawaWave,
    KanagawaDragon,
    KanagawaLotus,
    Moonfly,
    Nightfly,
    Oxocarbon,
    Ferra,
}

impl From<Theme> for iced::Theme {
    fn from(value: Theme) -> Self {
        match value {
            Theme::Light => iced::Theme::Light,
            Theme::Dark => iced::Theme::Dark,
            Theme::Dracula => iced::Theme::Dracula,
            Theme::Nord => iced::Theme::Nord,
            Theme::SolarizedLight => iced::Theme::SolarizedLight,
            Theme::SolarizedDark => iced::Theme::SolarizedDark,
            Theme::GruvboxLight => iced::Theme::GruvboxLight,
            Theme::GruvboxDark => iced::Theme::GruvboxDark,
            Theme::CatppuccinLatte => iced::Theme::CatppuccinLatte,
            Theme::CatppuccinFrappe => iced::Theme::CatppuccinFrappe,
            Theme::CatppuccinMacchiato => iced::Theme::CatppuccinMacchiato,
            Theme::CatppuccinMocha => iced::Theme::CatppuccinMocha,
            Theme::TokyoNight => iced::Theme::TokyoNight,
            Theme::TokyoNightStorm => iced::Theme::TokyoNightStorm,
            Theme::TokyoNightLight => iced::Theme::TokyoNightLight,
            Theme::KanagawaWave => iced::Theme::KanagawaWave,
            Theme::KanagawaDragon => iced::Theme::KanagawaDragon,
            Theme::KanagawaLotus => iced::Theme::KanagawaLotus,
            Theme::Moonfly => iced::Theme::Moonfly,
            Theme::Nightfly => iced::Theme::Nightfly,
            Theme::Oxocarbon => iced::Theme::Oxocarbon,
            Theme::Ferra => iced::Theme::Ferra,
        }
    }
}

impl std::fmt::Display for Theme {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Theme::Light => "Light",
            Theme::Dark => "Dark",
            Theme::Dracula => "Dracula",
            Theme::Nord => "Nord",
            Theme::SolarizedLight => "Solarized Light",
            Theme::SolarizedDark => "Solarized Dark",
            Theme::GruvboxLight => "Gruvbox Light",
            Theme::GruvboxDark => "Gruvbox Dark",
            Theme::CatppuccinLatte => "Catppuccin Latte",
            Theme::CatppuccinFrappe => "Catppuccin Frappe",
            Theme::CatppuccinMacchiato => "Catppuccin Macchiato",
            Theme::CatppuccinMocha => "Catppuccin Mocha",
            Theme::TokyoNight => "TokyoNight",
            Theme::TokyoNightStorm => "TokyoNight Storm",
            Theme::TokyoNightLight => "TokyoNight Light",
            Theme::KanagawaWave => "Kanagawa Wave",
            Theme::KanagawaDragon => "Kanagawa Dragon",
            Theme::KanagawaLotus => "Kanagawa Lotus",
            Theme::Moonfly => "Moonfly",
            Theme::Nightfly => "Nightfly",
            Theme::Oxocarbon => "Oxocarbon",
            Theme::Ferra => "Ferra",
        })
    }
}

impl Theme {
    pub const ALL: [Self; 22] = [
        Self::Light,
        Self::Dark,
        Self::Dracula,
        Self::Nord,
        Self::SolarizedLight,
        Self::SolarizedDark,
        Self::GruvboxLight,
        Self::GruvboxDark,
        Self::CatppuccinLatte,
        Self::CatppuccinFrappe,
        Self::CatppuccinMacchiato,
        Self::CatppuccinMocha,
        Self::TokyoNight,
        Self::TokyoNightStorm,
        Self::TokyoNightLight,
        Self::KanagawaWave,
        Self::KanagawaDragon,
        Self::KanagawaLotus,
        Self::Moonfly,
        Self::Nightfly,
        Self::Oxocarbon,
        Self::Ferra,
    ];
}
