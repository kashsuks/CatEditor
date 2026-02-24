use iced::Color;
use once_cell::sync::Lazy;
use std::str::FromStr;

use syntect::highlighting::{
    Color as SynColor, FontStyle, ScopeSelectors, StyleModifier, Theme as SynTheme, ThemeItem,
    ThemeSettings,
};

// ── Layout constants ────────────────────────────────────────────────────────
pub const SIDEBAR_DEFAULT_WIDTH: f32 = 180.0;
pub const SIDEBAR_MIN_WIDTH: f32 = 100.0;
pub const SIDEBAR_MAX_WIDTH: f32 = 500.0;
pub const RESIZE_HIT_WIDTH: f32 = 12.0;
pub const ICON_SIZE: f32 = 16.0;
pub const INDENT_WIDTH: f32 = 16.0;
pub const BORDER_RADIUS: f32 = 14.0;
pub const BORDER_RADIUS_TAB: f32 = 10.0;

// ═══════════════════════════════════════════════════════════════════════════
// PALETTE – Generic color slots.  Swap these values to re-theme the editor.
// ═══════════════════════════════════════════════════════════════════════════

// -- Accent colours (warm → cool) --
pub const ACCENT_WARM_1: Color    = Color::from_rgb(0.961, 0.878, 0.863);  // #f5e0dc
pub const ACCENT_WARM_2: Color    = Color::from_rgb(0.949, 0.804, 0.804);  // #f2cdcd
pub const ACCENT_PINK: Color      = Color::from_rgb(0.961, 0.761, 0.906);  // #f5c2e7
pub const ACCENT_PURPLE: Color    = Color::from_rgb(0.796, 0.651, 0.969);  // #cba6f7
pub const ACCENT_RED: Color       = Color::from_rgb(0.953, 0.545, 0.659);  // #f38ba8
pub const ACCENT_DARK_RED: Color  = Color::from_rgb(0.922, 0.627, 0.675);  // #eba0ac
pub const ACCENT_ORANGE: Color    = Color::from_rgb(0.980, 0.702, 0.529);  // #fab387
pub const ACCENT_YELLOW: Color    = Color::from_rgb(0.976, 0.886, 0.686);  // #f9e2af
pub const ACCENT_GREEN: Color     = Color::from_rgb(0.651, 0.890, 0.631);  // #a6e3a1
pub const ACCENT_TEAL: Color      = Color::from_rgb(0.580, 0.886, 0.835);  // #94e2d5
pub const ACCENT_SKY: Color       = Color::from_rgb(0.537, 0.863, 0.922);  // #89dceb
pub const ACCENT_MID_BLUE: Color  = Color::from_rgb(0.455, 0.780, 0.925);  // #74c7ec
pub const ACCENT_BLUE: Color      = Color::from_rgb(0.537, 0.706, 0.980);  // #89b4fa
pub const ACCENT_SOFT_BLUE: Color = Color::from_rgb(0.706, 0.745, 0.996);  // #b4befe

// -- Text hierarchy --
pub const TEXT_1: Color           = Color::from_rgb(0.804, 0.839, 0.957);  // #cdd6f4
pub const TEXT_2: Color           = Color::from_rgb(0.729, 0.761, 0.871);  // #bac2de
pub const TEXT_3: Color           = Color::from_rgb(0.651, 0.678, 0.784);  // #a6adc8

// -- Overlay layers --
pub const OVERLAY_3: Color        = Color::from_rgb(0.576, 0.600, 0.698);  // #9399b2
pub const OVERLAY_2: Color        = Color::from_rgb(0.498, 0.518, 0.612);  // #7f849c
pub const OVERLAY_1: Color        = Color::from_rgb(0.424, 0.439, 0.525);  // #6c7086

// -- Surface layers --
pub const SURFACE_3: Color        = Color::from_rgb(0.345, 0.357, 0.439);  // #585b70
pub const SURFACE_2: Color        = Color::from_rgb(0.271, 0.278, 0.353);  // #45475a
pub const SURFACE_1: Color        = Color::from_rgb(0.192, 0.196, 0.267);  // #313244

// -- Background layers --
pub const BG_BASE: Color          = Color::from_rgb(0.118, 0.118, 0.180);  // #1e1e2e
pub const BG_MANTLE: Color        = Color::from_rgb(0.094, 0.094, 0.145);  // #181825
pub const BG_CRUST: Color         = Color::from_rgb(0.067, 0.067, 0.106);  // #11111b

// ═══════════════════════════════════════════════════════════════════════════
// ThemeColors – the struct the rest of the app consumes
// ═══════════════════════════════════════════════════════════════════════════

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

// ── Helpers ─────────────────────────────────────────────────────────────────

/// Convert an iced Color to a syntect SynColor (u8 components).
const fn to_syn(c: Color) -> SynColor {
    SynColor {
        r: (c.r * 255.0) as u8,
        g: (c.g * 255.0) as u8,
        b: (c.b * 255.0) as u8,
        a: 255,
    }
}

/// Build a single syntect ThemeItem from a scope selector string + foreground Color.
fn scope_item(scope_str: &str, fg: Color, style: FontStyle) -> ThemeItem {
    ThemeItem {
        scope: ScopeSelectors::from_str(scope_str).unwrap_or_default(),
        style: StyleModifier {
            foreground: Some(to_syn(fg)),
            background: None,
            font_style: Some(style),
        },
    }
}

