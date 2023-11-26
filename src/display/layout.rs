

pub fn get_display_grid_pos((gridX, gridY): (u16, u16), longest_item_len: u16, (right_margin, top_margin): (u16, u16))
-> Result<(u16, u16), Box<dyn std::error::Error>>{
    let x_pos = right_margin+((longest_item_len+2)*gridX);
    let y_pos = top_margin+(gridY*2);
    return Ok((x_pos, y_pos))
}

pub fn index_to_xy(cursor: usize, width: usize) -> (usize, usize) {
    let x = cursor % width;      // Calculate the column (x-coordinate)
    let y = cursor / width;      // Calculate the row (y-coordinate)
    (x, y)
}

pub fn xy_to_index(x: usize, y: usize, width: usize) -> usize {
    let index = y * width + x;
    index
}