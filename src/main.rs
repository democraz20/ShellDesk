#![allow(non_snake_case)]

use crossterm::event::KeyEventState;
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
use std::io::{stdout, Write};
use std::{error::Error, fs};

use crate::display::layout::{index_to_xy, xy_to_index};
use crate::file_management::preprocessing::is_within_range;

mod display;
mod file_management;
mod misc;
use std::fs::OpenOptions;

const TOP_MARGIN: u16 = 2;
const BOTTOM_MARGIN: u16 = 1;
const RIGHT_MARGIN: u16 = 1;
const LEFT_MARGIN: u16 = 1;

#[derive(Debug)]
struct Cursor {
    selecting: Selecting,
    current_line: usize,
    selected: Selected,
}

#[derive(Debug)]
struct DisplayInfo {
    top_display_line: usize
}

#[derive(Debug)]
struct Selecting {
    index: usize,
    x: usize,
    y: usize,
}

#[derive(Debug)]
struct Selected {
    items: Vec<String>,
    from: Option<usize>,
    to: Option<usize>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let ORIGINTESTPATH = "./../";

    init_logger!();

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

    match UdirItems.iter().max_by_key(|s| s.len()) {
        Some(s) => {
            longest_item = String::from(s);
        }
        None => {
            longest_item = String::from("Directory is empty");
        }
    }
    //`longest_item.len() + 2` is due to displaying the cursor on the side of the screen
    let items_per_row: usize = ((columns - (RIGHT_MARGIN + LEFT_MARGIN)) as f32
        / (longest_item.len() + 2) as f32)
        .floor() as usize;
    // let rows_on_screen: usize= (((rows as i32 - (TOP_MARGIN + BOTTOM_MARGIN) as i32) + 1) / 2) as usize;

    //for debug's sake
    let rows_on_screen: usize = 3;

    let dirItemsCount = UdirItems.len();

    let mut dirItems: Vec<Vec<String>> = vec![];

    for chunk in UdirItems.chunks(items_per_row) {
        let sub_vec: Vec<String> = chunk.to_vec();
        dirItems.push(sub_vec);
    }

    drop(UdirItems);

    execute!(stdout(), MoveTo(0, 0))?;
    // println!();
    for row in &dirItems {
        println!("{:?}", row);
    }

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
    let mut cursor = Cursor {
        selecting: Selecting {
            index: 0,
            x: 0,
            y: 0,
        },
        current_line: 0,
        selected: Selected {
            items: vec![],
            from: None,
            to: None,
        },
    };

    let mut displayinfo = DisplayInfo {
        top_display_line: 0
    };
    // log!("items count: {}", dirItems.len()*items_per_row);
    log!("items count: {}\n rowsonscreen: {}", dirItemsCount, rows_on_screen);

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

