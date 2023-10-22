#![allow(non_snake_case)]

use std::fs;
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

use crate::display::layout::index_to_xy;

mod display;
mod file_management;

const TOP_MARGIN: u16 = 2;
const BOTTOM_MARGIN: u16 = 1;
const RIGHT_MARGIN: u16 = 1;
const LEFT_MARGIN: u16 = 1;

struct Cursor {
    selected: Selected,
    current_line: usize
}

struct Selected {
    index: usize,
    x: usize,
    y: usize
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ORIGINTESTPATH = "./../";
    //desktop
    let UdirItems = fs::read_dir(ORIGINTESTPATH)?;
    // let mut UdirItems = filecustom_sort(UdirItems);
    let mut UdirItems = file_management::preprocessing::custom_sort(UdirItems);
    UdirItems.insert(0, String::from(".."));

    let longest_item: String;

    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;

    
    let (cursorX, cursorY) = cursor::position()?;
    let (columns, rows) = terminal::size()?;
    
    // print_borders(columns, rows)?;
    // execute!(stdout(), MoveTo(0,1))?;

    // println!("col : rows | {} : {}", columns, rows);

    match UdirItems.iter().max_by_key(|s| s.len()) {
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

    let dirItemsCount = UdirItems.len();

    let mut dirItems: Vec<Vec<String>> = vec![];

    for chunk in UdirItems.chunks(items_per_row as usize) {
        let sub_vec: Vec<String> = chunk.to_vec();
        dirItems.push(sub_vec);
    }

    drop(UdirItems);

    println!();
    for row in &dirItems {
        println!("{:?}", row);
    }

    // println!(" x {}, y {}", items_per_row, rows_on_screen);
    
    //IMPORTANT
    // for x in 0..items_per_row {
    //     for y in 0..rows_on_screen {
    //         let (posx, posy) = get_display_grid_pos((x as u16, y as u16), longest_item.len() as u16)?;
    //         execute!(stdout(), MoveTo(posx, posy))?;
    //         print!(">{}<", "-".repeat(longest_item.len()));
    //         stdout().flush()?;
    //     }
    // }

    // let mut selected_index = 0;
    // let (mut selected_index,mut selected_x,mut selected_y) = (0,0,0);
    let mut cursor = Cursor{
        selected: Selected { index: 0, x: 0, y: 0 },
        current_line: 0
    };

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
                if cursor.selected.index < dirItemsCount {
                    cursor.selected.index += 1;
                    (cursor.selected.x, cursor.selected.y) = index_to_xy(cursor.selected.index, items_per_row as usize);
                    execute!(stdout(), MoveTo(0,0))?;
                    println!(
                        "index: {} at ({},{}), selected: {}{}", 
                        cursor.selected.index, 
                        cursor.selected.x,
                        cursor.selected.y,
                        dirItems[cursor.selected.y][cursor.selected.x], 
                        "-".repeat(longest_item.len() - dirItems[cursor.selected.y][cursor.selected.x].len())
                    );
                }
            },
            Key(KeyEvent{
                code: KeyCode::Left,
                kind: KeyEventKind::Press, ..
            }) => {
                if cursor.selected.index > 0 {
                    cursor.selected.index -= 1;
                    (cursor.selected.x, cursor.selected.y) = index_to_xy(cursor.selected.index, items_per_row as usize);
                    execute!(stdout(), MoveTo(0,0))?;
                    println!(
                        "index: {} at ({},{}), selected: {}{}", 
                        cursor.selected.index, 
                        cursor.selected.x,
                        cursor.selected.y,
                        dirItems[cursor.selected.y][cursor.selected.x], 
                        "-".repeat(longest_item.len() - dirItems[cursor.selected.y][cursor.selected.x].len())
                    );
                }
            },
            Key(KeyEvent{
                code: KeyCode::Down,
                kind: KeyEventKind::Press, ..
            }) => {
                if cursor.selected.index < dirItemsCount {
                    if (cursor.selected.index+items_per_row as usize) > (dirItemsCount-1) {
                        cursor.selected.index = dirItemsCount-1
                    }
                    else {
                        cursor.selected.index += items_per_row as usize;
                    }
                    (cursor.selected.x, cursor.selected.y) = index_to_xy(cursor.selected.index, items_per_row as usize);
                    execute!(stdout(), MoveTo(0,0))?;
                    println!(
                        "index: {} at ({},{}), selected: {}{}", 
                        cursor.selected.index, 
                        cursor.selected.x,
                        cursor.selected.y,
                        dirItems[cursor.selected.y][cursor.selected.x], 
                        "-".repeat(longest_item.len() - dirItems[cursor.selected.y][cursor.selected.x].len())
                    );
                }
            },
            Key(KeyEvent{
                code: KeyCode::Up,
                kind: KeyEventKind::Press, ..
            }) => {
                if cursor.selected.index > 0 {
                    // #[allow(unused_comparisons)]
                    if (cursor.selected.index as i32-items_per_row) < 0 {
                        cursor.selected.index = 0;
                    }
                    else {
                        cursor.selected.index -= items_per_row as usize;
                    }
                    (cursor.selected.x, cursor.selected.y) = index_to_xy(cursor.selected.index, items_per_row as usize);
                    execute!(stdout(), MoveTo(0,0))?;
                    println!(
                        "index: {} at ({},{}), selected: {}{}", 
                        cursor.selected.index, 
                        cursor.selected.x,
                        cursor.selected.y,
                        dirItems[cursor.selected.y][cursor.selected.x], 
                        "-".repeat(longest_item.len() - dirItems[cursor.selected.y][cursor.selected.x].len())
                    );
                }
            },
            _ => {}
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}