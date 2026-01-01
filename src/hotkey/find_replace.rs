use eframe::egui;

pub struct FindReplace {
    pub open: bool,
    pub find_text: String,
    pub replace_text: String,
    pub case_sensitive: bool,
    pub match_count: usize,
    pub current_match: usize,
    pub matches: Vec<usize>,
}

impl Default for FindReplace {
    fn default() -> Self {
        Self {
            open: false,
            find_text: String::new(),
            replace_text: String::new(),
            case_sensitive: false,
            match_count: 0,
            current_match: 0,
            matches: Vec::new(),
        }
    }
}

impl FindReplace {
    pub fn toggle(&mut self) {
        self.open = !self.open;
        if self.open {
            self.match_count = 0;
            self.current_match = 0;
            self.matches.clear();
        }
    }

    pub fn find_matches(&mut self, text: &str) -> Vec<usize> {
        if self.find_text.is_empty() {
            self.matches.clear();
            return Vec::new();
        }

        let mut found_matches = Vec::new();
        let search_text = if self.case_sensitive {
            text.to_string()
        } else {
            text.to_lowercase()
        };
        let find = if self.case_sensitive {
            self.find_text.clone()
        } else {
            self.find_text.to_lowercase()
        };

        let mut start = 0;
        while let Some(pos) = search_text[start..].find(&find) {
            found_matches.push(start + pos);
            start += pos + 1;
        }

        self.matches = found_matches.clone();
        self.match_count = found_matches.len();
        found_matches
    }

    pub fn go_to_next_match(&mut self, cursor_pos: &mut usize) {
        if !self.matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.matches.len();
            *cursor_pos = self.matches[self.current_match];
        }
    }

    pub fn go_to_prev_match(&mut self, cursor_pos: &mut usize) {
        if !self.matches.is_empty() {
            if self.current_match == 0 {
                self.current_match = self.matches.len() - 1;
            } else {
                self.current_match -= 1;
            }
            *cursor_pos = self.matches[self.current_match];
        }
    }

    pub fn replace_next(&mut self, text: &mut String) -> bool {
        if self.matches.is_empty() || self.current_match >= self.matches.len() {
            return false;
        }

        let pos = self.matches[self.current_match];
        let end = pos + self.find_text.len();
        text.replace_range(pos..end, &self.replace_text);
        
        self.find_matches(text);

        if self.current_match >= self.matches.len() && !self.matches.is_empty() {
            self.current_match = self.matches.len() - 1;
        }

        true
    }

    pub fn replace_all(&mut self, text: &mut String) -> usize {
        if self.find_text.is_empty() {
            return 0;
        }

        let count = self.matches.len();

        for &pos in self.matches.iter().rev() {
            let end = pos + self.find_text.len();
            text.replace_range(pos..end, &self.replace_text);
        }

        self.matches.clear();
        self.match_count = 0;
        self.current_match = 0;

        count
    }

    pub fn show(&mut self, ctx: &egui::Context, text: &mut String, cursor_pos: &mut usize) {
        if !self.open {
            return;
        }

        self.find_matches(text); // update the matches when a find text is changed

        egui::Window::new("Find and Replace")
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 50.0))
            .fixed_size(egui::vec2(500.0, 200.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label("Find:");
                    let find_response = ui.add(
                        egui::TextEdit::singleline(&mut self.find_text)
                            .desired_width(350.0)
                    );

                    if self.open && find_response.has_focus() {
                        find_response.request_focus();
                    }

                    if find_response.changed() {
                        self.current_match = 0;
                        self.find_matches(text);
                        //jump to the first match if any
                        if !self.matches.is_empty() {
                            *cursor_pos = self.matches[0];
                        }
                    }
                });

                ui.horizontal(|ui| {
                    ui.label("Replace:");
                    ui.add(
                        egui::TextEdit::singleline(&mut self.replace_text)
                            .desired_width(350.0)
                    );
                });

                ui.horizontal(|ui| {
                    ui.checkbox(&mut self.case_sensitive, "Case sensitive");

                    if self.match_count > 0 {
                        ui.label(format!("Match {} of {}", self.current_match + 1, self.match_count));
                    } else if !self.find_text.is_empty() {
                        ui.label("No matches found");
                    }
                });

                ui.add_space(10.0);

                ui.horizontal(|ui| {
                    if ui.button("Previous").clicked() && self.match_count > 0 {
                        self.go_to_prev_match(cursor_pos);
                    }

                    if ui.button("Next").clicked() && self.match_count > 0 {
                        self.go_to_next_match(cursor_pos);
                    }

                    if ui.button("Replace").clicked() {
                        self.replace_next(text);
                    }

                    if ui.button("Replace all").clicked() {
                        self.replace_all(text);
                    }

                    if ui.button("Close").clicked() {
                        self.open = false;
                    }
                });

                if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                    self.open = false;
                }

                if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                    if ui.input(|i| i.modifiers.shift) {
                        self.go_to_prev_match(cursor_pos);
                    } else {
                        self.go_to_next_match(cursor_pos);
                    }
                }
            });
    }

    pub fn get_highlight_ranges(&self) -> Vec<(usize, usize)> {
        if self.find_text.is_empty() {
            return Vec::new();
        }

        self.matches
            .iter()
            .map(|&pos| (pos, pos + self.find_text.len()))
            .collect()
    }

    pub fn get_current_match_range(&self) -> Option<(usize, usize)> {
        if self.matches.is_empty() || self.current_match >= self.matches.len() {
            return None;
        }

        let pos = self.matches[self.current_match];
        Some((pos, pos + self.find_text.len()))
    }
}