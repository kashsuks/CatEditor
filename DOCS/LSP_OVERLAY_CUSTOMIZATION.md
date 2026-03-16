# LSP Overlay Customization Guide

This document explains how Pinel currently handles:

- hover documentation timing
- LSP autocomplete filtering
- the built-in `iced-code-editor` hover/completion overlay
- how to change overlay color, font size, spacing, and layout

## Current Behavior

Pinel uses the built-in `iced-code-editor` LSP overlay when LSP is enabled.

Relevant files in this project:

- `/Users/ksukshavasi/Pinel/src/app/update.rs`
- `/Users/ksukshavasi/Pinel/src/app/view_editor.rs`
- `/Users/ksukshavasi/Pinel/src/app.rs`

The installed crate used by this project is:

- `iced-code-editor = 0.3.7`

The local crate source on this machine is:

- `/Users/ksukshavasi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced-code-editor-0.3.7/src/canvas_editor/lsp_process/overlay.rs`

## What Pinel Changes Locally

### Hover delay

Hover requests are not sent immediately.

Pinel tracks the currently hovered symbol and only sends the LSP hover request after the pointer stays on the same symbol for 2 seconds.

The delay is defined in:

- `/Users/ksukshavasi/Pinel/src/app.rs`

Current code:

```rust
const HOVER_TRIGGER_DELAY: Duration = Duration::from_secs(2);
```

To change the delay:

```rust
const HOVER_TRIGGER_DELAY: Duration = Duration::from_millis(750);
```

or:

```rust
const HOVER_TRIGGER_DELAY: Duration = Duration::from_secs(3);
```

### Completion filtering

When the LSP server returns completion items, Pinel now applies the current typed prefix to the built-in overlay state before showing the menu.

This logic lives in:

- `/Users/ksukshavasi/Pinel/src/app/update.rs`

The important lines are conceptually:

```rust
self.lsp_overlay.set_completions(items, position);
self.lsp_overlay.completion_filter = prefix;
self.lsp_overlay.filter_completions();
```

If you want stricter matching, change the `prefix` generation or apply extra filtering before `set_completions`.

Example: prefix-only filtering:

```rust
let filtered: Vec<String> = items
    .into_iter()
    .filter(|item| item.starts_with(&prefix))
    .collect();

self.lsp_overlay.set_completions(filtered, position);
```

Example: case-insensitive prefix filtering:

```rust
let prefix_lower = prefix.to_lowercase();
let filtered: Vec<String> = items
    .into_iter()
    .filter(|item| item.to_lowercase().starts_with(&prefix_lower))
    .collect();

self.lsp_overlay.set_completions(filtered, position);
```

## What You Can Customize Without Forking `iced-code-editor`

These changes can be made directly in Pinel.

### 1. Font size and line height

The built-in overlay is rendered in:

- `/Users/ksukshavasi/Pinel/src/app/view_editor.rs`

Current call:

```rust
iced_code_editor::view_lsp_overlay(
    &self.lsp_overlay,
    code_editor,
    &iced::Theme::CatppuccinMocha,
    13.0,
    20.0,
    Message::LspOverlay,
)
```

The parameters `13.0` and `20.0` are:

- overlay font size
- overlay line height

Example change:

```rust
iced_code_editor::view_lsp_overlay(
    &self.lsp_overlay,
    code_editor,
    &iced::Theme::TokyoNight,
    15.0,
    24.0,
    Message::LspOverlay,
)
```

### 2. Theme colors

The third argument is an `iced::Theme`.

Current code hardcodes:

```rust
&iced::Theme::CatppuccinMocha
```

You can replace it with any `iced::Theme` variant or your own theme object if you build one.

Example:

```rust
&iced::Theme::GruvboxDark
```

or:

```rust
&iced::Theme::Nord
```

If you want the LSP overlay to follow the app theme instead of a hardcoded theme, change the code so it passes the active app theme rather than a fixed `iced::Theme`.

## What Requires Forking or Patching `iced-code-editor`

The built-in overlay exposes only a small customization surface.

If you want to change:

- hover border radius
- hover padding
- hover width logic
- completion menu width
- completion row height
- completion header height
- completion border styling
- hover background internals
- completion menu layout internals

you need to patch the crate.

Those values are implemented inside:

- `/Users/ksukshavasi/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/iced-code-editor-0.3.7/src/canvas_editor/lsp_process/overlay.rs`

Important internal constants in that file include:

```rust
const MAX_COMPLETION_ITEMS: usize = 8;
const COMPLETION_ITEM_HEIGHT: f32 = 20.0;
const COMPLETION_HEADER_HEIGHT: f32 = 24.0;
const COMPLETION_PADDING: f32 = 4.0;
const COMPLETION_MENU_WIDTH: f32 = 250.0;
const SCROLLABLE_BORDER_RADIUS: f32 = 4.0;
```

The hover tooltip also hardcodes internal values such as:

- `hover_padding = 8.0`
- border width
- border radius
- background palette usage
- placement strategy

## Recommended Way To Patch The Crate

Do not edit the registry copy directly as a long-term solution.

Instead:

1. Vendor the crate into your repo, for example under:

`/Users/ksukshavasi/Pinel/vendor/iced-code-editor`

2. Point `Cargo.toml` to the local path:

```toml
[dependencies]
iced-code-editor = { path = "vendor/iced-code-editor", features = ["lsp-process"] }
```

3. Edit:

`/Users/ksukshavasi/Pinel/vendor/iced-code-editor/src/canvas_editor/lsp_process/overlay.rs`

This keeps your changes stable and reviewable.

## Example Patches In The Crate

### Make completion menu wider

In `overlay.rs`:

```rust
const COMPLETION_MENU_WIDTH: f32 = 340.0;
```

### Show more items at once

```rust
const MAX_COMPLETION_ITEMS: usize = 12;
```

### Increase row height

```rust
const COMPLETION_ITEM_HEIGHT: f32 = 26.0;
const COMPLETION_HEADER_HEIGHT: f32 = 28.0;
```

### Increase hover padding

Inside `build_hover_layer`:

```rust
let hover_padding = 12.0;
```

### Change hover border radius

Inside the hover `container::Style`:

```rust
radius: 10.0.into(),
```

### Change hover border color

Inside the hover `container::Style`:

```rust
border: iced::Border {
    color: Color::from_rgb(0.80, 0.55, 0.20),
    width: 1.0,
    radius: 6.0.into(),
},
```

### Change completion menu background

In `build_completion_layer`, locate the completion menu container style and replace the background:

```rust
background: Some(Background::Color(Color::from_rgb(0.10, 0.11, 0.14))),
```

## If You Want Full Visual Control

If the built-in overlay is too limited, the practical alternatives are:

1. Keep using `LspOverlayState`, but fork `view_lsp_overlay` inside a vendored `iced-code-editor`.
2. Stop calling `view_lsp_overlay` and render your own hover/completion widgets from `LspOverlayState`.

Option 2 gives maximum control over:

- colors
- typography
- width and height
- animation
- padding
- border radius
- placement behavior
- interaction behavior

That is the right choice if you want the LSP UI to match Pinel exactly.

## Summary

You can already customize from Pinel:

- hover delay
- overlay font size
- overlay line height
- overlay theme
- completion filtering rules

You need a crate patch or fork to customize:

- hover box internals
- completion menu internals
- exact visual styling and dimensions
- layout constants