fn build_palette_syntax_theme() -> SynTheme {
    let none = FontStyle::empty();
    let italic = FontStyle::ITALIC;
    let bold = FontStyle::BOLD;

    let scopes = vec![
        // Comments
        scope_item("comment, comment.line, comment.block, punctuation.definition.comment", OVERLAY_2, italic),
        // Keywords & control flow
        scope_item("keyword, keyword.control, keyword.operator.logical, storage.type, storage.modifier", ACCENT_PURPLE, none),
        // Functions / methods
        scope_item("entity.name.function, support.function, meta.function-call", ACCENT_BLUE, none),
        // Types / classes
        scope_item("entity.name.type, entity.name.class, support.type, support.class", ACCENT_YELLOW, none),
        // Strings
        scope_item("string, string.quoted, punctuation.definition.string", ACCENT_GREEN, none),
        // Numbers
        scope_item("constant.numeric, constant.numeric.integer, constant.numeric.float", ACCENT_ORANGE, none),
        // Boolean / language constants
        scope_item("constant.language, constant.language.boolean", ACCENT_ORANGE, italic),
        // Other constants
        scope_item("constant.other, variable.other.constant", ACCENT_ORANGE, none),
        // Variables
        scope_item("variable, variable.other, variable.parameter", TEXT_1, none),
        // Properties / fields
        scope_item("variable.other.property, variable.other.member, support.variable.property", ACCENT_SOFT_BLUE, none),
        // Operators
        scope_item("keyword.operator, keyword.operator.assignment, punctuation.accessor", ACCENT_SKY, none),
        // Punctuation / brackets
        scope_item("punctuation, punctuation.section, punctuation.separator, meta.brace", OVERLAY_3, none),
        // Tags (HTML / XML)
        scope_item("entity.name.tag, punctuation.definition.tag", ACCENT_PURPLE, none),
        // Attributes
        scope_item("entity.other.attribute-name", ACCENT_YELLOW, italic),
        // Namespaces / modules
        scope_item("entity.name.namespace, entity.name.module", ACCENT_WARM_1, none),
        // Macros
        scope_item("entity.name.macro, support.function.macro", ACCENT_TEAL, bold),
        // Lifetimes / labels
        scope_item("storage.modifier.lifetime, entity.name.lifetime", ACCENT_DARK_RED, italic),
        // Escape sequences
        scope_item("constant.character.escape", ACCENT_PINK, none),
        // Regex
        scope_item("string.regexp", ACCENT_ORANGE, none),
        // Decorators / annotations
        scope_item("meta.decorator, meta.annotation, punctuation.decorator", ACCENT_ORANGE, italic),
        // Markdown headings
        scope_item("markup.heading, entity.name.section", ACCENT_BLUE, bold),
        // Markdown bold / italic
        scope_item("markup.bold", TEXT_1, bold),
        scope_item("markup.italic", TEXT_1, italic),
        // Links
        scope_item("markup.underline.link, string.other.link", ACCENT_MID_BLUE, none),
        // Diff
        scope_item("markup.inserted", ACCENT_GREEN, none),
        scope_item("markup.deleted", ACCENT_RED, none),
        scope_item("markup.changed", ACCENT_YELLOW, none),
        // Invalid / errors
        scope_item("invalid, invalid.illegal", ACCENT_RED, none),
    ];

    SynTheme {
        name: Some("Palette".to_string()),
        author: None,
        settings: ThemeSettings {
            foreground: Some(to_syn(TEXT_1)),
            background: Some(to_syn(BG_BASE)),
            caret: Some(to_syn(ACCENT_WARM_1)),
            line_highlight: Some(to_syn(SURFACE_1)),
            selection: Some(SynColor { r: 137, g: 180, b: 250, a: 77 }), // ACCENT_BLUE @ 0.3
            ..ThemeSettings::default()
        },
        scopes,
    }
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            bg_primary:         SURFACE_1,
            bg_secondary:       BG_MANTLE,
            bg_editor:          BG_BASE,
            bg_tab_active:      SURFACE_1,
            bg_tab_inactive:    BG_MANTLE,
            bg_status_bar:      BG_MANTLE,
            bg_tab_bar:         BG_CRUST,
            bg_hover:           SURFACE_2,
            bg_pressed:         SURFACE_3,
            bg_drag_handle:     SURFACE_1,
            text_primary:       TEXT_1,
            text_secondary:     TEXT_2,
            text_muted:         TEXT_3,
            text_dim:           OVERLAY_2,
            text_placeholder:   OVERLAY_1,
            border_subtle:      SURFACE_2,
            border_very_subtle: SURFACE_1,
            selection:          Color::from_rgba(0.537, 0.706, 0.980, 0.3), // ACCENT_BLUE @ 30%
            shadow_dark:        Color::from_rgba(0.067, 0.067, 0.106, 0.5), // BG_CRUST @ 50%
            shadow_light:       Color::from_rgba(0.345, 0.357, 0.439, 0.08), // SURFACE_3 @ 8%
            syntax_theme:       build_palette_syntax_theme(),
        }
    }
}

pub static THEME: Lazy<ThemeColors> = Lazy::new(ThemeColors::default);
