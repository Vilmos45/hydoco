/// Application.
#[derive(Debug, Default)]
pub struct App {
    /// should the application exit?
    pub should_quit: bool,
    /// tab
    pub tab: u8,
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set should_quit to true to quit the application.
    pub fn quit(&mut self) {
        self.should_quit = true;
    }

    pub fn next_tab(&mut self) {
        if let Some(res) = self.tab.checked_add(1) {
            self.tab = res % 6;//6 tabs
        }
    }

    pub fn previous_tab(&mut self) {
        if let Some(res) = self.tab.checked_sub(1) {
            self.tab = res % 6;
        }
        else if self.tab == 0 {
            self.tab = 5;
        }
    }
}