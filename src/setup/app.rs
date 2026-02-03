use crate::command_palette::CommandPalette;
use crate::config::theme_manager::{ThemeColors, load_theme};
use crate::file_tree::FileTree;
use crate::fuzzy_finder::FuzzyFinder;
use crate::hotkey::command_input::CommandInput;
use crate::hotkey::find_replace::FindReplace;
use crate::setup::menu;
use crate::setup::theme;
use crate::terminal::Terminal;
use crate::vim_mode::{VimState, VimMode};
use eframe::egui;
use std::path::PathBuf;

pub struct CatEditorApp {
    pub text: String,
    pub command_buffer: String,
    pub should_quit: bool,
    pub current_file: Option<String>,
    pub current_folder: Option<PathBuf>,
    pub cursor_pos: usize,

    pub theme: ThemeColors,
    pub vim_state: VimState,

    pub command_palette: CommandPalette,
    pub find_replace: FindReplace,
    pub command_input: CommandInput,
    pub fuzzy_finder: FuzzyFinder,
    pub file_tree: FileTree,
    pub terminal: Terminal,
    
    // For tracking text changes in normal mode
    last_text: String,
}

impl Default for CatEditorApp {
    fn default() -> Self {
        let theme = load_theme();
        Self {
            text: String::new(),
            command_buffer: String::new(),
            should_quit: false,
            current_file: None,
            current_folder: None,
            cursor_pos: 0,
            theme,
            vim_state: VimState::default(),
            command_palette: CommandPalette::default(),
            find_replace: FindReplace::default(),
            command_input: CommandInput::default(),
            fuzzy_finder: FuzzyFinder::default(),
            file_tree: FileTree::default(),
            terminal: Terminal::default(),
            last_text: String::new(),
        }
    }
}

