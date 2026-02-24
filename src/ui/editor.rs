use iced::keyboard::{key, Key};
use iced::widget::text_editor::{TextEditor, Content, Binding, KeyPress, Motion};
use iced::{Element, Length};

use crate::message::Message;
use crate::syntax::{VscodeHighlighter, Settings};
use crate::ui::styles::text_editor_style;

pub fn create_editor<'a>(content: &'a Content, extension: &str) -> Element<'a, Message> {
    TextEditor::new(content) // Creates a new TextEditor object
        .on_action(Message::EditorAction) // Sends a Message when an edit is made
        .key_binding(editor_key_bindings) // Uses key bindings from the below function
        .highlight_with::<VscodeHighlighter>(
            Settings {
                extension: extension.to_string(),
            },
        |highlight, _theme| highlight.to_format(),
        )
        .style(text_editor_style) // Uses the editor styles determined in the styles.rs file
        .height(Length::Fill)
        .into()
}

fn editor_key_bindings(key_press: KeyPress) -> Option<Binding<Message>> {
    let modifiers = key_press.modifiers;

    if let Key::Character(_c) = key_press.key.as_ref() {
        if modifiers.command() {
            return None;
        }
    }

    match key_press.key.as_ref() {
        Key::Named(key::Named::Backspace) => {
            if modifiers.command() {
                Some(Binding::Sequence(vec![ // Detects when the cmd key is pressed and begin a sequence
                    Binding::Select(Motion::Home),
                    Binding::Backspace, // If home + backspace is detected, remove whole line
                ]))
            } else if modifiers.alt() {
                Some(Binding::Sequence(vec![
                    Binding::Select(Motion::WordLeft),
                    Binding::Backspace, // If the alt + delete, the word to the left is gone
                ]))
            } else {
                Binding::from_key_press(key_press) // Returns the default key press.
            }
        }
        Key::Named(key::Named::Delete) => {
            if modifiers.command() {
                Some(Binding::Sequence(vec![
                    Binding::Select(Motion::End),
                    Binding::Delete, // cmd + delete (the one that deletes a character to the right) deletes the line to the right of the cursor
                ]))
            } else if modifiers.alt() {
                Some(Binding::Sequence(vec![
                    Binding::Select(Motion::WordRight),
                    Binding::Delete, // alt + delete removes the word to the right
                ]))
            } else {
                Binding::from_key_press(key_press) // Again, ensures default actions
            }
        }
        _ => Binding::from_key_press(key_press),
    }
}

pub fn empty_editor<'a>() -> Element<'a, Message> {
    iced::widget::text("").into()
}
