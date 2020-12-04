use spinners_rs::*;

fn main() {
    let mut spinner = SpinnerBuilder::new().text("Loading data").build();
    spinner.start();
    // std::thread::sleep(std::time::Duration::from_secs(3));
    // spinner.stop();
}
