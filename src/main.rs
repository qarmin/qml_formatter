mod rules;
mod tests;

use crate::rules::*;

fn main() {
    convert_file("AB".to_string());
}

fn convert_file(file: String) -> String {
    let mut lines: Vec<_> = file.split('\n').map(|e| e.to_string()).collect();

    lines = remove_empty_space_on_end_of_line(lines);
    lines = skip_start_end_empty_lines(lines);

    // This functions are safe to think, that non empty lines starts and ends with non empty(" ", "\t") characters
    lines = connect_multiple_empty_lines_into_one(lines);
    lines = move_single_open_bracket(lines);
    lines = remove_useless_spaces_around_colon(lines);
    lines = remove_empty_line_before_close_bracket(lines);

    // Always at the end, before lines are guaranteed to start not with whitespace
    lines = move_elements_inside(lines);

    lines.join("\n")
}
