use std::fs::ReadDir;
use std::fs::DirEntry;

pub fn custom_sort(read_dir: ReadDir) -> Vec<String> {
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

pub fn is_within_range(from: i32, to: i32, num: i32) -> Option<bool> {
    if from == to || from > to {
        // Return None if from and to are the same or if from is greater than to
        None
    } else {
        // Check if num is within the range (excluding from and to)
        Some(num > from && num < to)
    }
}
