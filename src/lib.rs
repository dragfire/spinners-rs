//! Show spinners using ASCII characters.

use std::thread;
use std::time::Duration;

include!(concat!(env!("OUT_DIR"), "/spinner.rs"));

const ESC_SEQ: &'static str = "\x1B[";

pub struct SpinnerData<'a> {
    interval: u8,
    frames: Vec<&'a str>,
}

impl<'a> SpinnerData<'a> {
    fn new(interval: u8, frames: Vec<&'a str>) -> Self {
        Self { interval, frames }
    }
}

pub struct Spinner {
    data: SpinnerData<'static>,
    text: String,
    position: Position,
}

impl Spinner {
    fn with_data(data: SpinnerData<'static>) -> Self {
        Self {
            data,
            text: "".to_string(),
            position: Position::After,
        }
    }

    pub fn start(&self) {
        let interval = self.data.interval as u64;
        let frames: Vec<String> = self.data.frames.iter().map(|s| s.to_string()).collect();
        let join_handle = thread::spawn(move || {
            let frames = &frames;
            loop {
                for frame in frames {
                    print!("{}1K{}", ESC_SEQ, frame);
                    thread::sleep(Duration::from_millis(interval));
                }
            }
        });
        join_handle.join().unwrap();
    }
}

/// Specify the position of the Spinner before or after text.
/// Default position is After.
///
/// Example:
/// ```rust
/// fn main() {
///     let spinner = SpinnerBuilder::new()
///     .text("Fetching data...")
///     .position(Position::Before).build();
///
///     spinner.stop();
/// }
/// ```
pub enum Position {
    /// Place Spinner before text.
    Before,

    /// Place Spinner after text.
    After,
}

pub struct SpinnerBuilder {
    spinner: Option<Spinner>,
}

impl SpinnerBuilder {
    pub fn new() -> Self {
        Self {
            spinner: Some(Spinner::new(SpinnerType::Aesthetic)),
        }
    }

    pub fn spinner(mut self, spinner_type: SpinnerType) -> Self {
        self.spinner = Some(Spinner::new(spinner_type));
        self
    }

    pub fn text(mut self, text: &str) -> Self {
        if let Some(ref mut spinner) = self.spinner {
            spinner.text = text.to_string();
        }
        self
    }

    pub fn position(mut self, position: Position) -> Self {
        if let Some(ref mut spinner) = self.spinner {
            spinner.position = position;
        }
        self
    }

    pub fn build(self) -> Spinner {
        self.spinner.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let spinner = SpinnerBuilder::new().text("Fetching data").build();
        spinner.start();
    }
}
