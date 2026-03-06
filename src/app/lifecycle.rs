use super::*;

impl App {
    /// Creates the application state and schedules an initial update check.
    pub fn new() -> (Self, iced::Task<Message>) {
        let app = Self::default();
        let task = iced::Task::perform(crate::updater::check_for_update(), |result| match result {
            Some(info) => Message::UpdateAvailable(info),
            None => Message::DismissUpdateBanner,
        });
        (app, task)
    }
}
