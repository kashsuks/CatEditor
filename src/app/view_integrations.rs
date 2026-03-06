use super::*;
use iced::widget::column;

impl App {
    pub(super) fn view_settings_theme(&self) -> Element<'_, Message> {
        use iced::widget::Space;

        let heading = text("Theme").size(18).color(theme().text_primary);
        let desc = text("Theme options have been moved to Preferences.")
            .size(12)
            .color(theme().text_dim);

        let separator = container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))),
                ..Default::default()
            });

        let hint_btn = button(text("Go to Preferences →").size(13).color(ACCENT_PURPLE))
            .on_press(Message::SettingsNavigate("preferences".to_string()))
            .style(|_theme, _status| button::Style {
                background: Some(Background::Color(ACCENT_PURPLE.scale_alpha(0.10))),
                border: iced::Border {
                    color: ACCENT_PURPLE.scale_alpha(0.25),
                    width: 1.0,
                    radius: 6.0.into(),
                },
                text_color: ACCENT_PURPLE,
                ..Default::default()
            })
            .padding(iced::Padding {
                top: 10.0,
                right: 18.0,
                bottom: 10.0,
                left: 18.0,
            });

        let reload_desc = text("Or reload a custom theme.lua from ~/.config/rode/")
            .size(11)
            .color(theme().text_dim);

        let reload_btn = button(
            text("↻  Reload theme.lua")
                .size(12)
                .color(theme().text_primary),
        )
        .on_press(Message::SettingsReloadTheme)
        .style(|_theme, _status| button::Style {
            background: Some(Background::Color(theme().bg_secondary)),
            border: iced::Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
                width: 1.0,
                radius: 6.0.into(),
            },
            text_color: theme().text_primary,
            ..Default::default()
        })
        .padding(iced::Padding {
            top: 8.0,
            right: 16.0,
            bottom: 8.0,
            left: 16.0,
        });

        column![
            heading,
            desc,
            separator,
            hint_btn,
            Space::new().height(Length::Fixed(8.0)),
            reload_desc,
            reload_btn
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    }

    pub(super) fn view_settings_wakatime(&self) -> Element<'_, Message> {
        use iced::widget::Space;

        let heading = text("WakaTime").size(18).color(theme().text_primary);
        let desc = text("Configure WakaTime integration for activity tracking.")
            .size(12)
            .color(theme().text_dim);

        let separator = container(Space::new().width(Length::Fill).height(Length::Fixed(1.0)))
            .style(|_theme| container::Style {
                background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.06))),
                ..Default::default()
            });

        let api_key_row = row![
            column![
                text("API Key").size(13).color(theme().text_muted),
                text("Your WakaTime API key for authentication")
                    .size(11)
                    .color(theme().text_dim),
            ]
            .spacing(2)
            .width(Length::FillPortion(2)),
            text_input("waka_xxxxx", &self.wakatime.api_key)
                .on_input(Message::WakaTimeApiKeyChanged)
                .size(13)
                .padding(iced::Padding {
                    top: 8.0,
                    right: 12.0,
                    bottom: 8.0,
                    left: 12.0
                })
                .style(search_input_style)
                .width(Length::FillPortion(3)),
        ]
        .spacing(16)
        .align_y(iced::Alignment::Center);

        let api_url_row = row![
            column![
                text("API URL").size(13).color(theme().text_muted),
                text("WakaTime API endpoint URL")
                    .size(11)
                    .color(theme().text_dim),
            ]
            .spacing(2)
            .width(Length::FillPortion(2)),
            text_input("https://api.wakatime.com/api/v1", &self.wakatime.api_url)
                .on_input(Message::WakaTimeApiUrlChanged)
                .size(13)
                .padding(iced::Padding {
                    top: 8.0,
                    right: 12.0,
                    bottom: 8.0,
                    left: 12.0
                })
                .style(search_input_style)
                .width(Length::FillPortion(3)),
        ]
        .spacing(16)
        .align_y(iced::Alignment::Center);

        let save_btn = button(
            text("Save WakaTime Settings")
                .size(12)
                .color(theme().text_primary),
        )
        .on_press(Message::SaveWakaTimeSettings)
        .style(|_theme, _status| button::Style {
            background: Some(Background::Color(ACCENT_PURPLE.scale_alpha(0.2))),
            border: iced::Border {
                color: ACCENT_PURPLE.scale_alpha(0.4),
                width: 1.0,
                radius: 4.0.into(),
            },
            text_color: theme().text_primary,
            ..Default::default()
        })
        .padding(iced::Padding {
            top: 8.0,
            right: 20.0,
            bottom: 8.0,
            left: 20.0,
        });

        column![
            heading,
            desc,
            separator,
            api_key_row,
            container(Space::new().width(Length::Fill).height(Length::Fixed(1.0))).style(
                |_theme| container::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.03))),
                    ..Default::default()
                }
            ),
            api_url_row,
            container(Space::new().width(Length::Fill).height(Length::Fixed(1.0))).style(
                |_theme| container::Style {
                    background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.03))),
                    ..Default::default()
                }
            ),
            Space::new().height(Length::Fixed(8.0)),
            save_btn,
        ]
        .spacing(12)
        .width(Length::Fill)
        .into()
    }
}
