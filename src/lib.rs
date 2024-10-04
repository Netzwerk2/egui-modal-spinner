//! egui-modal-spinner
#![warn(missing_docs)] // Let's keep the public API well documented!

use std::time::SystemTime;

use egui::Widget;

/// Represents the state the spinner is currently in.
#[derive(Debug, Clone, PartialEq)]
pub enum SpinnerState {
    /// The spinner is currently closed and not visible.
    Closed,
    /// The spinner is currently open and user input is suppressed.
    /// The value is the timestamp when the spinner was opened
    /// This is used to display the elapsed time.
    Open(SystemTime),
}

/// Represents a spinner instance.
#[derive(Debug, Clone, PartialEq)]
pub struct ModalSpinner {
    state: SpinnerState,
    id: egui::Id,

    fill_color: egui::Color32,
    spinner: Spinner,
    show_elapsed_time: bool,
}

/// Creation methods
impl ModalSpinner {
    /// Creates a new spinner instance.
    pub fn new() -> Self {
        Self {
            state: SpinnerState::Closed,
            id: egui::Id::from("_modal_spinner"),

            fill_color: egui::Color32::from_rgba_premultiplied(0, 0, 0, 120),
            spinner: Spinner::default(),
            show_elapsed_time: false,
        }
    }

    /// Sets the ID of the spinner.
    pub fn id(mut self, id: impl Into<egui::Id>) -> Self {
        self.id = id.into();
        self
    }

    /// Sets the fill color of the modal background.
    pub fn fill_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.fill_color = color.into();
        self
    }

    /// Sets the size of the spinner.
    pub fn spinner_size(mut self, size: f32) -> Self {
        self.spinner.size = Some(size);
        self
    }

    /// Sets the color of the spinner.
    pub fn spinner_color(mut self, color: impl Into<egui::Color32>) -> Self {
        self.spinner.color = Some(color.into());
        self
    }

    /// If the elapsed time should be displayed below the spinner.
    pub fn show_elapsed_time(mut self, show_elapsed_time: bool) -> Self {
        self.show_elapsed_time = show_elapsed_time;
        self
    }
}

/// Getter and setter
impl ModalSpinner {
    /// Gets the current state of the spinner.
    pub fn state(&self) -> &SpinnerState {
        &self.state
    }
}

/// Implementation methods
impl ModalSpinner {
    /// Opens the spinner.
    pub fn open(&mut self) {
        self.state = SpinnerState::Open(SystemTime::now());
    }

    /// Closes the spinner.
    pub fn close(&mut self) {
        self.state = SpinnerState::Closed;
    }

    /// Main update method of the spinner that should be called every frame if you want the
    /// spinner to be visible.
    ///
    /// This has no effect if the `SpinnerState` is currently not `SpinnerState::Open`.
    pub fn update(&mut self, ctx: &egui::Context) {
        if !matches!(self.state, SpinnerState::Open(_)) {
            return;
        }

        let screen_rect = ctx.input(|i| i.screen_rect);

        ctx.style_mut(|s| s.visuals.window_fill = self.fill_color);

        let re = egui::Area::new(self.id)
            .interactable(true)
            .movable(false)
            .fixed_pos(screen_rect.left_top())
            .sense(egui::Sense::click())
            .show(ctx, |ui| {
                ui.painter()
                    .rect_filled(screen_rect, egui::Rounding::ZERO, self.fill_color);

                let child_ui = egui::UiBuilder::new()
                    .max_rect(screen_rect)
                    .layout(egui::Layout::top_down(egui::Align::Center));

                ui.allocate_new_ui(child_ui, |ui| {
                    let spinner_h = self
                        .spinner
                        .size
                        .unwrap_or_else(|| ui.style().spacing.interact_size.y);

                    ui.add_space(screen_rect.height() / 2.0 - spinner_h / 2.0);

                    self.spinner.update(ui);
                });
            });

        ctx.move_to_top(re.response.layer_id);
    }
}

/// This tests if the spinner is send and sync.
#[cfg(test)]
const fn test_prop<T: Send + Sync>() {}

#[test]
const fn test() {
    test_prop::<ModalSpinner>();
}

/// Wrapper above `egui::Spinner` to be able to customize trait implementations.
#[derive(Debug, Clone, PartialEq)]
struct Spinner {
    pub size: Option<f32>,
    pub color: Option<egui::Color32>,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: None,
            color: None,
        }
    }
}

impl Spinner {
    fn update(&self, ui: &mut egui::Ui) -> egui::Response {
        let mut spinner = egui::Spinner::new();

        if let Some(size) = self.size {
            spinner = spinner.size(size);
        }

        if let Some(color) = self.color {
            spinner = spinner.color(color);
        }

        spinner.ui(ui)
    }
}