            //-----------multiselect-----------------
            Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                if cursor.selecting.index < dirItemsCount - 1 {
                    let (x, y) = index_to_xy(cursor.selecting.index, items_per_row);
                    let t = &dirItems[y][x];
                    if t == &String::from("..") {
                        cursor.selecting.index += 1;
                        (cursor.selecting.x, cursor.selecting.y) =
                            index_to_xy(cursor.selecting.index, items_per_row );

                        let diff = cursor.current_line - displayinfo.top_display_line;
                        if diff > rows_on_screen {
                            displayinfo.top_display_line += 1;
                        }

                        d_p(&displayinfo)?;
                        t_p(&cursor, &dirItems, &longest_item)?;
                        v_p(&cursor.selected)?;
                        
                        log!("finished key press");
                        continue;
                    }

                    //start selecting
                    if cursor.selected.items.len() == 0
                        && cursor.selecting.index != dirItemsCount - 1
                    {
                        cursor.selected.from = Some(cursor.selecting.index);
                        cursor.selected.to = Some(cursor.selecting.index + 1);

                        // cursor.selected.items = vec
                        // cursor.selected.items.push(value)
                        let (x, y) = index_to_xy(cursor.selecting.index, items_per_row );
                        let t = &dirItems[y][x];
                        cursor.selected.items.push(t.to_string());
                        let (x, y) =
                            index_to_xy(cursor.selecting.index + 1, items_per_row );
                        let t = &dirItems[y][x];
                        cursor.selected.items.push(t.to_string());

                        cursor.selecting.index += 1;

                        let diff = cursor.current_line - displayinfo.top_display_line;
                        if diff > rows_on_screen {
                            displayinfo.top_display_line += 1;
                        }

                        d_p(&displayinfo)?;
                        t_p(&cursor, &dirItems, &longest_item)?;
                        v_p(&cursor.selected)?;
                        log!("finished key press");
                        continue;
                    }

                    //after
                    //if something is selected
                    if cursor.selected.from != None && cursor.selected.to != None {
                        //`.unwrap()` is safe under here because already checked for `None`

                        let v = is_within_range(
                            cursor.selected.from.unwrap() as i32,
                            cursor.selected.to.unwrap() as i32,
                            cursor.selecting.index as i32,
                        );
                        if v != None && v == Some(true) {
                            //reset the cursor
                            cursor.selected.items.clear();
                            cursor.selected.from = None;
                            cursor.selected.to = None;

                            cursor.selected.from = Some(cursor.selecting.index);

                            let (x, y) =
                                index_to_xy(cursor.selecting.index, items_per_row );
                            let t = &dirItems[y][x];
                            cursor.selected.items.push(t.to_string());

                            cursor.selecting.index += 1;
                            (cursor.selecting.x, cursor.selecting.y) =
                                index_to_xy(cursor.selecting.index, items_per_row );

                            cursor.selected.to = Some(cursor.selecting.index);

                            let (x, y) =
                                index_to_xy(cursor.selecting.index, items_per_row );
                            let t = &dirItems[y][x];
                            cursor.selected.items.push(t.to_string());

                            // log!("pushed: {}, buf: {:?}", t, cursor.selected.items);
                        } else if Some(cursor.selecting.index) == cursor.selected.from {
                            if cursor.selected.from.unwrap() + 1 == cursor.selected.to.unwrap() {
                                //colapse select
                                cursor.selected.items.clear();
                                cursor.selected.from = None;
                                cursor.selected.to = None;
                                cursor.selecting.index += 1;
                            } else {
                                cursor.selecting.index += 1;
                                let v = cursor.selected.from.unwrap() + 1;
                                cursor.selected.from = Some(v);
                                cursor.selected.items.remove(0);
                            }
                        } else if Some(cursor.selecting.index) == cursor.selected.to {
                            // if cursor.selecting.index != cursor.selected.items.len() {
                            if cursor.selecting.index != dirItemsCount - 1 {
                                cursor.selecting.index += 1;
                                let v = cursor.selected.to.unwrap() + 1;
                                cursor.selected.to = Some(v);
                                let (x, y) =
                                    index_to_xy(cursor.selecting.index, items_per_row);
                                let t = &dirItems[y][x];
                                cursor.selected.items.push(t.to_string());
                            }
                        }
                        cursor.current_line = cursor.selecting.y;

                        let diff = cursor.current_line - displayinfo.top_display_line;
                        if diff > rows_on_screen {
                            displayinfo.top_display_line += 1;
                        }

                        d_p(&displayinfo)?;
                        t_p(&cursor, &dirItems, &longest_item)?;
                        v_p(&cursor.selected)?;
                        log!("finished key press");
                    }
                }
            }

            Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                if cursor.selecting.index - 1 >= 1 {
                    //cant be ".." because ".." is index 0
                    if cursor.selected.items.len() == 0 {
                        //nothing selected yet
                        cursor.selected.from = Some(cursor.selecting.index - 1);
                        cursor.selected.to = Some(cursor.selecting.index);

                        let (x, y) =
                            index_to_xy(cursor.selecting.index - 1, items_per_row );
                        let t = &dirItems[y][x];
                        cursor.selected.items.push(t.to_string());
                        let (x, y) = index_to_xy(cursor.selecting.index, items_per_row  );
                        let t = &dirItems[y][x];
                        cursor.selected.items.push(t.to_string());

                        cursor.selecting.index -= 1;

                        let diff = cursor.current_line - displayinfo.top_display_line;
                        if diff < rows_on_screen {
                            if displayinfo.top_display_line != 0 {
                                displayinfo.top_display_line -= 1;
                            }
                        }
                        
                        d_p(&displayinfo)?;
                        t_p(&cursor, &dirItems, &longest_item)?;
                        v_p(&cursor.selected)?;
                        log!("finished key press");
                        continue;
                    }

                    if cursor.selected.from != None && cursor.selected.to != None {
                        let v = is_within_range(
                            cursor.selected.from.unwrap() as i32,
                            cursor.selected.to.unwrap() as i32,
                            cursor.selecting.index as i32,
                        );
                        if v != None && v == Some(true) {
                            cursor.selected.items.clear();
                            cursor.selected.from = None;
                            cursor.selected.to = None;
                        } else if Some(cursor.selecting.index) == cursor.selected.from {
                            if cursor.selected.from.unwrap() - 1 != 0 {
                                cursor.selecting.index -= 1;
                                let v = cursor.selected.from.unwrap() - 1;
                                cursor.selected.from = Some(v);
                                let (x, y) =
                                    index_to_xy(cursor.selecting.index, items_per_row  );
                                let t = &dirItems[y][x];
                                cursor.selected.items.insert(0, t.to_string());
                            }
                        } else if Some(cursor.selecting.index) == cursor.selected.to
                            && cursor.selecting.index - 1 != 0
                        {
                            if cursor.selected.to.unwrap() - 1 == cursor.selected.from.unwrap() {
                                cursor.selected.items.clear();
                                cursor.selected.from = None;
                                cursor.selected.to = None;
                                cursor.selecting.index -= 1;
                            } else {
                                cursor.selecting.index -= 1;
                                let v = cursor.selected.to.unwrap() - 1;
                                cursor.selected.to = Some(v);
                                cursor.selected.items.pop();
                            }
                        }
                        cursor.current_line = cursor.selecting.y;

                        let diff = cursor.current_line - displayinfo.top_display_line;
                        if diff < rows_on_screen {
                            if displayinfo.top_display_line != 0 {
                                displayinfo.top_display_line -= 1;
                            }
                        }

                        d_p(&displayinfo)?;
                        t_p(&cursor, &dirItems, &longest_item)?;
                        v_p(&cursor.selected)?;
                        log!("finished key press");
                    }
                }
            }

            Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                if cursor.selecting.index < dirItemsCount - 1 { //not sure what this does yet
                    if cursor.selecting.index == 0 { //special case
                        //just normal select
                        if (cursor.selecting.index + items_per_row  ) > (dirItemsCount - 1) {
                            cursor.selecting.index = dirItemsCount - 1
                        } else {
                            cursor.selecting.index += items_per_row  ;
                        }
                        (cursor.selecting.x, cursor.selecting.y) =
                            index_to_xy(cursor.selecting.index, items_per_row  );

                        let diff = cursor.current_line - displayinfo.top_display_line;
                        if diff > rows_on_screen {
                            displayinfo.top_display_line += 1;
                        }

                        d_p(&displayinfo)?;
                        t_p(&cursor, &dirItems, &longest_item)?;
                        log!("finished key press");
                        continue;
                    } 
                    if cursor.selected.items.len() == 0 { //first time selecting
                        if cursor.selected.from != None && cursor.selected.to != None { //idk
                        } else {
                            if (cursor.selecting.index + items_per_row  )
                                > (dirItemsCount - 1)
                            { //if selecting will overflow
                                cursor.selected.from = Some(cursor.selecting.index);
                                cursor.selected.to = Some(dirItemsCount - 1);
                                for i in cursor.selected.from.unwrap()..cursor.selected.to.unwrap() {
                                    let (x, y) =
                                        index_to_xy(i, items_per_row );
                                    let t = &dirItems[y][x];
                                    cursor.selected.items.push(t.to_string());
                                }
                                cursor.selecting.index += dirItemsCount-1;
                            } else {
                                cursor.selected.from = Some(cursor.selecting.index);
                                cursor.selected.to = Some(cursor.selecting.index+items_per_row );
                                for i in cursor.selected.from.unwrap()..cursor.selected.to.unwrap()+1 {
                                    let (x, y) =
                                        index_to_xy(i, items_per_row );
                                    let t = &dirItems[y][x];
                                    cursor.selected.items.push(t.to_string());
                                }
                                cursor.selecting.index += items_per_row ;
                            }
                        }
                    } else {
                        //moving existing ends
                        if cursor.selected.from != None && cursor.selected.to != None {
                            if cursor.selecting.index == cursor.selected.to.unwrap() {
                                if cursor.selected.to.unwrap()+items_per_row > dirItemsCount-1{
                                    let old_to = cursor.selected.to.unwrap();
                                    cursor.selected.to = Some(dirItemsCount - 1);
                                    for i in old_to+1..dirItemsCount {
                                        let (x, y) = index_to_xy(i, items_per_row ); 
                                        let t = &dirItems[y][x];
                                        cursor.selected.items.push(t.to_string());
                                    }
                                    cursor.selecting.index = dirItemsCount-1;
                                }
                                else {
                                    let old_to = cursor.selected.to.unwrap();
                                    let mut v = cursor.selected.to.unwrap();
                                    v += items_per_row ;
                                    cursor.selected.to = Some(v);
                                    
                                    let diff_range = old_to+1..v+1; // +1 somehow?
                                    for i in diff_range {
                                        let (x, y) = index_to_xy(i, items_per_row ); 
                                        let t = &dirItems[y][x];
                                        cursor.selected.items.push(t.to_string());
                                    }
                                    cursor.selecting.index += items_per_row ;
                                }
                            }
                            else if cursor.selecting.index == cursor.selected.from.unwrap() {
                                log!("uhh");
                                // for now if overlapped, just collapse
                                // let diff = cursor.selected.to.unwrap() - cursor.selected.from.unwrap();
                                // log!("diff : {diff}");
                                // if cursor.selected.items.len() >= items_per_row  {
                                
                                if cursor.selected.items.len() <= items_per_row  ||
                                cursor.selected.from.unwrap()+items_per_row 
                                == cursor.selected.to.unwrap()
                                {
                                    //collapse
                                    log!("collapse");
                                    cursor.selecting.index = cursor.selected.to.unwrap();
                                    cursor.selected.items.clear();
                                    cursor.selected.from = None;
                                    cursor.selected.to = None;
                                }
                                // else if cursor.selected.to.unwrap()+items_per_row > dirItemsCount-1{
                                //     let old_to = cursor.selected.to.unwrap();
                                //     cursor.selected.to = Some(dirItemsCount - 1);
                                //     for i in old_to..dirItemsCount-1 {
                                //         let (x, y) = index_to_xy(i, items_per_row ); 
                                //         let t = &dirItems[y][x];
                                //         cursor.selected.items.push(t.to_string());
                                //     }
                                //     cursor.selecting.index = dirItemsCount-1;
                                // }
                                else {
                                    // let old_from = cursor.selected.from.unwrap();
                                    let mut v = cursor.selected.from.unwrap();
                                    v += items_per_row ;
                                    cursor.selected.from = Some(v);

                                    for _ in 0..items_per_row {
                                        cursor.selected.items.remove(0);
                                    }
                                    cursor.selecting.index += items_per_row ;
                                }
                            }
                        }
                    }
                    cursor.current_line = cursor.selecting.y;

                    let diff = cursor.current_line - displayinfo.top_display_line;
                    if diff > rows_on_screen {
                        displayinfo.top_display_line += 1;
                    }

                    d_p(&displayinfo)?;
                    t_p(&cursor, &dirItems, &longest_item)?;
                    v_p(&cursor.selected)?; 
                    log!("finished key press");
                }
            }
            Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                modifiers: KeyModifiers::SHIFT,
                ..
            }) => {
                // if (cursor.selecting.index as i32-items_per_row) <= 1 {
                    // log!("pressed up");
                    // if cursor.selecting.index == 0 { //special case
                    //     //nothing happens
                    // }
                    if cursor.selected.items.len() == 0 { //start selecting
                        if (cursor.selecting.index-items_per_row) <= 1 {
                            cursor.selected.from = Some(1);
                            cursor.selected.to = Some(cursor.selecting.index);
                            // for i in cursor.selected.from.unwrap()..cursor.selected.to.unwrap() {
                            for i in cursor.selected.from.unwrap()..cursor.selected.to.unwrap()+1 { //??????? it worked
                                let (x, y) =
                                    index_to_xy(i, items_per_row );
                                let t = &dirItems[y][x];
                                cursor.selected.items.push(t.to_string());
                            }
                            cursor.selecting.index = 1;
                        } else {
                            cursor.selected.from = Some(cursor.selecting.index - items_per_row );
                            cursor.selected.to = Some(cursor.selecting.index);
                            for i in cursor.selected.from.unwrap()..cursor.selected.to.unwrap()+1 {
                                let (x, y) =
                                    index_to_xy(i, items_per_row );
                                let t = &dirItems[y][x];
                                cursor.selected.items.push(t.to_string());
                            }
                            cursor.selecting.index -= items_per_row ;
                        }
                    } else { //moving existing ends
                        if cursor.selected.from != None && cursor.selected.to != None {
                            if cursor.selecting.index == cursor.selected.to.unwrap() {
                                if cursor.selected.items.len() <= items_per_row ||
                                cursor.selected.to.unwrap()-items_per_row == 
                                cursor.selected.from.unwrap(){
                                    //overlapped -> collapse (for now)
                                    cursor.selecting.index = cursor.selected.from.unwrap();
                                    cursor.selected.items.clear();
                                    cursor.selected.from = None;
                                    cursor.selected.to = None;
                                } else {
                                    let v = cursor.selected.to.unwrap()-items_per_row ;
                                    cursor.selected.to = Some(v);
                                    for _ in 0..items_per_row {
                                        cursor.selected.items.remove(
                                            cursor.selected.items.len()-1
                                        );
                                    }
                                    cursor.selecting.index -= items_per_row 
                                }
                            }
                            else if cursor.selecting.index==cursor.selected.from.unwrap() {
                                if cursor.selected.from.unwrap()-items_per_row == 0 {
                                    let old_from = cursor.selected.from.unwrap();
                                    cursor.selected.from = Some(
                                        cursor.selected.from.unwrap()-(items_per_row-1)
                                    );
                                    log!("{}..{}", cursor.selected.from.unwrap(), old_from);
                                    // for i in cursor.selected.from.unwrap()..old_from {
                                    for i in (cursor.selected.from.unwrap()..old_from).rev() {
                                    // for i in old_from..cursor.selected.from.unwrap() {
                                        let (x, y) =
                                        index_to_xy(i, items_per_row );
                                        let t = &dirItems[y][x];
                                        // cursor.selected.items.push(t.to_string());
                                        cursor.selected.items.insert(0, t.to_string());
                                    }
                                } else {
                                    let old_from = cursor.selected.from.unwrap();
                                    cursor.selected.from = Some(
                                        cursor.selected.from.unwrap()-items_per_row
                                    );
                                    log!("{}..{}", cursor.selected.from.unwrap(), old_from);
                                    // for i in cursor.selected.from.unwrap()..old_from {
                                    for i in (cursor.selected.from.unwrap()..old_from).rev() {
                                    // for i in old_from..cursor.selected.from.unwrap() {
                                        let (x, y) =
                                        index_to_xy(i, items_per_row );
                                        let t = &dirItems[y][x];
                                        // cursor.selected.items.push(t.to_string());
                                        cursor.selected.items.insert(0, t.to_string());
                                    }
                                }
                            }
                        }
                    }
                    cursor.current_line = cursor.selecting.y;

                    let diff = cursor.current_line - displayinfo.top_display_line;
                    if diff < rows_on_screen {
                        if displayinfo.top_display_line != 0 {
                            displayinfo.top_display_line -= 1;
                        }
                    }

                    d_p(&displayinfo)?;
                    t_p(&cursor, &dirItems, &longest_item)?;
                    v_p(&cursor.selected)?; 
                    log!("finished key press");
                // }
            }

            //------------normal select----------------
            Key(KeyEvent {
                code: KeyCode::Right,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if cursor.selecting.index < dirItemsCount - 1 {
                    cursor.selecting.index += 1;
                    (cursor.selecting.x, cursor.selecting.y) =
                        index_to_xy(cursor.selecting.index, items_per_row );
                    
                    cursor.current_line = cursor.selecting.y;
                    let diff = cursor.current_line - displayinfo.top_display_line;
                    if diff > rows_on_screen {
                        displayinfo.top_display_line += 1;
                    }
                    d_p(&displayinfo)?;
                    t_p(&cursor, &dirItems, &longest_item)?;
                    //top row check
                    // log!(
                    //     "diff = currentline-topdisplay\n{} = {} - {}, increment: {:?}",
                    //     diff, cursor.current_line, displayinfo.top_display_line,
                    //     diff>rows_on_screen
                    // );
                    //
                    log!("finished key press");
                }
            }
            Key(KeyEvent {
                code: KeyCode::Left,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if cursor.selecting.index > 0 {
                    cursor.selecting.index -= 1;
                    (cursor.selecting.x, cursor.selecting.y) =
                        index_to_xy(cursor.selecting.index, items_per_row );
                    
                    cursor.current_line = cursor.selecting.y;

                    let diff = cursor.current_line - displayinfo.top_display_line;
                    if diff < rows_on_screen {
                        if displayinfo.top_display_line != 0 {
                            displayinfo.top_display_line -= 1;
                        }
                    }

                    d_p(&displayinfo)?;
                    t_p(&cursor, &dirItems, &longest_item)?;
                    log!("finished key press");
                }
            }
            Key(KeyEvent {
                code: KeyCode::Down,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if cursor.selecting.index < dirItemsCount - 1 {
                    if (cursor.selecting.index + items_per_row ) > (dirItemsCount - 1) {
                        cursor.selecting.index = dirItemsCount - 1
                    } else {
                        cursor.selecting.index += items_per_row ;
                    }
                    (cursor.selecting.x, cursor.selecting.y) =
                        index_to_xy(cursor.selecting.index, items_per_row );

                    cursor.current_line = cursor.selecting.y;
                    let diff = cursor.current_line - displayinfo.top_display_line;
                    if diff > rows_on_screen {
                        displayinfo.top_display_line += 1;
                    }
                    d_p(&displayinfo)?;
                    t_p(&cursor, &dirItems, &longest_item)?;
                    log!("finished key press");
                }
            }
            Key(KeyEvent {
                code: KeyCode::Up,
                kind: KeyEventKind::Press,
                ..
            }) => {
                if cursor.selecting.index > 0 {
                    // #[allow(unused_comparisons)]
                    if (cursor.selecting.index as isize - items_per_row as isize) < 0 {
                        cursor.selecting.index = 0;
                    } else {
                        cursor.selecting.index -= items_per_row ;
                    }
                    (cursor.selecting.x, cursor.selecting.y) =
                        index_to_xy(cursor.selecting.index, items_per_row );
                        
                    cursor.current_line = cursor.selecting.y;

                    

                    d_p(&displayinfo)?;
                    t_p(&cursor, &dirItems, &longest_item)?;
                    log!("finished key press");
                }
            }
            _ => {}
        }
    }

    execute!(stdout(), LeaveAlternateScreen)?;
    disable_raw_mode()?;

    Ok(())
}

