//! Configuration type definitions.

use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Application configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct AppConfig {
    /// Name of the theme to use.
    pub theme: String,
    /// Size of the launcher panel (width, height) in pixels.
    /// Default: (600.0, 400.0)
    pub launcher_size: Option<(f32, f32)>,
    /// Optional explicit window buffer size (width, height).
    /// Overrides compositor-specific defaults to reduce VRAM usage.
    /// Only used when enable_backdrop is true.
    /// Must be >= launcher_size.
    pub window_size: Option<(f32, f32)>,
    /// Enable the transparent backdrop overlay.
    /// When false, the window is just the launcher panel with no click-outside behavior.
    /// Default: true
    pub enable_backdrop: bool,
    /// Automatically apply blur layer rules on Hyprland.
    pub hyprland_auto_blur: bool,
    /// Modules that are disabled (DEPRECATED: use combined_modules instead).
    pub disabled_modules: Option<HashSet<ConfigModule>>,
    /// Enable transparency of the window.
    pub enable_transparency: bool,
    /// List of search providers.
    pub search_providers: Option<Vec<ConfigSearchProvider>>,
    /// Default modes to cycle through with Ctrl+Tab (ordered).
    pub default_modes: Option<Vec<String>>,
    /// Modules to include in combined view (ordered).
    pub combined_modules: Option<Vec<ConfigModule>>,
}

impl AppConfig {
    /// Const default for static initialization.
    pub const fn default_const() -> Self {
        Self {
            theme: String::new(),
            launcher_size: None,
            window_size: None,
            enable_backdrop: true,
            hyprland_auto_blur: true,
            disabled_modules: None,
            enable_transparency: true,
            search_providers: None,
            default_modes: None,
            combined_modules: None,
        }
    }

    /// Get the launcher panel size, using default if not configured.
    pub fn get_launcher_size(&self) -> (f32, f32) {
        self.launcher_size.unwrap_or((600.0, 400.0))
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            launcher_size: None,
            window_size: None,
            enable_backdrop: true,
            hyprland_auto_blur: true,
            disabled_modules: None,
            enable_transparency: true,
            search_providers: Some(vec![
                ConfigSearchProvider {
                    name: "Google".to_string(),
                    trigger: "!g".to_string(),
                    url: "https://www.google.com/search?q={query}".to_string(),
                    icon: "magnifying-glass".to_string(),
                },
                ConfigSearchProvider {
                    name: "DuckDuckGo".to_string(),
                    trigger: "!d".to_string(),
                    url: "https://duckduckgo.com/?q={query}".to_string(),
                    icon: "globe".to_string(),
                },
                ConfigSearchProvider {
                    name: "Wikipedia".to_string(),
                    trigger: "!wiki".to_string(),
                    url: "https://en.wikipedia.org/wiki/Special:Search?search={query}".to_string(),
                    icon: "book-open".to_string(),
                },
                ConfigSearchProvider {
                    name: "YouTube".to_string(),
                    trigger: "!yt".to_string(),
                    url: "https://www.youtube.com/results?search_query={query}".to_string(),
                    icon: "youtube-logo".to_string(),
                },
            ]),
            default_modes: None,
            combined_modules: None,
        }
    }
}

/// Modules enum - configurable components of the launcher.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ConfigModule {
    Applications,
    Ai,
    Emojis,
    Calculator,
    Clipboard,
    Actions,
    Search,
    Themes,
    Windows,
}

impl ConfigModule {
    /// Returns all module variants in default order.
    pub fn all() -> Vec<ConfigModule> {
        vec![
            ConfigModule::Calculator,
            ConfigModule::Windows,
            ConfigModule::Emojis,
            ConfigModule::Clipboard,
            ConfigModule::Actions,
            ConfigModule::Themes,
            ConfigModule::Applications,
            ConfigModule::Ai,
            ConfigModule::Search,
        ]
    }
}

/// Launcher modes - determines what view is shown.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, clap::ValueEnum)]
#[serde(rename_all = "lowercase")]
pub enum LauncherMode {
    /// Combined mode: shows all enabled modules together.
    Combined,
    #[value(alias = "apps", alias = "app")]
    Applications,
    Ai,
    #[value(alias = "emoji")]
    Emojis,
    #[value(alias = "calc")]
    Calculator,
    Clipboard,
    #[value(alias = "action")]
    Actions,
    Search,
    #[value(alias = "theme")]
    Themes,
    #[value(alias = "window")]
    Windows,
}

