use eframe::egui;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VimMode {
    Normal,
    Insert,
}

#[derive(Debug, Clone, Copy)]
enum CharSearchDirection {
    Forward,
    Backward,
}

#[derive(Debug, Clone, Copy)]
enum CharSearchType {
    To,      // f/F
    Before,  // t/T
}

pub struct VimState {
    pub mode: VimMode,
    pub count_buffer: String,
    last_char_search: Option<(char, CharSearchDirection, CharSearchType)>,
}

impl Default for VimState {
    fn default() -> Self {
        Self {
            mode: VimMode::Normal,
            count_buffer: String::new(),
            last_char_search: None,
        }
    }
}

impl VimState {
    pub fn handle_input(
        &mut self,
        ctx: &egui::Context,
        text: &mut String,
        cursor_pos: &mut usize,
    ) -> bool {
        let text_changed = false;

        ctx.input(|i| {
            match self.mode {
                VimMode::Normal => {
                    // Handle Escape to stay in normal mode (no-op)
                    if i.key_pressed(egui::Key::Escape) {
                        self.count_buffer.clear();
                        return;
                    }

                    // Handle 'i' to enter insert mode
                    if i.key_pressed(egui::Key::I) && !i.modifiers.any() {
                        self.mode = VimMode::Insert;
                        self.count_buffer.clear();
                        return;
                    }

                    // Handle number input for counts
                    for key in &i.events {
                        if let egui::Event::Text(text) = key {
                            if let Ok(num) = text.parse::<u32>() {
                                // Don't allow leading zeros unless it's just "0" for start of line
                                if num > 0 || (num == 0 && !self.count_buffer.is_empty()) {
                                    self.count_buffer.push_str(text);
                                    return;
                                }
                            }
                        }
                    }

                    let count = self.get_count();

                    // Basic movements
                    if i.key_pressed(egui::Key::H) && !i.modifiers.any() {
                        self.move_left(text, cursor_pos, count);
                        self.count_buffer.clear();
                    } else if i.key_pressed(egui::Key::J) && !i.modifiers.any() {
                        if i.modifiers.shift {
                            // Shift+J is not a movement, clear for now
                            self.count_buffer.clear();
                        } else {
                            self.move_down(text, cursor_pos, count);
                            self.count_buffer.clear();
                        }
                    } else if i.key_pressed(egui::Key::K) && !i.modifiers.any() {
                        self.move_up(text, cursor_pos, count);
                        self.count_buffer.clear();
                    } else if i.key_pressed(egui::Key::L) && !i.modifiers.any() {
                        self.move_right(text, cursor_pos, count);
                        self.count_buffer.clear();
                    }

                    // Word movements
                    else if i.key_pressed(egui::Key::W) && !i.modifiers.any() {
                        if i.modifiers.shift {
                            self.move_word_forward(text, cursor_pos, count, true);
                        } else {
                            self.move_word_forward(text, cursor_pos, count, false);
                        }
                        self.count_buffer.clear();
                    } else if i.key_pressed(egui::Key::E) && !i.modifiers.any() {
                        if i.modifiers.shift {
                            self.move_word_end(text, cursor_pos, count, true);
                        } else {
                            self.move_word_end(text, cursor_pos, count, false);
                        }
                        self.count_buffer.clear();
                    } else if i.key_pressed(egui::Key::B) && !i.modifiers.any() {
                        if i.modifiers.shift {
                            self.move_word_backward(text, cursor_pos, count, true);
                        } else {
                            self.move_word_backward(text, cursor_pos, count, false);
                        }
                        self.count_buffer.clear();
                    }

                    // Line movements
                    else if i.key_pressed(egui::Key::Num0) && self.count_buffer.is_empty() {
                        self.move_line_start(text, cursor_pos);
                    } else if i.key_pressed(egui::Key::Num4) && i.modifiers.shift {
                        // $ key
                        self.move_line_end(text, cursor_pos);
                        self.count_buffer.clear();
                    } else if i.key_pressed(egui::Key::Num6) && i.modifiers.shift {
                        // ^ key
                        self.move_first_non_blank(text, cursor_pos);
                        self.count_buffer.clear();
                    }

                    // Document movements
                    else if i.key_pressed(egui::Key::G) && !i.modifiers.any() {
                        if i.modifiers.shift {
                            // G - go to last line
                            self.move_to_line(text, cursor_pos, usize::MAX);
                            self.count_buffer.clear();
                        } else if !self.count_buffer.is_empty() {
                            // nG - go to line n
                            self.move_to_line(text, cursor_pos, count);
                            self.count_buffer.clear();
                        }
                        // Handle 'gg', 'gj', 'gk', 'ge', 'gE', 'g_' in event processing
                    }

                    // Screen positioning
                    else if i.key_pressed(egui::Key::Z) && !i.modifiers.any() {
                        // Will handle zz, zt, zb through event processing
                    }

                    // Paragraph movements (}, {) are handled in text events below

                    // TODO: Screen movements with Ctrl (need ScrollArea access from app level)
                    // Ctrl+e, Ctrl+y, Ctrl+f, Ctrl+b, Ctrl+d, Ctrl+u

                    // Handle multi-character commands through text events
                    self.handle_text_events(i, text, cursor_pos);
                }
                VimMode::Insert => {
                    if i.key_pressed(egui::Key::Escape) {
                        self.mode = VimMode::Normal;
                        // Move cursor back one position when leaving insert mode
                        if *cursor_pos > 0 {
                            *cursor_pos -= 1;
                        }
                    }
                }
            }
        });

        text_changed
    }

