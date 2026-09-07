//! Window event subscription handlers.

use iced::{window, Event, Subscription};

use crate::message::Message;

/// Emits window resize messages to persist size preferences.
pub fn resizes() -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _id| match event {
        Event::Window(window::Event::Resized(size)) => Some(Message::WindowResized(
            size.width.max(0.0) as u32,
            size.height.max(0.0) as u32,
        )),
        _ => None,
    })
}

/// Refreshes workspace-derived state when the app regains focus
/// This catches branch switches or filesystem changes when made outside Pinel
pub fn focus_refresh() -> Subscription<Message> {
    iced::event::listen_with(|event, _status, _id| match event {
        Event::Window(window::Event::Focused) => Some(Message::FileTreeRefresh),
        _ => None,
    })
}

/// Lets the app persist state (e.g. the session snapshot) before the
/// window actually closes - requires `exit_on_close_request: false` on the
/// window settings, otherwise iced closes immediately and this never fires.
pub fn close_requests() -> Subscription<Message> {
    iced::event::listen_with(|event, _status, id| match event {
        Event::Window(window::Event::CloseRequested) => Some(Message::WindowCloseRequested(id)),
        _ => None,
    })
}
