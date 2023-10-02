#![allow(non_snake_case)]
use std::fs::{DirEntry, ReadDir};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};
use std::thread;

use crossterm::{
    cursor,
    cursor::MoveTo,
    event::{self, Event::Key, KeyCode, KeyEvent, KeyEventKind, KeyModifiers},
    execute,
    style::Stylize,
    terminal,
    terminal::enable_raw_mode,
    terminal::{disable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    fs,
    io::{self, Write},
};

use std::io::{stdout, Seek};

fn main() -> Result<(), Box<dyn std::error::Error>> {
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
    execute!(stdout(), MoveTo(1, 1))?; //1 cell margin

    let ORIGINTESTPATH = "./";

    let dirItems = fs::read_dir(ORIGINTESTPATH)?;
    let dirItems = custom_sort(dirItems);
    // println!("=LIST=");
    let (cursorX, cursorY) = cursor::position()?;
    let (columns, rows) = terminal::size()?;

    let longest_item: String;
    match dirItems.iter().max_by_key(|s| s.len()) {
        Some(s) => {
            longest_item = String::from(s);
        }
        None => {
            longest_item = String::from("Directory is empty");
        }
    }
    let Ndisplayable: i32 = ((columns - 2) as f32 / (longest_item.len() + 2) as f32).floor() as i32;
    // let Ndisplayable = div.floor();

    let mut displayablecounter = 0;
    let mut currentline = 0;

    let totallines = (dirItems.len() as f32 / Ndisplayable as f32).ceil() as i32;

    execute!(stdout(), MoveTo(2, 1))?;
    for path in dirItems.clone() {
        // println!("Name: {}", path);
        // print!("");
        stdout().flush()?;

        if displayablecounter == Ndisplayable {
            displayablecounter = 0;
            currentline += 2;
            if currentline >= rows - 2 {
                break;
            }
            execute!(stdout(), MoveTo(2, 1 + currentline))?;
        }
        displayablecounter += 1;
        let pathbuf: PathBuf = [ORIGINTESTPATH, &path].iter().collect();
        if pathbuf.is_dir() {
            print!(
                "{}{}",
                path.clone().blue().bold(),
                " ".repeat(longest_item.len() - path.len() + 2)
            );
        }
        // else if pathbuf.is_file() {
        //     print!("{}{}", path.clone().green().bold(), " ".repeat(longest_item.len()-path.len()+2));
        // }
        else {
            print!(
                "{}{}",
                path,
                " ".repeat(longest_item.len() - path.len() + 2)
            );
        }
    }

    print_borders(columns, rows)?;
    execute!(stdout(), MoveTo(1, 0))?;
    print!(
        "termsize : {}x{}, longest item: {}, n displayable: {}, total lines: {}",
        columns,
        rows,
        longest_item.len(),
        Ndisplayable,
        totallines
    );
    stdout().flush()?;

    let mut selected: usize = 0;
    if dirItems.len() > 0 {
        //selecting first item
        execute!(stdout(), MoveTo(1, 1))?;
        print!(">");
        execute!(stdout(), MoveTo(dirItems[0].len() as u16 + 2, 1))?;
        print!("<");
        stdout().flush()?;
    }
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
            }

            Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if dirItems.len() > 0 {
                    if !(selected >= dirItems.len() - 1) {
                        selected += 1;
                        
                        let top_margin = 1;
                        let currentline: u16 = (selected / Ndisplayable as usize + 1) as u16;

                        execute!(stdout(), MoveTo(1, rows - 2))?;
                        print!("currentline: {}", currentline);
                        stdout().flush()?;
                        // execute!(stdout(), MoveTo(1, rows-2))?;
                        // print!("adding 1, is now {}", selected);
                        // if selected+1 <= Ndisplayable as usize && selected < dirItems.len() {
                        //     let pos = find_pos_of_selected(&dirItems, selected, Ndisplayable, 3, 2);
                        // }
                        // //2 line down
                        // if selected+1 > Ndisplayable as usize && selected < dirItems.len() {
                        //     let pos = find_pos_of_selected(&dirItems, selected, Ndisplayable, 3, 2);
                            
                        // }
                        if selected < dirItems.len() {
                            let pos = find_pos_of_selected(&dirItems, selected, Ndisplayable, 3, 2);
                            let pre_pos = find_pos_of_selected(&dirItems, selected-1, Ndisplayable, 3, 2);
                        }
                    }
                    //finding current line

                    // if selected > Ndisplayable as usize && selected < dirItems.len() {

                    // }
                }
                // execute!(stdout(), MoveTo(1, rows-1))?;
                stdout().flush()?;
            }

            Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if dirItems.len() > 0 {
                    if selected >= 1 {
                        // execute!(stdout(), MoveTo(1, rows-2))?;
                        // print!("subtracting 1, is now {}", selected);
                        selected -= 1;
                        if selected + 1 <= Ndisplayable as usize && selected + 1 < dirItems.len() {
                            let previous_pos: u16 =
                                ((selected as u16) + 1) * (longest_item.len() + 2) as u16 + 1;

                            execute!(stdout(), MoveTo(previous_pos, 1))?;
                            print!(" ");
                            execute!(
                                stdout(),
                                MoveTo(previous_pos + dirItems[selected + 1].len() as u16 + 1, 1)
                            )?;
                            print!(" ");

                            let pos: u16 =
                                ((selected as u16) * (longest_item.len() as u16 + 2)) + 1;

                            execute!(stdout(), MoveTo(pos, 1))?;
                            print!(">");
                            execute!(
                                stdout(),
                                MoveTo(pos + dirItems[selected].len() as u16 + 1, 1)
                            )?;
                            print!("<");
                            stdout().flush()?;
                        }
                        // selected -= 1;
                    }
                }
                // execute!(stdout(), MoveTo(1, rows-1))?;
                stdout().flush()?;
            }

            Key(KeyEvent {
                code: KeyCode::F(5),
                kind: KeyEventKind::Press,
                ..
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
    let mut file_list: Vec<DirEntry> = read_dir.filter_map(Result::ok).collect();

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
        .collect();
}

fn print_borders(X: u16, Y: u16) -> Result<(), Box<dyn std::error::Error>> {
    let top_left = '\u{02554}';
    let top_right = '\u{02557}';
    let bottom_left = '\u{0255A}';
    let bottom_right = '\u{0255D}';

    let vertical = '\u{02551}';
    let horizontal = '\u{02550}';

    //corners

    execute!(stdout(), MoveTo(0, 0))?;
    print!("{}", top_left); //╔

    execute!(stdout(), MoveTo(X, 0))?;
    print!("{}", top_right);

    execute!(stdout(), MoveTo(0, Y))?;
    print!("{}", bottom_left);

    execute!(stdout(), MoveTo(X, Y))?;
    print!("{}", bottom_right);

    //sides

    execute!(stdout(), MoveTo(1, 0))?;
    print!("{}", String::from(horizontal).repeat((X - 2).into()));

    execute!(stdout(), MoveTo(1, Y))?;
    print!("{}", String::from(horizontal).repeat((X - 2).into()));

    for i in 0..Y - 2 {
        execute!(stdout(), MoveTo(0, i + 1))?;
        print!("{}", vertical);
    }

    for i in 0..Y - 2 {
        execute!(stdout(), MoveTo(X, i + 1))?;
        print!("{}", vertical);
    }

    Ok(())
}

fn find_pos_of_selected(items: &Vec<String>, selected: usize, nDisplayable: i32, topPadding: usize, sidePadding: usize) -> ((u16, u16), (u16, u16)) {
    return ((0,0),(0,0))
}