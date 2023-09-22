#![allow(non_snake_case)]
use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, Event::Key},
    execute, terminal::{EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    terminal::{
        enable_raw_mode
    }
};
use std::{io::{self, Write}, error::Error, fmt::format, hash::Hash, iter::Cloned, path::Path};
use std::collections::HashSet;
use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    
    // Create a boolean variable with an Arc (atomic reference counter) and Mutex for thread safety.
    let flag = Arc::new(Mutex::new(false));

    // Clone the Arc for the thread.
    let flag_clone = Arc::clone(&flag);
    // Spawn a new thread to modify the variable to true after a delay.
    println!("start thread");
    let handle = thread::spawn(move || {
        // Modify the variable to true.

        loop {
            match event::read().unwrap() {
                Key(KeyEvent {
                    code: KeyCode::Char('q'),
                    modifiers: KeyModifiers::CONTROL,
                    kind: KeyEventKind::Press,
                    state: _,
                }) => {
                    // Exit the loop on Ctrl + Q.
                    // break;
                    println!("exiting 1");
                    let mut data = flag_clone.lock().unwrap();
                    println!("test1");
                    *data = true;
                    println!("test2");
                    break;
                },
                Key(KeyEvent{
                    code: KeyCode::Char(c),
                    kind: KeyEventKind::Press, ..
                }) => {
                    println!("pressed : {}", c)
                }
                Key(KeyEvent{
                    code:KeyCode::F(5),
                    kind: KeyEventKind::Press, ..
                }) => {
                }
                _ => {}
                
            }
        }
        println!("end of created thread");
    });
    println!("started thread");
    // Main thread continuously checks the variable until it becomes true.
    loop {
        // Lock the mutex to access the variable.
        thread::sleep(std::time::Duration::from_secs(1));
        let data = flag.lock().unwrap();
        if *data {
            println!("Flag is true. Breaking the loop.");
            break;
        }
        drop(data); // Release the lock before sleeping to allow other threads to access it.
    }
    
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    // Join the spawned thread.
    handle.join().unwrap();
    Ok(())
}



// fn watch<P: AsRef<Path>>(path: P) -> notify::Result<()> {
//     let (tx, rx) = std::sync::mpsc::channel();

//     // Automatically select the best implementation for your platform.
//     // You can also access each implementation directly e.g. INotifyWatcher.
//     let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

//     // Add a path to be watched. All files and directories at that path and
//     // below will be monitored for changes.
//     watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

//     for res in rx {
//         match res {
//             Ok(event) => println!("Change: {event:?}"),
//             Err(error) => println!("Error: {error:?}"),
//         }
//     }

//     Ok(())
// }