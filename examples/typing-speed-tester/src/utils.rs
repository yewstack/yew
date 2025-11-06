use gloo::timers::callback::Interval;

pub(crate) const MAX_FONT_SIZE: i32 = 50i32;
pub(crate) const MIN_FONT_SIZE: i32 = 10i32;
pub(crate) const DEFAULT_TEXT: &str =
    "Hi! Welcome to this simple and free typing speed test tool. Click on this paragraph to start \
     typing, the timer will start by itself. When you type something wrong, it lights up red, \
     else it lights up white. Click on the '+/-' buttons to increase available minutes or if you \
     feel \"pressured\", you can pause the timer. Reset progress by clicking the button below \
     this paragraph. Feel free to upload your own text too! Happy typing:)";

pub(crate) const INITIAL_TIME_MINUTES: u32 = 5_u32;
pub(crate) const ADJUSTMENT_TIME_MINUTES: i32 = 1_i32;
pub(crate) const SPECIAL_KEYS: [&str; 9] = [
    "Shift", "Control", "Tab", "Alt", "Enter", "Meta", "Windows", "Command", "Super",
];

#[derive(Debug)]
pub(crate) struct AppUtils {
    pub font_size: i32,

    // for the timer
    /// Total time remaining in seconds (u because seconds can never be -ve)
    pub duration_seconds: u32,

    /// Tracks whether timer is running or paused
    pub timer_running: bool,

    /// Handle for the interval timer. Kept in state to be dropped when pausing/unmounting.
    pub interval_handle: Option<Interval>,

    /// Handle whether modal is open
    pub modal_open: bool,
}

impl Default for AppUtils {
    fn default() -> Self {
        Self {
            font_size: 30,
            duration_seconds: INITIAL_TIME_MINUTES * 60,
            timer_running: false,
            interval_handle: None,
            modal_open: false,
        }
    }
}

impl AppUtils {
    pub(crate) fn inc_font_size(&mut self) {
        self.font_size += 10;
    }

    pub(crate) fn dec_font_size(&mut self) {
        self.font_size -= 10;
    }

    pub(crate) fn format_time(&self) -> (String, String) {
        let total_seconds = self.duration_seconds;
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;

        // Use pad_start for "05:08" formatting
        (format!("{:02}", minutes), format!("{:02}", seconds))
    }

    pub(crate) fn toggle_modal(&mut self) {
        self.modal_open = !self.modal_open;
    }
}