impl LauncherMode {
    /// Parse a mode from a string name.
    pub fn parse_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "combined" => Some(Self::Combined),
            "applications" | "apps" | "app" => Some(Self::Applications),
            "ai" => Some(Self::Ai),
            "emojis" | "emoji" => Some(Self::Emojis),
            "calculator" | "calc" => Some(Self::Calculator),
            "clipboard" => Some(Self::Clipboard),
            "actions" | "action" => Some(Self::Actions),
            "search" => Some(Self::Search),
            "themes" | "theme" => Some(Self::Themes),
            "windows" | "window" => Some(Self::Windows),
            _ => None,
        }
    }

    /// Get the display name for this mode.
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::Combined => "Combined",
            Self::Applications => "Applications",
            Self::Ai => "AI",
            Self::Emojis => "Emojis",
            Self::Calculator => "Calculator",
            Self::Clipboard => "Clipboard",
            Self::Actions => "Actions",
            Self::Search => "Search",
            Self::Themes => "Themes",
            Self::Windows => "Windows",
        }
    }

    /// Convert a ConfigModule to a LauncherMode.
    pub fn from_module(module: &ConfigModule) -> Self {
        match module {
            ConfigModule::Applications => Self::Applications,
            ConfigModule::Ai => Self::Ai,
            ConfigModule::Emojis => Self::Emojis,
            ConfigModule::Calculator => Self::Calculator,
            ConfigModule::Clipboard => Self::Clipboard,
            ConfigModule::Actions => Self::Actions,
            ConfigModule::Search => Self::Search,
            ConfigModule::Themes => Self::Themes,
            ConfigModule::Windows => Self::Windows,
        }
    }

    /// Convert back to ConfigModule (None for Combined).
    pub fn to_module(&self) -> Option<ConfigModule> {
        match self {
            Self::Combined => None,
            Self::Applications => Some(ConfigModule::Applications),
            Self::Ai => Some(ConfigModule::Ai),
            Self::Emojis => Some(ConfigModule::Emojis),
            Self::Calculator => Some(ConfigModule::Calculator),
            Self::Clipboard => Some(ConfigModule::Clipboard),
            Self::Actions => Some(ConfigModule::Actions),
            Self::Search => Some(ConfigModule::Search),
            Self::Themes => Some(ConfigModule::Themes),
            Self::Windows => Some(ConfigModule::Windows),
        }
    }
}

