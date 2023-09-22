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
    GetForegroundWindow, ShowWindow, SW_MAXIMIZE, SWP_FRAMECHANGED, GWL_STYLE, SW_HIDE, SW_RESTORE,
    GetWindowLongA, SetWindowLongA, WS_POPUP, WS_CAPTION, WS_SYSMENU, WS_THICKFRAME, WS_MINIMIZEBOX, WS_MAXIMIZEBOX,
};

use std::io::stdout;

fn main() -> Result<(), Box<dyn std::error::Error>>{

    //window prep
    unsafe {
        // Get the handle of the currently focused window
        let hwnd = GetForegroundWindow();

        if hwnd.is_null() {
            panic!("Failed to get the foreground window handle");
        }

        // Check if the window is maximized
        let is_maximized = GetWindowLongA(hwnd, GWL_STYLE) & WS_MAXIMIZEBOX as i32 != 0;

        // If the window is maximized, restore it to normal state
        if is_maximized {
            ShowWindow(hwnd, SW_RESTORE);
        }

        // Remove the WS_CAPTION style to hide the window title bar
        let mut window_style = GetWindowLongA(hwnd, GWL_STYLE) as u32;
        window_style &= !(WS_CAPTION | WS_SYSMENU | WS_THICKFRAME | WS_MINIMIZEBOX | WS_MAXIMIZEBOX);
        SetWindowLongA(hwnd, GWL_STYLE, window_style as i32);

        // Maximize the window
        ShowWindow(hwnd, SW_MAXIMIZE);
        ShowWindow(hwnd, SWP_FRAMECHANGED.try_into().unwrap());

        println!("Window maximized to full-screen.");
    }


    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    let paths = fs::read_dir("./").unwrap();
    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }


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
                break;
            },
            Key(KeyEvent{
                code:KeyCode::F(5),
                kind: KeyEventKind::Press, ..
            }) => {

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

