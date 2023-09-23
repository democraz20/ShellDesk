#![allow(non_snake_case)]
use std::fs::{ReadDir, DirEntry};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::{
    event::{self, KeyCode, KeyEvent, KeyEventKind, KeyModifiers, Event::Key},
    execute, terminal::{EnterAlternateScreen, disable_raw_mode, LeaveAlternateScreen},
    terminal,
    terminal::{
        enable_raw_mode
    },
    style::Stylize,
    cursor,
    cursor::MoveTo
};
use std::{
    io::{self, Write}, 
    error::Error, 
    fmt::format, 
    hash::Hash, 
    iter::Cloned, 
    path::Path,
    fs
};

use std::ptr::null_mut;
use winapi::um::winuser::{
    GetForegroundWindow, ShowWindow, SW_MAXIMIZE, SWP_FRAMECHANGED, GWL_STYLE, SW_RESTORE,
    GetWindowLongA, SetWindowLongA, WS_CAPTION, WS_SYSMENU, WS_THICKFRAME, WS_MINIMIZEBOX, WS_MAXIMIZEBOX,
};

use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    //window prep
    // unsafe {
    //     // Get the handle of the currently focused window
    //     let hwnd = GetForegroundWindow();

    //     if hwnd.is_null() {
    //         panic!("Failed to get the foreground window handle");
    //     }

    //     // Check if the window is maximized
    //     let is_maximized = GetWindowLongA(hwnd, GWL_STYLE) & WS_MAXIMIZEBOX as i32 != 0;

    //     // If the window is maximized, restore it to normal state
    //     if is_maximized {
    //         ShowWindow(hwnd, SW_RESTORE);
    //     }

    //     // Remove the WS_CAPTION style to hide the window title bar
    //     let mut window_style = GetWindowLongA(hwnd, GWL_STYLE) as u32;
    //     window_style &= !(WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX);
    //     SetWindowLongA(hwnd, GWL_STYLE, window_style as i32);

    //     // Maximize the window
    //     ShowWindow(hwnd, SW_MAXIMIZE);
    //     ShowWindow(hwnd, SWP_FRAMECHANGED.try_into().unwrap());

    //     println!("Window maximized to full-screen.");
    // }


    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    execute!(stdout(), MoveTo(1,1))?; //1 cell margin

    let paths = fs::read_dir("./")?;
    let paths = custom_sort(paths);
    println!("=PATHS=");
    let (cursorX, cursorY) = cursor::position()?;
    let (columns, rows) = terminal::size()?;

    execute!(stdout(), MoveTo(cursorX+2,cursorY))?;

    for path in paths.clone() {
        // println!("Name: {}", path);
        // print!("");
        stdout().flush()?;
        if PathBuf::from(&path).is_dir() {
            print!("{}  ", path.blue().bold());
        } else {
            print!("{}  ", path);
        }
    }

    stdout().flush()?;

    let mut selected: usize = 0;
    loop {
        match event::read()? {
            Key(KeyEvent {
                code: KeyCode::Char('q'),
                modifiers: KeyModifiers::CONTROL,
                kind: KeyEventKind::Press,
                state: _,
            }) => {
                // Exit the loop on Ctrl + Q.
                // break;
                break;
            },

            Key(KeyEvent{
                code: KeyCode::Right,
                kind: KeyEventKind::Press, ..
            }) => {
                if !(selected > paths.len()-1) {
                    selected += 1;
                    execute!(stdout(), MoveTo(1, rows-2))?;
                    print!("adding 1, is now {}", selected);
                }
                execute!(stdout(), MoveTo(1, rows-1))?;
                stdout().flush()?;
            }

            Key(KeyEvent{
                code: KeyCode::Left,
                kind: KeyEventKind::Press, ..
            }) => {
                if selected > 1 {
                    selected -= 1;
                    execute!(stdout(), MoveTo(1, rows-2))?;
                    print!("subtracting 1, is now {}", selected);
                }
                execute!(stdout(), MoveTo(1, rows-1))?;
                stdout().flush()?;
            }


            Key(KeyEvent{
                code:KeyCode::F(5),
                kind: KeyEventKind::Press, ..
            }) => {
                //refresh code
            }
            _ => {}
            
        }
    }

    let handle = thread::spawn(move || {
        //handle file changes here

    });
    
    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;
    // Join the spawned thread.
    handle.join().unwrap();
    Ok(())
}


fn custom_sort(read_dir: ReadDir) -> Vec<String> {
    let mut file_list: Vec<DirEntry> = read_dir
        .filter_map(Result::ok)
        .collect();

    file_list.sort_by(|a, b| {
        let a_name = a.file_name().to_string_lossy().to_string();
        let b_name = b.file_name().to_string_lossy().to_string();

        let is_hidden_a = a_name.starts_with('.');
        let is_hidden_b = b_name.starts_with('.');
        let is_directory_a = a.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
        let is_directory_b = b.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        match (is_hidden_a, is_directory_a, is_hidden_b, is_directory_b) {
            (true, true, _, _) => std::cmp::Ordering::Less,
            (_, _, true, true) => std::cmp::Ordering::Greater,
            (true, false, _, _) => std::cmp::Ordering::Less,
            (_, _, false, true) => std::cmp::Ordering::Greater,
            (_, true, _, _) => std::cmp::Ordering::Less,
            (_, _, _, _) => a_name.cmp(&b_name),
        }
    });

    return file_list
        .iter()
        .map(|entry| entry.file_name().to_string_lossy().to_string())
        .collect()
}