/// Search providers config.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ConfigSearchProvider {
    /// Provider name.
    pub name: String,
    /// Trigger (e.g. "!br").
    pub trigger: String,
    /// Url containing {query}.
    pub url: String,
    /// Optional icon name (defaults to MagnifyingGlass).
    #[serde(default)]
    pub icon: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_module_all() {
        let all = ConfigModule::all();
        assert_eq!(all.len(), 9);
        assert!(all.contains(&ConfigModule::Applications));
        assert!(all.contains(&ConfigModule::Calculator));
        assert!(all.contains(&ConfigModule::Ai));
    }

    #[test]
    fn test_launcher_mode_parse_combined() {
        assert_eq!(
            LauncherMode::parse_str("combined"),
            Some(LauncherMode::Combined)
        );
        assert_eq!(
            LauncherMode::parse_str("COMBINED"),
            Some(LauncherMode::Combined)
        );
    }

    #[test]
    fn test_launcher_mode_parse_applications() {
        assert_eq!(
            LauncherMode::parse_str("applications"),
            Some(LauncherMode::Applications)
        );
        assert_eq!(
            LauncherMode::parse_str("apps"),
            Some(LauncherMode::Applications)
        );
        assert_eq!(
            LauncherMode::parse_str("app"),
            Some(LauncherMode::Applications)
        );
    }

    #[test]
    fn test_launcher_mode_parse_emojis() {
        assert_eq!(
            LauncherMode::parse_str("emojis"),
            Some(LauncherMode::Emojis)
        );
        assert_eq!(LauncherMode::parse_str("emoji"), Some(LauncherMode::Emojis));
    }

    #[test]
    fn test_launcher_mode_parse_calculator() {
        assert_eq!(
            LauncherMode::parse_str("calculator"),
            Some(LauncherMode::Calculator)
        );
        assert_eq!(
            LauncherMode::parse_str("calc"),
            Some(LauncherMode::Calculator)
        );
    }

    #[test]
    fn test_launcher_mode_parse_invalid() {
        assert_eq!(LauncherMode::parse_str("invalid"), None);
        assert_eq!(LauncherMode::parse_str(""), None);
    }

    #[test]
    fn test_launcher_mode_display_name() {
        assert_eq!(LauncherMode::Combined.display_name(), "Combined");
        assert_eq!(LauncherMode::Applications.display_name(), "Applications");
        assert_eq!(LauncherMode::Ai.display_name(), "AI");
    }

    #[test]
    fn test_launcher_mode_from_module() {
        assert_eq!(
            LauncherMode::from_module(&ConfigModule::Applications),
            LauncherMode::Applications
        );
        assert_eq!(
            LauncherMode::from_module(&ConfigModule::Ai),
            LauncherMode::Ai
        );
    }

    #[test]
    fn test_launcher_mode_to_module() {
        assert_eq!(LauncherMode::Combined.to_module(), None);
        assert_eq!(
            LauncherMode::Applications.to_module(),
            Some(ConfigModule::Applications)
        );
    }

    #[test]
    fn test_app_config_default() {
        let config = AppConfig::default();
        assert_eq!(config.theme, "default");
        assert!(config.launcher_size.is_none());
        assert_eq!(config.get_launcher_size(), (600.0, 400.0));
        assert!(config.enable_backdrop);
        assert!(config.hyprland_auto_blur);
        assert!(config.enable_transparency);
        assert!(config.search_providers.is_some());
    }

    #[test]
    fn test_app_config_const_default() {
        let config = AppConfig::default_const();
        // Const default has empty theme (can't create String in const context)
        assert!(config.theme.is_empty());
        assert!(config.launcher_size.is_none());
        assert!(config.enable_backdrop);
    }

    #[test]
    fn test_config_serialization() {
        let config = AppConfig {
            theme: "dark".to_string(),
            launcher_size: Some((800.0, 600.0)),
            ..AppConfig::default()
        };

        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        assert!(toml_str.contains("theme = \"dark\""));
        assert!(toml_str.contains("launcher_size = [800.0, 600.0]"));
    }

    #[test]
    fn test_config_deserialization() {
        let toml_str = r#"
            theme = "catppuccin"
            launcher_size = [700.0, 500.0]
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.theme, "catppuccin");
        assert_eq!(config.launcher_size, Some((700.0, 500.0)));
        assert_eq!(config.get_launcher_size(), (700.0, 500.0));
    }

    #[test]
    fn test_config_module_serde() {
        let toml_str = r#"
            combined_modules = ["applications", "calculator", "ai"]
        "#;

        #[derive(Deserialize)]
        struct TestConfig {
            combined_modules: Vec<ConfigModule>,
        }

        let config: TestConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.combined_modules.len(), 3);
        assert_eq!(config.combined_modules[0], ConfigModule::Applications);
        assert_eq!(config.combined_modules[1], ConfigModule::Calculator);
        assert_eq!(config.combined_modules[2], ConfigModule::Ai);
    }

    #[test]
    fn test_window_size_default_none() {
        let config = AppConfig::default();
        assert!(config.window_size.is_none());
    }

    #[test]
    fn test_window_size_deserialization() {
        let toml_str = r#"
            window_size = [1920.0, 1080.0]
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.window_size, Some((1920.0, 1080.0)));
    }

    #[test]
    fn test_window_size_serialization() {
        let config = AppConfig {
            window_size: Some((1920.0, 1080.0)),
            ..AppConfig::default()
        };

        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        assert!(toml_str.contains("window_size = [1920.0, 1080.0]"));
    }

    #[test]
    fn test_window_size_missing_uses_none() {
        let toml_str = r#"
            theme = "default"
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert!(config.window_size.is_none());
    }

    #[test]
    fn test_launcher_size_default() {
        let config = AppConfig::default();
        assert!(config.launcher_size.is_none());
        assert_eq!(config.get_launcher_size(), (600.0, 400.0));
    }

    #[test]
    fn test_launcher_size_deserialization() {
        let toml_str = r#"
            launcher_size = [800.0, 500.0]
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert_eq!(config.launcher_size, Some((800.0, 500.0)));
        assert_eq!(config.get_launcher_size(), (800.0, 500.0));
    }

    #[test]
    fn test_enable_backdrop_default_true() {
        let config = AppConfig::default();
        assert!(config.enable_backdrop);
    }

    #[test]
    fn test_enable_backdrop_deserialization() {
        let toml_str = r#"
            enable_backdrop = false
        "#;

        let config: AppConfig = toml::from_str(toml_str).expect("Failed to deserialize");
        assert!(!config.enable_backdrop);
    }

    #[test]
    fn test_enable_backdrop_serialization() {
        let config = AppConfig {
            enable_backdrop: false,
            ..AppConfig::default()
        };

        let toml_str = toml::to_string(&config).expect("Failed to serialize");
        assert!(toml_str.contains("enable_backdrop = false"));
    }
}
