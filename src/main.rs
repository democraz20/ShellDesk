#![allow(non_snake_case)]
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


#[derive(Eq, Hash, PartialEq, Debug, Clone)]
enum DirChange {
    Create(String),
    Remove(String)
}

//FileSystem items
#[derive(Debug, Eq, PartialEq, Hash, Clone)]
enum FSItem {
    File(String),
    Directory(String)
}

struct ProgramState {
    currentdir: HashSet<FSItem>
}

fn main() -> io::Result<()>{
    // env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    // let path = std::env::args()
    //     .nth(1)
    //     .expect("Argument 1 needs to be a path");

    // execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    
    let path = "".to_string();
    
    //init stage
    let mut pr = ProgramState { currentdir: HashSet::new() };

    let entries = std::fs::read_dir(".")?;
    let mut current_state: HashSet<FSItem> = HashSet::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Check if it's a file or directory and print its name
        if path.is_file() {
            // println!("File: {:?}", path.file_name().unwrap());
            let path = format!("{:?}", path.file_name().unwrap_or_else(|| {
                std::ffi::OsStr::new("Unable to retrieve file name")
            }));
            current_state.insert(FSItem::File(path));
        } else if path.is_dir() {
            // println!("Directory: {:?}", path.file_name().unwrap());
            let path = format!("{:?}", path.file_name().unwrap_or_else(|| {
                std::ffi::OsStr::new("Unable to retrieve Directory name")
            }));
            current_state.insert(FSItem::Directory(path));
        }
    }

    pr.currentdir = current_state;


    //end of init, main event loop (?)
    loop {
        println!("test");
        match event::read()? {
            Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                // Exit the loop on Ctrl + Q.
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
        //checking for directory change
        match check_dir_change(&pr.currentdir) {
            Ok((changes, new_state)) => {
                //process changes
                match changes {
                    Some(c) => {
                        println!("CHANGES : ");
                        dbg!(c);
                        //update pr status
                        pr.currentdir = new_state
                    },
                    None => {}
                }
            },
            Err(_) => {
                
            }
        }

    }
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;


    Ok(())
}

//Result<(WHAT changed, current state), error>
fn check_dir_change(last_state: &HashSet<FSItem>) -> Result<(Option<HashSet<DirChange>>, HashSet<FSItem>), Box<dyn Error>> {
    let entries = std::fs::read_dir(".")?;
    let mut current_state: HashSet<FSItem> = HashSet::new();
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        // Check if it's a file or directory and print its name
        if path.is_file() {
            // println!("File: {:?}", path.file_name().unwrap());
            let path = format!("{:?}", path.file_name().unwrap_or_else(|| {
                std::ffi::OsStr::new("Unable to retrieve file name")
            }));
            current_state.insert(FSItem::File(path));
        } else if path.is_dir() {
            // println!("Directory: {:?}", path.file_name().unwrap());
            let path = format!("{:?}", path.file_name().unwrap_or_else(|| {
                std::ffi::OsStr::new("Unable to retrieve Directory name")
            }));
            current_state.insert(FSItem::Directory(path));
        }
    }
    if current_state != *last_state {
        dbg!(&current_state);
        dbg!(last_state);
        //check what changed
        let intersect: HashSet<FSItem> = last_state.intersection(&current_state).cloned().collect();
        if intersect.is_empty() { //nothing changed
            return Ok((None, current_state))
        } else { //things changed
            let mut changes: HashSet<DirChange> = HashSet::new();
            for i in intersect {
                //check if exists or not, to determine deleted or created
                // let filename: String = match i {
                let f = match i {
                    FSItem::File(f) => f,
                    FSItem::Directory(f) => f
                };
                if Path::new(&f).exists() {
                    //created
                    changes.insert(DirChange::Create(f));
                } else {
                    changes.insert(DirChange::Remove(f));
                }
            }
            return Ok((Some(changes), current_state))
        }
    }
    Ok((None, HashSet::new()))
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