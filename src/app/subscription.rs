use super::*;

impl App {
    /// Registers global event listeners and maps them to [`Message`] values.
    pub fn subscription(&self) -> Subscription<Message> {
        iced::event::listen_with(|event, _status, _id| match event {
            Event::Mouse(iced::mouse::Event::CursorMoved { position }) => {
                Some(Message::SidebarResizing(position.x))
            }
            Event::Mouse(iced::mouse::Event::ButtonReleased(iced::mouse::Button::Left)) => {
                Some(Message::SidebarResizeEnd)
            }
            Event::Keyboard(iced::keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                let navigation_msg = match &key {
                    Key::Named(iced::keyboard::key::Named::Escape) => Some(Message::EscapePressed),
                    Key::Named(iced::keyboard::key::Named::ArrowUp) => {
                        Some(Message::FuzzyFinderNavigate(-1))
                    }
                    Key::Named(iced::keyboard::key::Named::ArrowDown) => {
                        Some(Message::FuzzyFinderNavigate(1))
                    }
                    Key::Named(iced::keyboard::key::Named::Enter) => {
                        Some(Message::FuzzyFinderSelect)
                    }
                    _ => None,
                };

                if navigation_msg.is_some() {
                    return navigation_msg;
                }

                if let Key::Character(c) = &key {
                    if modifiers.command() && modifiers.control() {
                        match c.as_str() {
                            "f" => {
                                return Some(Message::ToggleFullscreen(window::Mode::Fullscreen))
                            }
                            _ => {}
                        }
                    } else if modifiers.command() && modifiers.shift() {
                        match c.as_str() {
                            "v" | "V" => return Some(Message::PreviewMarkdown),
                            "f" | "F" => return Some(Message::ToggleFuzzyFinder),
                            "p" | "P" => return Some(Message::ToggleCommandPalette),
                            "s" | "S" => return Some(Message::ToggleSettings),
                            _ => {}
                        }
                    } else if modifiers.command() {
                        match c.as_str() {
                            "b" => return Some(Message::ToggleSidebar),
                            "r" => return Some(Message::ToggleSidebar),
                            "o" => return Some(Message::OpenFolderDialog),
                            "w" => return Some(Message::CloseActiveTab),
                            "s" => return Some(Message::SaveFile),
                            "t" => return Some(Message::ToggleFileFinder),
                            "j" => return Some(Message::ToggleTerminal),
                            "f" => return Some(Message::ToggleFindReplace),
                            "n" => return Some(Message::NewFile),
                            _ => {}
                        }
                    }
                }
                None
            }
            _ => None,
        })
    }
}