fn t_p(
    cursor: &Cursor,
    dirItems: &Vec<Vec<String>>,
    longest_item: &String,
) -> Result<(), Box<dyn Error>> {
    // execute!(stdout(), MoveTo(0,0))?;
    // println!(
    //     "index: {} at ({},{}), selecting: {}{}",
    //     cursor.selecting.index,
    //     cursor.selecting.x,
    //     cursor.selecting.y,
    //     dirItems[cursor.selecting.y][cursor.selecting.x],
    //     "-".repeat(longest_item.len() - dirItems[cursor.selecting.y][cursor.selecting.x].len())
    // );
    // stdout().flush()?;
    log!(
        "T | index: {} at ({},{}), selecting: {}, current line: {}",
        cursor.selecting.index,
        cursor.selecting.x,
        cursor.selecting.y,
        dirItems[cursor.selecting.y][cursor.selecting.x],
        cursor.current_line
    );
    Ok(())
}
fn v_p(selected: &Selected) -> Result<(), Box<dyn Error>> {
    // execute!(stdout(), MoveTo(0, 1))?;
    // //lol what the fuck is even this
    // println!(
    //     "selecting buffer: {}",
    //     {
    //         let mut s = String::new();
    //         for i in &selected.items {
    //             s.push_str(
    //                 &format!(
    //                     "\"{}\", ",
    //                     i
    //                 )
    //             );
    //         }
    //         s
    //     }
    // );
    // println!("from: {:?}, to: {:?}", selected.from, selected.to);
    log!("V | selecting buffer: [{}]", {
        let mut s = String::new();
        for i in &selected.items {
            s.push_str(&format!("\"{}\", ", i));
        }
        s
    });
    log!("from: {:?}, to: {:?}", selected.from, selected.to);
    Ok(())
}
fn d_p(displayinfo: &DisplayInfo) -> Result<(), Box<dyn Error>> {
    log!("D | topdline: {}",
        displayinfo.top_display_line,
    );
    Ok(())
}
// fn i_p(cursor: &Cursor) -> Result<(), Box<dyn Error>> {
//     log!("items count: {}", cursor.selected.items.len());
//     Ok(())
// }
