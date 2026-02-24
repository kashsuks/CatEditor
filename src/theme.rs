use iced::Color;
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::{fs, str::FromStr};

use syntect::highlighting::{
    Color as SynColor, FontStyle, ScopeSelectors, StyleModifier, Theme as SynTheme, ThemeItem, ThemeSettings
};

pub const SIDEBAR_DEFAULT_WIDTH: f32 = 180.0;
pub const SIDEBAR_MIN_WIDTH: f32 = 100.0;
pub const SIDEBAR_MAX_WIDTH: f32 = 500.0;
pub const RESIZE_HIT_WIDTH: f32 = 12.0;
pub const ICON_SIZE: f32 = 16.0;
pub const INDENT_WIDTH: f32 = 16.0;
pub const BORDER_RADIUS: f32 = 14.0;
pub const BORDER_RADIUS_TAB: f32 = 10.0;

pub struct ThemeColors {
    pub bg_primary: Color,
    pub bg_secondary: Color,
    pub bg_editor: Color,
    pub bg_tab_active: Color,
    pub bg_tab_inactive: Color,
    pub bg_status_bar: Color,
    pub bg_tab_bar: Color,
    pub bg_hover: Color,
    pub bg_pressed: Color,
    pub bg_drag_handle: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_dim: Color,
    pub text_placeholder: Color,
    pub border_subtle: Color,
    pub border_very_subtle: Color,
    pub selection: Color,
    pub shadow_dark: Color,
    pub shadow_light: Color,
    pub syntax_theme: SynTheme,
}

const THEME_JSON_PATH: &str =
    "extensions/themes/sainnhe.gruvbox-material-6.5.2/themes/gruvbox-material-dark.json";

#[derive(Deserialize)]
#[serde(untagged)]
enum TokenScope{
    Single(String),
    Multiple(Vec<String>),
}

#[derive(Deserialize)]
struct TokenSettings {
    foreground: Option<String>,
    #[serde(rename = "fontStyle")]
    font_style: Option<String>,
    background: Option<String>,
}

#[derive(Deserialize)]
struct VscodeTokenColor {
    #[allow(dead_code)]
    name: Option<String>,
    scope: Option<TokenScope>,
    settings: TokenSettings,
}
#[derive(Deserialize)]
struct VscodeTheme {
    colors: VscodeColors,
    #[serde(rename = "tokenColors", default)]
    token_colors: Vec<VscodeTokenColor>,
}

fn hex_to_syn(hex: &str) -> Option<SynColor> {
    let hex = hex.trim_start_matches("#");
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).ok()?
    } else {
        255
    };
    Some(SynColor { r, g, b, a })
}

fn parse_font_style(s: &str) -> FontStyle {
    let mut style = FontStyle::empty();
    for part in s.split_whitespace() {
        match part {
            "bold" => style |= FontStyle::BOLD,
            "italic" => style |= FontStyle::ITALIC,
            "underline" => style |= FontStyle::UNDERLINE,
            _ => {}
        }
    }
    style
}

fn build_syntect_theme(
    token_colors: &[VscodeTokenColor],
    editor_bg: Option<&str>,
    editor_fg: Option<&str>,
) -> SynTheme {
    let settings = ThemeSettings {
        foreground: editor_fg.and_then(hex_to_syn),
        background: editor_bg.and_then(hex_to_syn),
        ..ThemeSettings::default()
    };

    let scopes: Vec<ThemeItem> = token_colors
        .iter()
        .filter_map(|tc| {
            let scope_str = match &tc.scope {
                Some(TokenScope::Single(s)) => s.clone(),
                Some(TokenScope::Multiple(v)) => v.join(", "),
                None => return None,
            };

            let scope = ScopeSelectors::from_str(&scope_str).ok()?;

            let style = StyleModifier {
                foreground: tc.settings.foreground.as_deref().and_then(hex_to_syn),
                background: tc.settings.background.as_deref().and_then(hex_to_syn),
                font_style: tc.settings.font_style.as_deref().map(parse_font_style),
            };
            Some(ThemeItem { scope, style })
        })
        .collect();
    SynTheme {
        name: None,
        author: None,
        settings,
        scopes,
    }
}

#[derive(Deserialize)]
struct VscodeColors {
    #[serde(rename = "editor.background")]
    editor_background: Option<String>,
    #[serde(rename = "editor.foreground")]
    editor_foreground: Option<String>,
    #[serde(rename = "editor.selectionBackground")]
    editor_selection: Option<String>,
    #[serde(rename = "editor.lineHighlightBackground")]
    line_highlight: Option<String>,
    #[serde(rename = "sideBar.background")]
    sidebar_background: Option<String>,
    #[serde(rename = "sideBar.foreground")]
    sidebar_foreground: Option<String>,
    #[serde(rename = "tab.activeBackground")]
    tab_active_bg: Option<String>,
    #[serde(rename = "tab.inactiveBackground")]
    tab_inactive_bg: Option<String>,
    #[serde(rename = "tab.activeForeground")]
    tab_active_fg: Option<String>,
    #[serde(rename = "tab.inactiveForeground")]
    tab_inactive_fg: Option<String>,
    #[serde(rename = "tab.border")]
    tab_border: Option<String>,
    #[serde(rename = "statusBar.background")]
    status_bar_bg: Option<String>,
    #[serde(rename = "list.hoverBackground")]
    list_hover_bg: Option<String>,
    foreground: Option<String>,
    #[serde(rename = "input.placeholderForeground")]
    placeholder_fg: Option<String>,
}

