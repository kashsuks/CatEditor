//! This file is used in order to conduct a periodic
//! states and update check on the system

use super::*;

impl App {
    /// Creates the application state and schedules an initial update check.
    #[allow(dead_code)]
    pub fn new() -> (Self, iced::Task<Message>) {
        Self::new_with_path(None)
    }

    pub fn new_with_path(startup_path: Option<std::path::PathBuf>) -> (Self, iced::Task<Message>) {
        let app = Self::default();

        let update_task = iced::Task::perform(
            crate::features::updater::check_for_update(),
            |result| match result {
                Some(info) => Message::UpdateAvailable(info),
                None => Message::DismissUpdateBanner,
            },
        );

        let startup_task = match startup_path {
            Some(path) if path.is_dir() => iced::Task::done(Message::FolderOpened(path)),
            Some(path) => Self::open_path_task(path),
            None => iced::Task::none(),
        };

        (app, iced::Task::batch([update_task, startup_task]))
    }

    fn restore_last_session_task(&mut self) -> iced::Task<Message> {
        if !self.editor_preferences.restore_session_enabled {
            return iced::Task::none();
        }

        let Some(session) = crate::config::session::load_session() else {
            return iced::Task::none();
        };

        self.pending_active_tab_path = session.active_tab_index;
        self.pending_cursor_restores = session.cursor_position;

        let mut tasks = Vec::new();

        if let Some(folder) = session.folder {
            if folder.is_dir() {
                tasks.push(iced::Task::done(Message::FolderOpened(folder)));
            }
        }

        for path in session.open_tabs {
            if path.is_file() {
                tasks.push(Self::open_path_task(path));
            }
        }

        iced::Task::batch(tasks)
    }
}
