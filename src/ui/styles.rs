use iced::widget::button::{Style as ButtonStyle, Status as ButtonStatus};
use iced::widget::container;
use iced::widget::text_editor;
use iced::border::Radius;
use iced::{Background, Border, Color, Theme, Vector};

use crate::theme::*;

fn lighten(color: Color, amount: f32) -> Color {
    Color::from_rgba(
        (color.r + amount).min(1.0),
        (color.g + amount).min(1.0),
        (color.b + amount).min(1.0),
        color.a,
    )
}

pub fn tree_button_style(_theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let background = match status {
        ButtonStatus::Hovered => Some(Background::Color(THEME.bg_hover)),
        ButtonStatus::Pressed => Some(Background::Color(THEME.bg_pressed)),
        _ => None,
    };

    ButtonStyle {
        background,
        text_color: THEME.text_secondary,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn tab_button_style(is_active: bool) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, status| {
        let (background, text_color) = if is_active {
            (
                Some(Background::Color(lighten(THEME.bg_tab_bar, 0.08))),
                THEME.text_primary,
            )
        } else {
            let bg = match status {
                ButtonStatus::Hovered => Some(Background::Color(lighten(THEME.bg_tab_bar, 0.04))),
                _ => None,
            };
            (bg, THEME.text_dim)
        };
        ButtonStyle {
            background,
            text_color,
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: BORDER_RADIUS_TAB.into(),
            },
            shadow: Default::default(),
            snap: false,
        }
    }
}

pub fn tab_close_button_style(_theme: &Theme, _status: ButtonStatus) -> ButtonStyle {
    ButtonStyle {
        background: None,
        text_color: THEME.text_dim,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn editor_container_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.bg_primary)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: THEME.shadow_dark,
            offset: Vector::new(0.0, 4.0),
            blur_radius: 16.0,
        },
        ..Default::default()
    }
}

pub fn sidebar_container_style(_theme: &Theme) -> container::Style {
    let bg = THEME.bg_secondary;
    let bg_translucent = Color::from_rgba(bg.r, bg.g, bg.b, 0.93);
    container::Style {
        background: Some(Background::Color(bg_translucent)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: THEME.shadow_light,
            offset: Vector::new(-10.0, 0.0),
            blur_radius: 12.0,
        },
        ..Default::default()
    }
}

pub fn status_bar_style(_theme: &Theme) -> container::Style {
    let bg = THEME.bg_status_bar;
    let bg_subtle = Color::from_rgba(bg.r, bg.g, bg.b, bg.a * 0.5);
    container::Style {
        background: Some(Background::Color(bg_subtle)),
        border: Border {
            color: THEME.border_very_subtle,
            width: 0.0,
            radius: Radius { top_left: 0.0, top_right: 0.0, bottom_right: BORDER_RADIUS, bottom_left: BORDER_RADIUS },
        },
        ..Default::default()
    }
}

pub fn tab_bar_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(THEME.bg_tab_bar)),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: Radius { top_left: BORDER_RADIUS, top_right: BORDER_RADIUS, bottom_right: 0.0, bottom_left: 0.0 },
        },
        ..Default::default()
    }
}

pub fn text_editor_style(_theme: &Theme, _status: text_editor::Status) -> text_editor::Style {
    text_editor::Style {
        background: Background::Color(THEME.bg_editor),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        placeholder: THEME.text_placeholder,
        value: THEME.text_primary,
        selection: THEME.selection,
    }
}

pub fn drag_handle_style(_theme: &Theme, status: ButtonStatus) -> ButtonStyle {
    let background = match status {
        ButtonStatus::Hovered => Some(Background::Color(THEME.bg_hover)),
        ButtonStatus::Pressed => Some(Background::Color(THEME.bg_pressed)),
        _ => Some(Background::Color(THEME.bg_drag_handle)),
    };

    ButtonStyle {
        background,
        text_color: Color::TRANSPARENT,
        border: Border::default(),
        shadow: Default::default(),
        snap: false,
    }
}

pub fn search_panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            THEME.bg_primary.r,
            THEME.bg_primary.g,
            THEME.bg_primary.b,
            0.97,
        ))),
        border: Border {
            color: THEME.border_subtle,
            width: 1.0,
            radius: BORDER_RADIUS.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.5),
            offset: Vector::new(4.0, 4.0),
            blur_radius: 24.0,
        },
        ..Default::default()
    }
}

pub fn search_input_style(_theme: &Theme, _status: iced::widget::text_input::Status) -> iced::widget::text_input::Style {
    iced::widget::text_input::Style {
        background: Background::Color(Color::TRANSPARENT),
        border: Border {
            color: Color::TRANSPARENT,
            width: 0.0,
            radius: 0.0.into(),
        },
        icon: THEME.text_dim,
        placeholder: THEME.text_placeholder,
        value: THEME.text_primary,
        selection: THEME.selection,
    }
}

pub fn file_finder_panel_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: Some(Background::Color(Color::from_rgba(
            (THEME.bg_primary.r + 0.04).min(1.0),
            (THEME.bg_primary.g + 0.04).min(1.0),
            (THEME.bg_primary.b + 0.07).min(1.0),
            0.97,
        ))),
        border: Border {
            color: Color::from_rgba(1.0, 1.0, 1.0, 0.10),
            width: 1.0,
            radius: 18.0.into(),
        },
        shadow: iced::Shadow {
            color: Color::from_rgba(0.0, 0.0, 0.0, 0.7),
            offset: Vector::new(0.0, 24.0),
            blur_radius: 80.0,
        },
        ..Default::default()
    }
}

pub fn file_finder_item_style(is_selected: bool) -> impl Fn(&Theme, ButtonStatus) -> ButtonStyle {
    move |_theme, status| {
        let background = if is_selected {
            Some(Background::Color(THEME.bg_pressed))
        } else {
            match status {
                ButtonStatus::Hovered => Some(Background::Color(THEME.bg_hover)),
                _ => None,
            }
        };

        ButtonStyle {
            background,
            text_color: if is_selected { THEME.text_primary } else { THEME.text_muted },
            border: Border {
                color: Color::TRANSPARENT,
                width: 0.0,
                radius: 8.0.into(),
            },
            shadow: Default::default(),
            snap: false,
        }
    }
}