#![allow(non_snake_case)]

use std::fs;
use std::fs::ReadDir;
use std::fs::DirEntry;
use std::io::Write;
use std::io::stdout;
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

const TOP_MARGIN: u16 = 2;
const BOTTOM_MARGIN: u16 = 1;
const RIGHT_MARGIN: u16 = 1;
const LEFT_MARGIN: u16 = 1;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ORIGINTESTPATH = "./../";
    //desktop
    let dirItems = fs::read_dir(ORIGINTESTPATH)?;
    let dirItems = custom_sort(dirItems);
    let longest_item: String;

    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    
    let (cursorX, cursorY) = cursor::position()?;
    let (columns, rows) = terminal::size()?;
    
    print_borders(columns, rows)?;
    execute!(stdout(), MoveTo(0,1))?;

    println!("col : rows | {} : {}", columns, rows);

    match dirItems.iter().max_by_key(|s| s.len()) {
        Some(s) => {
            longest_item = String::from(s);
        }
        None => {
            longest_item = String::from("Directory is empty");
        }
    }
    //`longest_item.len() + 2` is due to displaying the cursor on the side of the screen
    let items_per_row: i32 = ((columns - (RIGHT_MARGIN+LEFT_MARGIN)) as f32 / (longest_item.len() + 2) as f32).floor() as i32;
    let rows_on_screen: i32 = ((rows as i32 - (TOP_MARGIN+BOTTOM_MARGIN) as i32) + 1) / 2;  

    // println!(" x {}, y {}", items_per_row, rows_on_screen);
    
    for x in 0..items_per_row {
        for y in 0..rows_on_screen {
            let (posx, posy) = get_display_grid_pos((x as u16, y as u16), longest_item.len() as u16)?;
            execute!(stdout(), MoveTo(posx, posy))?;
            print!(">{}<", "-".repeat(longest_item.len()));
            stdout().flush()?;
        }
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
            }, 
            _ => {}
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn get_display_grid_pos((gridX, gridY): (u16, u16), longest_item_len: u16) -> Result<(u16, u16), Box<dyn std::error::Error>>{
    let x_pos = RIGHT_MARGIN+((longest_item_len+2)*gridX);
    let y_pos = TOP_MARGIN+(gridY*2);
    return Ok((x_pos, y_pos))
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