use eframe::egui;

/// Struct for command input states
///
/// # Fields
///
/// - `open` (`bool`) - State for whether command palette is open
/// - `input` (`String`) - Input from the user for the text field
///
/// # Examples
///
/// ```
/// use crate::...;
///
/// let s = CommandInput {
///     open: value,
///     input: value,
/// };
/// ```
pub struct CommandInput {
    pub open: bool,
    pub input: String,
}

impl Default for CommandInput {
    fn default() -> Self {
        Self {
            open: false,
            input: String::new(),
        }
    }
}

impl CommandInput {
    /// Opens the command input modal and clears any previous input.
    ///
    /// # Arguments
    ///
    /// - `&mut self` - The CommandInput instance to open.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hotkey::command_input::CommandInput;
    ///
    /// let mut cmd = CommandInput::default();
    /// cmd.open();
    /// ```
    pub fn open(&mut self) {
        self.open = true;
        self.input.clear();
    }

    /// Describe this function.
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`undefined`) - Describe this parameter.
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::...;
    ///
    /// let _ = close();
    /// ```
    pub fn close(&mut self) {
        self.open = false;
    }

    /// This function is used for returning the suggestions based on user input for the command palette
    ///
    /// # Arguments
    ///
    /// - `&mut self` (`undefined`) - Describe this parameter.
    /// - `ctx` (`&egui`) - Describe this parameter.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Returns all the suggestions as strings
    ///
    /// # Examples
    ///
    /// ```
    /// use crate::hotkey::command_input;
    ///
    /// let _ = show(ctx, |ui|...);
    /// ```
    pub fn show(&mut self, ctx: &egui::Context) -> Option<String> {
        if !self.open {
            return None;
        }

        let mut submitted_command = None;

        egui::Window::new("cmd_input_modal")
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .anchor(egui::Align2::CENTER_TOP, egui::vec2(0.0, 100.0))
            .fixed_size(egui::vec2(500.0, 40.0))
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(":")
                            .text_style(egui::TextStyle::Monospace)
                            .strong(),
                    );

                    let response = ui.add_sized(
                        ui.available_size(),
                        egui::TextEdit::singleline(&mut self.input)
                            .hint_text("Enter Vim command...")
                            .font(egui::TextStyle::Monospace)
                            .lock_focus(true),
                    );

                    response.request_focus();

                    if ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                        submitted_command = Some(self.input.clone());
                        self.close();
                    }
                    if ui.input(|i| i.key_pressed(egui::Key::Escape)) {
                        self.close();
                    }
                });
            });

        submitted_command
    }
}
