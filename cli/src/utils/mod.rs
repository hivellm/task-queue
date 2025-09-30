//! Utility functions and helpers

use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

pub struct ProgressManager {
    multi: MultiProgress,
}

impl ProgressManager {
    pub fn new() -> Self {
        Self {
            multi: MultiProgress::new(),
        }
    }
    
    pub fn create_task_progress(&self, task_name: &str) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new_spinner());
        pb.set_style(
            ProgressStyle::default_spinner()
                .template("{spinner:.green} {msg}")
                .unwrap()
                .tick_strings(&["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"]),
        );
        pb.set_message(format!("Processing task: {}", task_name));
        pb
    }
    
    pub fn create_download_progress(&self, total: u64) -> ProgressBar {
        let pb = self.multi.add(ProgressBar::new(total));
        pb.set_style(
            ProgressStyle::default_bar()
                .template("{bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
                .unwrap()
                .progress_chars("█▉▊▋▌▍▎▏  "),
        );
        pb
    }
}