impl eframe::App for CatEditorApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        if self.should_quit {
            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            return;
        }

        // Handle global keyboard shortcuts (should work regardless of mode)
        ctx.input(|i| {
            let modifier_pressed = if cfg!(target_os = "macos") {
                i.modifiers.command
            } else {
                i.modifiers.ctrl
            };

            // Only allow these shortcuts in Normal mode or when not in text editor
            if modifier_pressed && i.key_pressed(egui::Key::Comma) {
                if i.modifiers.shift {
                    self.theme = load_theme();
                } else {
                    self.command_palette.toggle();
                }
            }

            if modifier_pressed && i.key_pressed(egui::Key::F) {
                self.find_replace.toggle();
            }

            if modifier_pressed && i.key_pressed(egui::Key::B) {
                self.file_tree.toggle();
            }

            // Opens system's default terminal in current folder
            if modifier_pressed && i.key_pressed(egui::Key::J) {
                self.terminal.toggle();
            }

            if modifier_pressed && i.key_pressed(egui::Key::K) {
                if let Some(path) = rfd::FileDialog::new().pick_folder() {
                    self.current_folder = Some(path.clone());
                    self.fuzzy_finder.set_folder(path.clone());
                    self.file_tree.set_root(path.clone());
                    self.terminal.set_directory(path);
                }
            }
        });

        theme::apply_theme(ctx, self);

        // Only process if no modal dialogs are open
        let modals_open = self.command_palette.open 
            || self.find_replace.open 
            || self.command_input.open 
            || self.fuzzy_finder.open;

        menu::show_menu_bar(ctx, self);

        if let Some(file_path) = self.file_tree.show(ctx) {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                self.text = content;
                self.current_file = Some(file_path.display().to_string());
            }
        }

        if let Some(command) = self.command_palette.show(ctx) {
            self.execute_palette_command(ctx, &command);
        }

        self.find_replace
            .show(ctx, &mut self.text, &mut self.cursor_pos);

        if let Some(cmd) = self.command_input.show(ctx) {
            self.command_buffer = cmd;
        }

        if let Some(file_path) = self.fuzzy_finder.show(ctx) {
            if let Ok(content) = std::fs::read_to_string(&file_path) {
                self.text = content;
                self.current_file = Some(file_path.display().to_string());
            }
        }

        self.terminal.show(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::TopBottomPanel::bottom("status_bar").show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    // Show Vim mode on the left
                    ui.label(
                        egui::RichText::new(self.vim_state.get_mode_string())
                            .color(egui::Color32::from_rgb(150, 200, 255))
                            .text_style(egui::TextStyle::Monospace),
                    );

                    ui.separator();

                    // Show cursor position
                    let (line, col) = self.get_cursor_line_col();
                    ui.label(
                        egui::RichText::new(format!("Ln {}, Col {}", line + 1, col + 1))
                            .color(egui::Color32::from_gray(150))
                            .text_style(egui::TextStyle::Small),
                    );

                    // Show current folder if open
                    if let Some(folder) = &self.current_folder {
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            ui.label(
                                egui::RichText::new(format!("📁 {}", folder.display()))
                                    .color(egui::Color32::from_gray(150))
                                    .text_style(egui::TextStyle::Small),
                            );
                        });
                    }
                });
            });

            // Handle Vim motions if no modals are open
            // Store text before handling vim input
            if !modals_open && matches!(self.vim_state.mode, VimMode::Normal) {
                self.last_text = self.text.clone();
            }
            
            if !modals_open {
                self.vim_state.handle_input(ctx, &mut self.text, &mut self.cursor_pos);
            }

            egui::ScrollArea::vertical()
                .id_salt("main_scroll_area")
                .show(ui, |ui| {
                    ui.horizontal_top(|ui| {
                        let line_count = self.text.lines().count().max(1);

                        let max_line_digits = line_count.to_string().len();
                        let font_id = egui::TextStyle::Monospace.resolve(ui.style());
                        let char_width = ui.fonts(|f| f.glyph_width(&font_id, '0'));
                        let line_number_width = (max_line_digits as f32 * char_width) + 20.0;

                        ui.allocate_ui_with_layout(
                            egui::vec2(line_number_width, ui.available_height()),
                            egui::Layout::top_down(egui::Align::RIGHT),
                            |ui| {
                                ui.style_mut().spacing.item_spacing.y = 0.0;
                                for line_num in 1..=line_count {
                                    ui.label(
                                        egui::RichText::new(format!("{} ", line_num))
                                            .color(egui::Color32::from_gray(120))
                                            .text_style(egui::TextStyle::Monospace),
                                    );
                                }
                            },
                        );

                        let text_edit = egui::TextEdit::multiline(&mut self.text)
                            .font(egui::TextStyle::Monospace)
                            .frame(false)
                            .desired_width(f32::INFINITY)
                            .cursor_at_end(false);

                        let available = ui.available_size();
                        let mut output = ui.allocate_ui(available, |ui| text_edit.show(ui)).inner;

                        // In Normal mode, revert any text changes made by the widget
                        if matches!(self.vim_state.mode, VimMode::Normal) && self.text != self.last_text {
                            self.text = self.last_text.clone();
                        }

                        // sync widget cursor with vim cursor 
                        if matches!(self.vim_state.mode, VimMode::Normal) {
                            // hard force the widget to use our cursor
                            let cursor = output.galley.from_ccursor(egui::text::CCursor::new(self.cursor_pos));
                            let cursor_range = egui::text::CursorRange::one(cursor);
                            output.state.cursor.set_range(Some(cursor_range));
                            output.state.store(ui.ctx(), output.response.id);
                        }

                        // Draw custom block cursor for Normal mode
                        if matches!(self.vim_state.mode, VimMode::Normal) {
                            let galley = output.galley.clone();
                            let text_draw_pos = output.galley_pos;
                            let painter = ui.painter();

                            // Find cursor position in galley
                            let cursor = galley.from_ccursor(egui::text::CCursor::new(self.cursor_pos));
                            
                            if cursor.rcursor.row < galley.rows.len() {
                                let row = &galley.rows[cursor.rcursor.row];
                                let row_rect = row.rect;
                                
                                // Get the position of the character at cursor
                                let char_x = if cursor.rcursor.column < row.glyphs.len() {
                                    row.glyphs[cursor.rcursor.column].pos.x
                                } else if row_rect.width() > 0.0 {
                                    row_rect.max.x
                                } else {
                                    0.0
                                };
                                
                                // Get character width for block cursor
                                let char_width = if cursor.rcursor.column < row.glyphs.len() {
                                    let glyph = &row.glyphs[cursor.rcursor.column];
                                    glyph.size().x.max(8.0)
                                } else {
                                    8.0 // Default width if at end of line
                                };
                                
                                // Draw block cursor
                                let cursor_rect = egui::Rect::from_min_size(
                                    text_draw_pos + egui::vec2(char_x, row_rect.min.y),
                                    egui::vec2(char_width, row_rect.height()),
                                );
                                
                                // Draw a semi-transparent block cursor
                                painter.rect_filled(
                                    cursor_rect,
                                    egui::Rounding::ZERO,
                                    egui::Color32::from_rgba_unmultiplied(100, 150, 255, 120),
                                );
                                
                                // Draw cursor outline
                                painter.rect_stroke(
                                    cursor_rect,
                                    egui::Rounding::ZERO,
                                    egui::Stroke::new(2.0, egui::Color32::from_rgb(100, 150, 255)),
                                );
                                
                                // Draw the character in inverse color for better visibility
                                if cursor.rcursor.column < row.glyphs.len() {
                                    let chars: Vec<char> = self.text.chars().collect();
                                    if self.cursor_pos < chars.len() {
                                        let ch = chars[self.cursor_pos];
                                        painter.text(
                                            text_draw_pos + egui::vec2(char_x, row_rect.min.y),
                                            egui::Align2::LEFT_TOP,
                                            ch.to_string(),
                                            egui::FontId::monospace(14.0),
                                            egui::Color32::from_rgb(30, 30, 30),
                                        );
                                    }
                                }
                            }
                        }

                        if self.find_replace.open && !self.find_replace.find_text.is_empty() {
                            let galley = output.galley.clone();
                            let text_draw_pos = output.galley_pos;
                            let painter = ui.painter();

                            let highlight_ranges = self.find_replace.get_highlight_ranges();
                            let current_match_range = self.find_replace.get_current_match_range();

                            for (start, end) in highlight_ranges {
                                let is_current = current_match_range
                                    .map(|(curr_start, curr_end)| {
                                        start == curr_start && end == curr_end
                                    })
                                    .unwrap_or(false);

                                let start_cursor =
                                    galley.from_ccursor(egui::text::CCursor::new(start));
                                let end_cursor = galley.from_ccursor(egui::text::CCursor::new(end));

                                if start_cursor.rcursor.row == end_cursor.rcursor.row {
                                    let row_rect = galley.rows[start_cursor.rcursor.row].rect;

                                    let start_x = if start_cursor.rcursor.column
                                        < galley.rows[start_cursor.rcursor.row].glyphs.len()
                                    {
                                        galley.rows[start_cursor.rcursor.row].glyphs
                                            [start_cursor.rcursor.column]
                                            .pos
                                            .x
                                    } else {
                                        row_rect.max.x
                                    };

                                    let end_x = if end_cursor.rcursor.column
                                        < galley.rows[end_cursor.rcursor.row].glyphs.len()
                                    {
                                        galley.rows[end_cursor.rcursor.row].glyphs
                                            [end_cursor.rcursor.column]
                                            .pos
                                            .x
                                    } else {
                                        row_rect.max.x
                                    };

                                    let rect = egui::Rect::from_min_max(
                                        text_draw_pos + egui::vec2(start_x, row_rect.min.y),
                                        text_draw_pos + egui::vec2(end_x, row_rect.max.y),
                                    );

                                    let color = if is_current {
                                        egui::Color32::from_rgb(173, 216, 230)
                                    } else {
                                        egui::Color32::from_rgba_unmultiplied(255, 255, 0, 80)
                                    };

                                    painter.rect_filled(rect, egui::Rounding::same(2.0), color);
                                }

                                if is_current {
                                    let row_rect = galley.rows[start_cursor.rcursor.row].rect;
                                    let scroll_to_y = text_draw_pos.y + row_rect.min.y - 100.0;
                                    ui.scroll_to_rect(
                                        egui::Rect::from_min_size(
                                            egui::pos2(0.0, scroll_to_y),
                                            egui::vec2(1.0, 1.0),
                                        ),
                                        Some(egui::Align::Center),
                                    );
                                }
                            }
                        }

                        // Always request focus in Normal mode (for vim commands), but also in Insert mode
                        let something_else_has_focus = !output.response.has_focus()
                            && ctx.memory(|mem| mem.focused().is_some());

                        if !something_else_has_focus && !modals_open {
                            output.response.request_focus();
                        }
                        
                        if let Some(cursor) = output.cursor_range {
                            // Only update cursor_pos from widget in Insert mode
                            if matches!(self.vim_state.mode, VimMode::Insert) {
                                self.cursor_pos = cursor.primary.ccursor.index;
                            }
                        }
                    });
                });
        });
    }
}

