//! Tests for LSP overlay state (hover and completion UI).
//!
//! Run with: cargo test --test lsp_tests

use iced::Point;

#[test]
fn test_lsp_overlay_state_new() {
    let state = iced_code_editor::LspOverlayState::new();
    assert!(
        !state.hover_visible,
        "hover should not be visible initially"
    );
    assert!(
        !state.completion_visible,
        "completion should not be visible initially"
    );
}

#[test]
fn test_lsp_overlay_show_hover() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.show_hover("This is hover documentation".to_string());

    assert!(
        state.hover_visible,
        "hover should be visible after show_hover"
    );
    assert!(state.hover_text.is_some(), "hover text should be set");
    assert_eq!(
        state.hover_text.as_ref().unwrap(),
        "This is hover documentation"
    );
}

#[test]
fn test_lsp_overlay_clear_hover() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.show_hover("Some documentation".to_string());
    assert!(state.hover_visible);

    state.clear_hover();
    assert!(
        !state.hover_visible,
        "hover should not be visible after clear"
    );
    assert!(state.hover_text.is_none(), "hover text should be cleared");
}

#[test]
fn test_lsp_overlay_set_completions() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.set_completions(
        vec![
            "option1".to_string(),
            "option2".to_string(),
            "option3".to_string(),
        ],
        Point::new(10.0, 20.0),
    );

    assert!(state.completion_visible, "completion should be visible");
    assert_eq!(
        state.completion_items.len(),
        3,
        "should have 3 completion items"
    );
    assert_eq!(state.completion_selected, 0, "should start at first item");
    assert_eq!(state.completion_position, Some(Point::new(10.0, 20.0)));
}

#[test]
fn test_lsp_overlay_navigate() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.set_completions(
        vec!["a".to_string(), "b".to_string(), "c".to_string()],
        Point::ORIGIN,
    );

    // Navigate down
    state.navigate(1);
    assert_eq!(state.completion_selected, 1, "should be on second item");

    state.navigate(1);
    assert_eq!(state.completion_selected, 2, "should be on third item");

    state.navigate(1);
    assert_eq!(state.completion_selected, 0, "should wrap to first");

    // Navigate up
    state.navigate(-1);
    assert_eq!(state.completion_selected, 2, "should wrap to last");
}

#[test]
fn test_lsp_overlay_selected_item() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.set_completions(
        vec!["first".to_string(), "second".to_string()],
        Point::ORIGIN,
    );

    assert_eq!(
        state.selected_item(),
        Some("first"),
        "should return first item"
    );

    state.navigate(1);
    assert_eq!(
        state.selected_item(),
        Some("second"),
        "should return second item"
    );
}

#[test]
fn test_lsp_overlay_clear_completions() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.set_completions(vec!["item".to_string()], Point::ORIGIN);
    assert!(state.completion_visible);

    state.clear_completions();
    assert!(
        !state.completion_visible,
        "completion should not be visible after clear"
    );
    assert!(
        state.all_completions.is_empty(),
        "all completions should be cleared"
    );
}

#[test]
fn test_lsp_overlay_filter_completions() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.set_completions(
        vec!["foo".to_string(), "bar".to_string(), "foobar".to_string()],
        Point::ORIGIN,
    );

    state.completion_filter = "foo".to_string();
    state.filter_completions();

    assert_eq!(state.completion_items.len(), 2, "should filter to 2 items");
    assert!(state.completion_items.contains(&"foo".to_string()));
    assert!(state.completion_items.contains(&"foobar".to_string()));
    assert!(!state.completion_items.contains(&"bar".to_string()));
}

#[test]
fn test_lsp_overlay_set_hover_position() {
    let mut state = iced_code_editor::LspOverlayState::new();

    state.set_hover_position(Point::new(100.0, 200.0));

    assert_eq!(state.hover_position, Some(Point::new(100.0, 200.0)));
}

#[test]
fn test_lsp_overlay_default() {
    let state = iced_code_editor::LspOverlayState::default();

    assert!(!state.hover_visible);
    assert!(!state.completion_visible);
    assert!(state.completion_items.is_empty());
    assert!(state.hover_text.is_none());
}
