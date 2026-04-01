use viuer::{print_from_file, Config};
use std::{thread, time::Duration};

fn main () {
    let conf = Config {
    // Set dimensions.
    width: Some(80),
    height: Some(25),
    ..Default::default()
    };

    // Display `img.jpg` with dimensions 80×25 in terminal cells.
    // The image resolution will be 80×50 because each cell contains two pixels.
    print_from_file("dune.jpeg", &conf).expect("Image printing failed.");
    thread::sleep(Duration::from_secs(4));
    print_from_file("emperor.jpg", &conf).expect("Image printing failed.");
}
