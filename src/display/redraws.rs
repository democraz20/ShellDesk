use crossterm::{
    execute,
    cursor::MoveTo
};
use std::io::stdout;

pub fn print_borders(X: u16, Y: u16) -> Result<(), Box<dyn std::error::Error>> {
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