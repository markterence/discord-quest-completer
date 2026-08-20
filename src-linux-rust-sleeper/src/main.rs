use std::{thread, time};

fn main() {
    // Sleep for 1 hour (3600 seconds) to simulate an active game
    thread::sleep(time::Duration::from_secs(3600));
}