impl CatEditorApp {
    fn execute_palette_command(&mut self, ctx: &egui::Context, command: &str) {
        match command {
            "Theme" => {
                // The theme menu is already shown in menu.rs, so we don't need to do anything special
                // User can access it via the menu bar
            }
            "Open File" => {
                if let Some(path) = rfd::FileDialog::new().pick_file() {
                    if let Ok(content) = std::fs::read_to_string(&path) {
                        self.text = content;
                        self.current_file = Some(path.display().to_string());
                    }
                }
            }
            "Save File" => {
                if let Some(path) = &self.current_file {
                    let _ = std::fs::write(path, &self.text);
                } else if let Some(path) = rfd::FileDialog::new().save_file() {
                    let _ = std::fs::write(&path, &self.text);
                    self.current_file = Some(path.display().to_string());
                }
            }
            "Quit" => {
                ctx.send_viewport_cmd(egui::ViewportCommand::Close);
            }
            "New File" => {
                self.text.clear();
                self.current_file = None;
            }
            "Save As" => {
                if let Some(path) = rfd::FileDialog::new().save_file() {
                    let _ = std::fs::write(&path, &self.text);
                    self.current_file = Some(path.display().to_string());
                }
            }
            _ => {}
        }
    }

    fn get_cursor_line_col(&self) -> (usize, usize) {
        let mut current_pos = 0;
        let mut line = 0;

        for line_text in self.text.lines() {
            let line_len = line_text.len() + 1; // +1 for newline
            if current_pos + line_len > self.cursor_pos || current_pos + line_text.len() >= self.cursor_pos {
                return (line, self.cursor_pos - current_pos);
            }
            current_pos += line_len;
            line += 1;
        }

        (line.saturating_sub(1), 0)
    }
}
