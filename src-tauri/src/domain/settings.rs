use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppSettings {
    pub launch_at_login: bool,
    pub start_minimized: bool,
    pub show_dock_icon: bool,
    pub close_panel_when_unfocused: bool,
    pub menu_bar: MenuBarSettings,
    pub refresh: RefreshSettings,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MenuBarSettings {
    pub show_percentage: bool,
    pub displayed_usage: DisplayedUsage,
    pub warning_threshold: u8,
    pub critical_threshold: u8,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum DisplayedUsage {
    Highest,
    Codex,
    Antigravity,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshSettings {
    pub enabled: bool,
    pub interval_minutes: u16,
    pub refresh_after_wake: bool,
    pub refresh_when_popup_opens: bool,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            launch_at_login: false,
            start_minimized: true,
            show_dock_icon: false,
            close_panel_when_unfocused: true,
            menu_bar: MenuBarSettings {
                show_percentage: true,
                displayed_usage: DisplayedUsage::Highest,
                warning_threshold: 80,
                critical_threshold: 95,
            },
            refresh: RefreshSettings {
                enabled: true,
                interval_minutes: 5,
                refresh_after_wake: true,
                refresh_when_popup_opens: true,
            },
        }
    }
}
