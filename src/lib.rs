//! Show spinners using ASCII characters.

use std::sync::{mpsc, Arc, Mutex};
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

type StopSpinner = u8;

pub struct Spinner {
    data: SpinnerData<'static>,
    text: String,
    position: Position,
    sender: Option<mpsc::Sender<StopSpinner>>,
}

impl Spinner {
    fn with_data(data: SpinnerData<'static>) -> Self {
        Self {
            data,
            text: "".to_string(),
            position: Position::After,
            sender: None,
        }
    }

    pub fn start(&mut self) {
        let interval = self.data.interval as u64;
        let frames: Vec<String> = self.data.frames.iter().map(|s| s.to_string()).collect();
        let text = self.text.to_owned();
        let position = self.position;
        let (sender, receiver) = mpsc::channel::<StopSpinner>();
        self.sender = Some(sender);
        let join_handle = thread::spawn(move || {
            let frames = &frames;

            // Hide cursor
            print!("{}?25l", ESC_SEQ);

            loop {
                for frame in frames {
                    match position {
                        Position::Before => {
                            print!("{}{}", frame, text);
                        }
                        Position::After => {
                            print!("{}{}", text, frame);
                        }
                    }
                    let count = text.len() + frame.len();
                    print!("{}{}D{}K", ESC_SEQ, count, ESC_SEQ);
                    thread::sleep(Duration::from_micros(interval * 100));
                }

                match receiver.try_recv() {
                    Ok(_) => break,
                    Err(e) => {
                        if mpsc::TryRecvError::Disconnected == e {
                            break;
                        }
                    }
                }
            }

            // Show cursor
            print!("{}?25h", ESC_SEQ);
        });
        join_handle.join().unwrap();
    }

    pub fn stop(&self) {
        if let Some(ref sender) = self.sender {
            if let Err(e) = sender.send(1) {
                eprint!("Send error: {:#?}", e);
            }
        }
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
///     spinner.start();
/// }
/// ```
#[derive(Copy, Clone, Eq, PartialEq)]
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
        let mut spinner = SpinnerBuilder::new().text("Fetching data").build();
        spinner.start();
    }
}
