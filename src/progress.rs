use crate::utils::CmprssOutput;
use indicatif::{HumanBytes, ProgressBar};

#[derive(clap::ValueEnum, Clone, Copy, Debug)]
pub enum ProgressDisplay {
    Auto,
    On,
    Off,
}

/// Progress bar for the compress process
pub struct Progress {
    /// The progress bar
    bar: ProgressBar,
    /// The number of bytes read from the input
    input_read: u64,
    /// The number of bytes written to the output
    output_written: u64,
}

/// Create a progress bar if necessary
pub fn progress_bar(
    input_size: Option<u64>,
    progress: ProgressDisplay,
    output: &CmprssOutput,
) -> Option<Progress> {
    match (progress, output) {
        (ProgressDisplay::Auto, CmprssOutput::Pipe(_)) => None,
        (ProgressDisplay::Off, _) => None,
        (_, _) => Some(Progress::new(input_size)),
    }
}

impl Progress {
    /// Create a new progress bar
    /// Draws to stderr by default
    pub fn new(input_size: Option<u64>) -> Self {
        let bar = match input_size {
            Some(size) => ProgressBar::new(size),
            None => ProgressBar::new_spinner(),
        };
        bar.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{spinner:.green} [{elapsed_precise}] ({eta}) [{bar:40.cyan/blue}] {bytes}/{total_bytes} => {msg}").unwrap()
                .progress_chars("#>-"),
        );
        Progress {
            bar,
            input_read: 0,
            output_written: 0,
        }
    }

    /// Update the progress bar with the number of bytes read from the input
    pub fn update_input(&mut self, bytes_read: u64) {
        self.input_read = bytes_read;
        self.bar.set_position(self.input_read);
    }

    /// Update the progress bar with the number of bytes written to the output
    pub fn update_output(&mut self, bytes_written: u64) {
        self.output_written = bytes_written;
        self.bar
            .set_message(HumanBytes(self.output_written).to_string());
    }

    /// Finish the progress bar
    pub fn finish(&self) {
        self.bar.finish();
    }
}