impl VscodeColors {
    fn color(&self, field: &Option<String>, fallback: Color) -> Color {
        field.as_deref().and_then(hex_to_color).unwrap_or(fallback)
    }
}

fn hex_to_color(hex: &str) -> Option<Color> {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
    let a = if hex.len() >= 8 {
        u8::from_str_radix(&hex[6..8], 16).ok()?
    } else {
        255
    };
    Some(Color::from_rgba8(r, g, b, a as f32 / 255.0))
}

fn load_theme() -> ThemeColors {
    let d = ThemeColors::default();
    let path = crate::resources::resource_dir().join(THEME_JSON_PATH);
    let Ok(json) = fs::read_to_string(&path) else { return d; };
    let Ok(theme): Result<VscodeTheme, _> = serde_json::from_str(&json) else { return d; };
    let c = &theme.colors;
    let syntax_theme = build_syntect_theme(
        &theme.token_colors,
        c.editor_background.as_deref(),
        c.editor_foreground.as_deref(),
    );

    ThemeColors {
        bg_primary:       c.color(&c.line_highlight, d.bg_primary),
        bg_secondary:     c.color(&c.sidebar_background, d.bg_secondary),
        bg_editor:        c.color(&c.editor_background, d.bg_editor),
        bg_tab_active:    c.color(&c.tab_active_bg, d.bg_tab_active),
        bg_tab_inactive:  c.color(&c.tab_inactive_bg, d.bg_tab_inactive),
        bg_status_bar:    c.color(&c.status_bar_bg, d.bg_status_bar),
        bg_tab_bar:       c.color(&c.tab_border, d.bg_tab_bar),
        bg_hover:         c.color(&c.list_hover_bg, d.bg_hover),
        text_primary:     c.color(&c.editor_foreground, d.text_primary),
        text_secondary:   c.color(&c.tab_active_fg, d.text_secondary),
        text_muted:       c.color(&c.sidebar_foreground, d.text_muted),
        text_dim:         c.color(&c.tab_inactive_fg, d.text_dim),
        text_placeholder: c.color(&c.placeholder_fg, d.text_placeholder),
        selection:        c.color(&c.editor_selection, d.selection),
        // No VSCode equivalents for these
        bg_pressed:       d.bg_pressed,
        bg_drag_handle:   d.bg_drag_handle,
        border_subtle:    d.border_subtle,
        border_very_subtle: d.border_very_subtle,
        shadow_dark:      d.shadow_dark,
        shadow_light:     d.shadow_light,
        syntax_theme,
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_primary:       Color::from_rgba(0.08, 0.08, 0.1, 0.95),
            bg_secondary:     Color::from_rgba(0.1, 0.1, 0.12, 0.95),
            bg_editor:        Color::from_rgba(0.06, 0.06, 0.08, 1.0),
            bg_tab_active:    Color::from_rgba(0.15, 0.15, 0.18, 1.0),
            bg_tab_inactive:  Color::from_rgba(0.1, 0.1, 0.12, 0.6),
            bg_status_bar:    Color::from_rgba(0.1, 0.1, 0.12, 0.6),
            bg_tab_bar:       Color::from_rgba(0.08, 0.08, 0.1, 0.8),
            bg_hover:         Color::from_rgba(1.0, 1.0, 1.0, 0.1),
            bg_pressed:       Color::from_rgba(1.0, 1.0, 1.0, 0.15),
            bg_drag_handle:   Color::from_rgba(1.0, 1.0, 1.0, 0.03),
            text_primary:     Color::from_rgb(0.9, 0.9, 0.9),
            text_secondary:   Color::from_rgb(0.8, 0.8, 0.8),
            text_muted:       Color::from_rgb(0.7, 0.7, 0.7),
            text_dim:         Color::from_rgb(0.5, 0.5, 0.5),
            text_placeholder: Color::from_rgb(0.4, 0.4, 0.4),
            border_subtle:    Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            border_very_subtle: Color::from_rgba(1.0, 1.0, 1.0, 0.03),
            selection:        Color::from_rgba(0.3, 0.5, 0.8, 0.4),
            shadow_dark:      Color::from_rgba(0.0, 0.0, 0.0, 0.3),
            shadow_light:     Color::from_rgba(1.0, 1.0, 1.0, 0.02),
            syntax_theme: SynTheme::default(),
        }
    }
}

pub static THEME: Lazy<ThemeColors> = Lazy::new(load_theme);