    fn handle_text_events(&mut self, input: &egui::InputState, text: &str, cursor_pos: &mut usize) {
        // This is a simplified handler - in practice, you'd need to track
        // multi-key sequences like 'gg', 'ge', etc.
        // For now, we'll handle character search commands
        
        for event in &input.events {
            if let egui::Event::Text(ch) = event {
                if ch.len() == 1 {
                    let c = ch.chars().next().unwrap();
                    
                    // Handle character search commands
                    match c {
                        '}' => {
                            // Next paragraph
                            let count = self.get_count();
                            self.move_paragraph_forward(text, cursor_pos, count);
                            self.count_buffer.clear();
                        }
                        '{' => {
                            // Previous paragraph
                            let count = self.get_count();
                            self.move_paragraph_backward(text, cursor_pos, count);
                            self.count_buffer.clear();
                        }
                        'f' | 'F' | 't' | 'T' => {
                            // Next character will be the search target
                            // This needs more sophisticated state tracking
                        }
                        ';' => {
                            if let Some((ch, dir, stype)) = self.last_char_search {
                                self.find_char(text, cursor_pos, ch, dir, stype, 1);
                            }
                        }
                        ',' => {
                            if let Some((ch, dir, stype)) = self.last_char_search {
                                let reverse_dir = match dir {
                                    CharSearchDirection::Forward => CharSearchDirection::Backward,
                                    CharSearchDirection::Backward => CharSearchDirection::Forward,
                                };
                                self.find_char(text, cursor_pos, ch, reverse_dir, stype, 1);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    fn get_count(&self) -> usize {
        self.count_buffer.parse::<usize>().unwrap_or(1)
    }

    // Basic movements
    fn move_left(&self, _text: &str, cursor_pos: &mut usize, count: usize) {
        for _ in 0..count {
            if *cursor_pos > 0 {
                *cursor_pos -= 1;
            }
        }
    }

    fn move_right(&self, text: &str, cursor_pos: &mut usize, count: usize) {
        for _ in 0..count {
            if *cursor_pos < text.len() {
                *cursor_pos += 1;
            }
        }
    }

    fn move_down(&self, text: &str, cursor_pos: &mut usize, count: usize) {
        let lines: Vec<&str> = text.lines().collect();
        let (current_line, col) = self.get_line_and_column(text, *cursor_pos);

        let target_line = (current_line + count).min(lines.len().saturating_sub(1));
        
        if target_line < lines.len() {
            let target_col = col.min(lines[target_line].len());
            *cursor_pos = self.get_position_from_line_col(text, target_line, target_col);
        }
    }

    fn move_up(&self, text: &str, cursor_pos: &mut usize, count: usize) {
        let lines: Vec<&str> = text.lines().collect();
        let (current_line, col) = self.get_line_and_column(text, *cursor_pos);

        let target_line = current_line.saturating_sub(count);
        
        if target_line < lines.len() {
            let target_col = col.min(lines[target_line].len());
            *cursor_pos = self.get_position_from_line_col(text, target_line, target_col);
        }
    }

    // Word movements
    fn move_word_forward(&self, text: &str, cursor_pos: &mut usize, count: usize, big_word: bool) {
        for _ in 0..count {
            self.move_word_forward_once(text, cursor_pos, big_word);
        }
    }

    fn move_word_forward_once(&self, text: &str, cursor_pos: &mut usize, big_word: bool) {
        let chars: Vec<char> = text.chars().collect();
        if *cursor_pos >= chars.len() {
            return;
        }

        let mut pos = *cursor_pos;

        // Skip current word
        if big_word {
            while pos < chars.len() && !chars[pos].is_whitespace() {
                pos += 1;
            }
        } else {
            if chars[pos].is_alphanumeric() || chars[pos] == '_' {
                while pos < chars.len() && (chars[pos].is_alphanumeric() || chars[pos] == '_') {
                    pos += 1;
                }
            } else if !chars[pos].is_whitespace() {
                while pos < chars.len() && !chars[pos].is_whitespace() && !chars[pos].is_alphanumeric() && chars[pos] != '_' {
                    pos += 1;
                }
            }
        }

        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }

        *cursor_pos = pos.min(chars.len());
    }

    fn move_word_end(&self, text: &str, cursor_pos: &mut usize, count: usize, big_word: bool) {
        for _ in 0..count {
            self.move_word_end_once(text, cursor_pos, big_word);
        }
    }

    fn move_word_end_once(&self, text: &str, cursor_pos: &mut usize, big_word: bool) {
        let chars: Vec<char> = text.chars().collect();
        if *cursor_pos >= chars.len() {
            return;
        }

        let mut pos = *cursor_pos;

        // Move at least one character forward
        if pos < chars.len() - 1 {
            pos += 1;
        }

        // Skip whitespace
        while pos < chars.len() && chars[pos].is_whitespace() {
            pos += 1;
        }

        if pos >= chars.len() {
            *cursor_pos = chars.len().saturating_sub(1);
            return;
        }

        // Move to end of word
        if big_word {
            while pos < chars.len() - 1 && !chars[pos + 1].is_whitespace() {
                pos += 1;
            }
        } else {
            if chars[pos].is_alphanumeric() || chars[pos] == '_' {
                while pos < chars.len() - 1 && (chars[pos + 1].is_alphanumeric() || chars[pos + 1] == '_') {
                    pos += 1;
                }
            } else {
                while pos < chars.len() - 1 && !chars[pos + 1].is_whitespace() && !chars[pos + 1].is_alphanumeric() && chars[pos + 1] != '_' {
                    pos += 1;
                }
            }
        }

        *cursor_pos = pos;
    }

    fn move_word_backward(&self, text: &str, cursor_pos: &mut usize, count: usize, big_word: bool) {
        for _ in 0..count {
            self.move_word_backward_once(text, cursor_pos, big_word);
        }
    }

    fn move_word_backward_once(&self, text: &str, cursor_pos: &mut usize, big_word: bool) {
        let chars: Vec<char> = text.chars().collect();
        if *cursor_pos == 0 {
            return;
        }

        let mut pos = *cursor_pos;

        // Move back at least one character
        pos = pos.saturating_sub(1);

        // Skip whitespace
        while pos > 0 && chars[pos].is_whitespace() {
            pos -= 1;
        }

        if pos == 0 {
            *cursor_pos = 0;
            return;
        }

        // Move to start of word
        if big_word {
            while pos > 0 && !chars[pos - 1].is_whitespace() {
                pos -= 1;
            }
        } else {
            if chars[pos].is_alphanumeric() || chars[pos] == '_' {
                while pos > 0 && (chars[pos - 1].is_alphanumeric() || chars[pos - 1] == '_') {
                    pos -= 1;
                }
            } else {
                while pos > 0 && !chars[pos - 1].is_whitespace() && !chars[pos - 1].is_alphanumeric() && chars[pos - 1] != '_' {
                    pos -= 1;
                }
            }
        }

        *cursor_pos = pos;
    }

    // Line movements
    fn move_line_start(&self, text: &str, cursor_pos: &mut usize) {
        let (line_num, _) = self.get_line_and_column(text, *cursor_pos);
        *cursor_pos = self.get_position_from_line_col(text, line_num, 0);
    }

    fn move_line_end(&self, text: &str, cursor_pos: &mut usize) {
        let lines: Vec<&str> = text.lines().collect();
        let (line_num, _) = self.get_line_and_column(text, *cursor_pos);
        
        if line_num < lines.len() {
            let line_len = lines[line_num].len();
            *cursor_pos = self.get_position_from_line_col(text, line_num, line_len);
        }
    }

    fn move_first_non_blank(&self, text: &str, cursor_pos: &mut usize) {
        let lines: Vec<&str> = text.lines().collect();
        let (line_num, _) = self.get_line_and_column(text, *cursor_pos);
        
        if line_num < lines.len() {
            let line = lines[line_num];
            let first_non_blank = line.chars().position(|c| !c.is_whitespace()).unwrap_or(0);
            *cursor_pos = self.get_position_from_line_col(text, line_num, first_non_blank);
        }
    }

    // Document movements
    fn move_to_line(&self, text: &str, cursor_pos: &mut usize, line: usize) {
        let lines: Vec<&str> = text.lines().collect();
        let target_line = if line == usize::MAX {
            lines.len().saturating_sub(1)
        } else {
            (line.saturating_sub(1)).min(lines.len().saturating_sub(1))
        };
        
        *cursor_pos = self.get_position_from_line_col(text, target_line, 0);
    }

    // Paragraph movements
    fn move_paragraph_forward(&self, text: &str, cursor_pos: &mut usize, count: usize) {
        for _ in 0..count {
            self.move_paragraph_forward_once(text, cursor_pos);
        }
    }

    fn move_paragraph_forward_once(&self, text: &str, cursor_pos: &mut usize) {
        let lines: Vec<&str> = text.lines().collect();
        let (mut current_line, _) = self.get_line_and_column(text, *cursor_pos);

        // Skip current paragraph
        while current_line < lines.len() && !lines[current_line].trim().is_empty() {
            current_line += 1;
        }

        // Skip blank lines
        while current_line < lines.len() && lines[current_line].trim().is_empty() {
            current_line += 1;
        }

        if current_line < lines.len() {
            *cursor_pos = self.get_position_from_line_col(text, current_line, 0);
        } else {
            *cursor_pos = text.len();
        }
    }

    fn move_paragraph_backward(&self, text: &str, cursor_pos: &mut usize, count: usize) {
        for _ in 0..count {
            self.move_paragraph_backward_once(text, cursor_pos);
        }
    }

    fn move_paragraph_backward_once(&self, text: &str, cursor_pos: &mut usize) {
        let lines: Vec<&str> = text.lines().collect();
        let (mut current_line, _) = self.get_line_and_column(text, *cursor_pos);

        if current_line == 0 {
            *cursor_pos = 0;
            return;
        }

        current_line -= 1;

        // Skip blank lines
        while current_line > 0 && lines[current_line].trim().is_empty() {
            current_line -= 1;
        }

        // Skip current paragraph
        while current_line > 0 && !lines[current_line].trim().is_empty() {
            current_line -= 1;
        }

        // Move to first line of paragraph
        if current_line < lines.len() && lines[current_line].trim().is_empty() && current_line + 1 < lines.len() {
            current_line += 1;
        }

        *cursor_pos = self.get_position_from_line_col(text, current_line, 0);
    }

    // Character search
    fn find_char(
        &mut self,
        text: &str,
        cursor_pos: &mut usize,
        target: char,
        direction: CharSearchDirection,
        search_type: CharSearchType,
        count: usize,
    ) {
        let chars: Vec<char> = text.chars().collect();
        let mut pos = *cursor_pos;
        let mut found_count = 0;

        match direction {
            CharSearchDirection::Forward => {
                pos += 1; // Start searching from next character
                while pos < chars.len() && found_count < count {
                    if chars[pos] == target {
                        found_count += 1;
                        if found_count == count {
                            match search_type {
                                CharSearchType::To => *cursor_pos = pos,
                                CharSearchType::Before => {
                                    if pos > 0 {
                                        *cursor_pos = pos - 1;
                                    }
                                }
                            }
                            self.last_char_search = Some((target, direction, search_type));
                            return;
                        }
                    }
                    pos += 1;
                }
            }
            CharSearchDirection::Backward => {
                if pos > 0 {
                    pos -= 1;
                }
                loop {
                    if chars[pos] == target {
                        found_count += 1;
                        if found_count == count {
                            match search_type {
                                CharSearchType::To => *cursor_pos = pos,
                                CharSearchType::Before => {
                                    if pos < chars.len() - 1 {
                                        *cursor_pos = pos + 1;
                                    }
                                }
                            }
                            self.last_char_search = Some((target, direction, search_type));
                            return;
                        }
                    }
                    if pos == 0 {
                        break;
                    }
                    pos -= 1;
                }
            }
        }
    }

    // Helper functions
    fn get_line_and_column(&self, text: &str, pos: usize) -> (usize, usize) {
        let mut current_pos = 0;
        let mut line = 0;

        for line_text in text.lines() {
            let line_len = line_text.len() + 1; // +1 for newline
            if current_pos + line_len > pos {
                return (line, pos - current_pos);
            }
            current_pos += line_len;
            line += 1;
        }

        (line, 0)
    }

    fn get_position_from_line_col(&self, text: &str, line: usize, col: usize) -> usize {
        let mut pos = 0;
        let mut current_line = 0;

        for line_text in text.lines() {
            if current_line == line {
                return pos + col.min(line_text.len());
            }
            pos += line_text.len() + 1; // +1 for newline
            current_line += 1;
        }

        pos
    }

    pub fn get_mode_string(&self) -> String {
        match self.mode {
            VimMode::Normal => {
                if !self.count_buffer.is_empty() {
                    format!("NORMAL - {}", self.count_buffer)
                } else {
                    "NORMAL".to_string()
                }
            }
            VimMode::Insert => "INSERT".to_string(),
        }
    }
}