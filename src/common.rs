use std::io::{Write, Read, self};
use chrono::{Local, format};

pub fn pause() {
    let mut stdin = io::stdin();
    let mut stdout = io::stdout();

    // We want the cursor to stay at the end of the line, so we print without a newline and flush manually.
    write!(stdout, "\nPress enter to continue... or Ctrl + C to break!!!").unwrap();
    stdout.flush().unwrap();

    // Read a single byte and discard
    let _ = stdin.read(&mut [0u8]).unwrap();
}

// Gives you local date time
pub fn local_dt() -> format::DelayedFormat<format::StrftimeItems<'static>> {
    let date = Local::now();
    date.format("%Y-%m-%dT%H:%M:%S")
}