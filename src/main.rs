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
    fs
};


use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    
    // use std::ptr::null_mut;
    // use winapi::um::winuser::{
    //     GetForegroundWindow, ShowWindow, SW_MAXIMIZE, SWP_FRAMECHANGED, GWL_STYLE, SW_RESTORE,
    //     GetWindowLongA, SetWindowLongA, WS_CAPTION, WS_SYSMENU, WS_THICKFRAME, WS_MINIMIZEBOX, WS_MAXIMIZEBOX,
    // };
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

    
    let dirItems = fs::read_dir("./")?;
    let dirItems = custom_sort(dirItems);
    println!("=LIST=");
    let (cursorX, cursorY) = cursor::position()?;
    let (columns, rows) = terminal::size()?;
    
    print_borders(columns, rows)?;
    
    let longest_item: String;
    match dirItems.iter().max_by_key(|s| s.len()) {
        Some(s) => {
            longest_item = String::from(s);
        },
        None => {
            longest_item = String::from("Directory is empty");
        }
    }
    let Ndisplayable = ((columns-2) as f32 / (longest_item.len()+2) as f32).floor();
    // let Ndisplayable = div.floor();

    
    execute!(stdout(), MoveTo(1,0))?;
    print!("termsize : {}x{}, longest item: {}, n displayable: {}", columns, rows, longest_item.len(), Ndisplayable);
    stdout().flush()?;
    
    execute!(stdout(), MoveTo(cursorX+2,cursorY))?;
    for path in dirItems.clone() {
        // println!("Name: {}", path);
        // print!("");
        stdout().flush()?;
        if PathBuf::from(&path).is_dir() {
            print!("{}{}", path.clone().blue().bold(), " ".repeat(longest_item.len()-path.len()+2));
        } else {
            print!("{}{}", path, " ".repeat(longest_item.len()-path.len()+2));
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
                if !(selected > dirItems.len()-1) {
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

fn print_borders(X: u16, Y: u16) -> Result<(), Box<dyn std::error::Error>> {
    let top_left = '\u{02554}';
    let top_right = '\u{02557}';
    let bottom_left = '\u{0255A}';
    let bottom_right = '\u{0255D}';

    let vertical = '\u{02551}';
    let horizontal = '\u{02550}';

    //corners

    execute!(stdout(), MoveTo(0,0))?;
    print!("{}", top_left); //╔
    
    execute!(stdout(), MoveTo(X, 0))?;
    print!("{}", top_right); 

    execute!(stdout(), MoveTo(0, Y))?;
    print!("{}", bottom_left); 

    execute!(stdout(), MoveTo(X, Y))?;
    print!("{}", bottom_right); 

    //sides

    execute!(stdout(), MoveTo(1,0))?;
    print!("{}", String::from(horizontal).repeat((X-2).into()));

    execute!(stdout(), MoveTo(1,Y))?;
    print!("{}", String::from(horizontal).repeat((X-2).into()));

    for i in 0..Y-2 {
        execute!(stdout(), MoveTo(0,i+1))?;
        print!("{}", vertical);
    }

    for i in 0..Y-2 {
        execute!(stdout(), MoveTo(X,i+1))?;
        print!("{}", vertical);
    }

    Ok(())
}