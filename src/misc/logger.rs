// use std::fs::{File, OpenOptions};
// use std::io::{self, Read, Write};
// use std::path::Path;

// Set the log file path
// const LOG_FILE_PATH: &str = "log.txt";

// Initialize the logger by truncating the log file
#[macro_export]
macro_rules! init_logger {
    () => {
        let _ = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open("log.log")
            .expect("Failed to initialize log file");
    };
}

// Log a message to the file, appending a new line by default
#[macro_export]
macro_rules! log {
    ($($arg:tt)*) => {
        {
            let mut file = OpenOptions::new()
                .create(true)
                .append(true)
                .open("log.log")
                .expect("Failed to open log file");

            // Write the formatted message to the log file, appending a new line
            writeln!(&mut file, $($arg)*).expect("Failed to write to log file");
        }
    };
}
