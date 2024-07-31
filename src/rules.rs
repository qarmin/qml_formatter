#![allow(clippy::needless_late_init)]
#![allow(clippy::collapsible_if)]

use crate::split_text_into_parts_to_read::{split_text_into_comment_part, UserTextOrNot};
use crate::{calculate_empty_spaces_at_start, check_for_multi_comment, check_for_single_line_comment, split_into_normal_and_comment_part};
use std::cmp::max;

pub fn remove_useless_spaces_around_colon(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }

        let new_line;
        if line.trim().starts_with("regularExpression") {
            new_lines.push(line);
            continue;
        }
        if line.contains(':') {
            let mut collected_chars: Vec<char> = Vec::new();

            let quote_char: Option<char> = None;

            for (text_type, part) in split_text_into_comment_part(line.trim_start(), quote_char) {
                if text_type == UserTextOrNot::QMLCode {
                    for charr in part.chars() {
                        if charr == ' ' {
                            if collected_chars.last() != Some(&' ') {
                                collected_chars.push(' ');
                            }
                        } else if charr == '?' {
                            if ![Some(&'?'), Some(&' ')].contains(&collected_chars.last()) {
                                collected_chars.push(' ');
                                collected_chars.push(charr);
                                collected_chars.push(' ');
                            } else if collected_chars.last() == Some(&'?') {
                                collected_chars.push(charr);
                                collected_chars.push(' ');
                            } else {
                                collected_chars.push(charr);
                            }
                        } else if charr == ':' {
                            if collected_chars.contains(&'?') {
                                if collected_chars.last() != Some(&' ') {
                                    collected_chars.push(' ');
                                    collected_chars.push(charr);
                                    collected_chars.push(' ');
                                } else {
                                    collected_chars.push(charr);
                                }
                            } else {
                                if collected_chars.last() == Some(&' ') {
                                    collected_chars.pop();
                                }
                                collected_chars.push(charr);
                            }
                        } else {
                            collected_chars.push(charr)
                        }
                    }
                } else {
                    collected_chars.append(&mut part.chars().collect::<Vec<_>>());
                }
            }

            new_line = collected_chars.into_iter().collect();
        } else {
            new_line = line;
        }

        new_lines.push(new_line);
    }
    new_lines
}

// Function
pub fn move_elements_inside(lines: Vec<String>) -> Vec<String> {
    let mut current_bracket_number = 0u32;
    let mut new_lines: Vec<String> = Vec::new();
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }

        if current_bracket_number == 0 {
            new_lines.push(line.clone());
        } else {
            let mut new_line = "".to_string();

            let mut closing_brackets = 0;
            {
                let mut line_trimmed = line.chars().collect::<Vec<_>>();
                while !line_trimmed.is_empty() {
                    if ['}', ')', ']'].contains(&line_trimmed[0]) {
                        line_trimmed.remove(0);
                        closing_brackets += 1;
                    } else {
                        break;
                    }
                }
            }

            for _ in 0..(current_bracket_number as i32 - closing_brackets) {
                new_line.push_str("    ") // 4 spaces is default for QML
            }
            new_line.push_str(&line);
            new_lines.push(new_line.trim_end().to_string()); // Trimming, because empty line with only spaces could be here
        }

        let mut count_open_round_bracket = 0;
        let mut count_open_bracket = 0;
        let mut count_open_square_bracket = 0;
        let mut count_close_round_bracket = 0;
        let mut count_close_bracket = 0;
        let mut count_close_square_bracket = 0;
        let mut string_interpolation = false;
        for charr in line.chars() {
            if charr == '`' {
                string_interpolation = !string_interpolation;
                continue;
            }
            else if string_interpolation {
                continue;
            }
            if charr == '{' {
                count_open_bracket += 1;
            } else if charr == '}' {
                count_close_bracket += 1;
            } else if charr == '(' {
                count_open_round_bracket += 1;
            } else if charr == ')' {
                count_close_round_bracket += 1;
            } else if charr == '[' {
                count_open_square_bracket += 1;
            } else if charr == ']' {
                count_close_square_bracket += 1;
            }
        }

        current_bracket_number += count_open_round_bracket + count_open_bracket + count_open_square_bracket;

        let calculated_ending_brackets = count_close_bracket + count_close_round_bracket + count_close_square_bracket;
        if calculated_ending_brackets != 0 {
            if current_bracket_number < calculated_ending_brackets as u32 {
                println!("Mismatched number of {{ brackets, probably QML is broken");
                current_bracket_number = 0
            } else {
                current_bracket_number -= calculated_ending_brackets;
            }
        }
        if string_interpolation {
            println!("Unclosed template literal, probably QML is broken");
        }
    }
    new_lines
}

pub fn remove_empty_space_on_end_of_line(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }

        new_lines.push(line.trim().to_string());
    }
    new_lines
}

pub fn skip_start_end_empty_lines(mut lines: Vec<String>) -> Vec<String> {
    while !lines.is_empty() && lines[0].is_empty() {
        lines.remove(0);
    }
    while !lines.is_empty() && lines[lines.len() - 1].is_empty() {
        lines.pop();
    }
    lines
}

pub fn move_single_open_bracket(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        if line == "{" {
            while let Some(last_line) = new_lines.last() {
                if last_line.is_empty() {
                    new_lines.pop();
                } else {
                    break;
                }
            }
            if let Some(last_line) = new_lines.last_mut() {
                last_line.push_str(" {");
            }
        } else {
            new_lines.push(line);
        }
    }

    new_lines
}

pub fn connect_multiple_empty_lines_into_one(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();
    let mut was_empty = false;
    for line in lines {
        if line.is_empty() {
            if was_empty {
                continue;
            }
            was_empty = true;
        } else {
            was_empty = false;
        }
        new_lines.push(line);
    }
    new_lines
}

pub fn remove_empty_line_before_close_bracket(lines: Vec<String>) -> Vec<String> {
    let mut new_lines: Vec<String> = Vec::new();
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        if line == "}" && new_lines.last() == Some(&"".to_string()) {
            new_lines.pop();
        }
        new_lines.push(line);
    }
    new_lines
}

pub fn if_movement(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut find_oneliner = false;
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        let mut new_line = "".to_string();
        if find_oneliner {
            new_line.push_str("    ");
            find_oneliner = false
        }

        new_line.push_str(&line);
        new_line = new_line.replace(" if(", " if (");
        new_line = new_line.replace(" else if(", " else if (");
        new_line = new_line.replace(" else{", " else {");
        new_line = new_line.replace(" for(", " for (");
        new_lines.push(new_line.clone());

        let line_trimmed = new_line.trim();
        if line_trimmed == "else" || line_trimmed.ends_with(')') && ["if (", "else if (", "for ("].iter().any(|e| line_trimmed.starts_with(e)) {
            {
                find_oneliner = true
            }
        }
    }
    new_lines
}

pub fn connect_end_lines(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut next_line_with_addition = false;
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        let (line, comments) = split_into_normal_and_comment_part(&line);
        let mut spaces = "".to_string();

        if next_line_with_addition {
            next_line_with_addition = false;
            spaces = "    ".to_string();
        }

        if ["+", "-", ":", "?"].iter().any(|e| line.ends_with(e)) {
            next_line_with_addition = true;
        }

        new_lines.push(format!("{}{}{}", spaces, line, comments));
    }
    new_lines
}

pub fn switch_case(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut case_started = false;
    let mut current_case_line = 0;
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        let mut new_line = "".to_string();

        if case_started {
            current_case_line += 1;
        }

        // First line is already handled by adding spaces when previous line ends with : (connect_end_lines)
        if current_case_line == 1 {
            new_lines.push(line);
            continue;
        }

        let line_trimmed = line.trim();
        if (line_trimmed.starts_with("case ") || line_trimmed.starts_with("default:")) && line_trimmed.ends_with(':') {
            case_started = true;
            current_case_line = 0
        } else {
            if case_started && line_trimmed != "}" {
                new_line.push_str("    ");
            } else {
                case_started = false;
            }
        }

        new_line.push_str(&line);
        new_lines.push(new_line);
    }
    new_lines
}

pub fn space_before_bracket(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let quote_char: Option<char> = None; // if some, then means that string started, allowed values '`"
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        if line.contains('{') {
            let mut new_line: Vec<char> = Vec::new();
            for (text_type, part) in split_text_into_comment_part(&line, quote_char) {
                if text_type == UserTextOrNot::QMLCode {
                    for charr in part.chars() {
                        if charr == '{' {
                            if new_line.last() != Some(&' ') && ![Some(&'['), Some(&'(')].contains(&new_line.last()) && new_line.last() != None {
                                new_line.push(' ');
                            } else if new_line.last() == Some(&' ') && [Some(&'['), Some(&'(')].contains(&new_line.get(new_line.len() - 2)) {
                                new_line.pop();
                            }
                        }
                        new_line.push(charr);
                    }
                } else {
                    new_line.append(&mut part.chars().collect::<Vec<_>>());
                }
            }
            new_lines.push(new_line.iter().collect())
        } else {
            new_lines.push(line);
        }
    }
    new_lines
}

pub fn reorganize_space_in_models(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    let mut model_bracket_start_position: Option<usize> = None;
    let mut multi_comment = false;
    for line in lines {
        if check_for_multi_comment(&line, &mut multi_comment) || check_for_single_line_comment(&line) {
            new_lines.push(line);
            continue;
        }
        let (mut line, comments) = split_into_normal_and_comment_part(&line);
        if !line.ends_with(']') {
            if let Some(model_start_position) = model_bracket_start_position {
                // If thing inside is self created QML object, do not use tabs
                if !line.ends_with(" {") {
                    let difference = max(model_start_position as i32 - calculate_empty_spaces_at_start(&line), 0);
                    let mut new_line = "".to_string();
                    for _ in 0..difference {
                        new_line.push(' ');
                    }
                    new_line.push_str(&line);
                    line = new_line;
                } else {
                    model_bracket_start_position = None;
                }
            } else {
                if let Some(start_index) = line.find(": [") {
                    for (index, charr) in line.chars().enumerate() {
                        if charr.is_ascii_alphanumeric() || charr == '_' || charr.is_ascii_whitespace() {
                            continue;
                        } else if charr == ':' {
                            if index == start_index {
                                model_bracket_start_position = Some(index + 3);
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
        } else {
            if let Some(model_start_position) = model_bracket_start_position {
                if !["", "]"].contains(&line.trim()) {
                    let difference = max(model_start_position as i32 - calculate_empty_spaces_at_start(&line), 0);
                    let mut new_line = "".to_string();
                    for _ in 0..difference {
                        new_line.push(' ');
                    }
                    new_line.push_str(&line);
                    line = new_line;
                }
                model_bracket_start_position = None;
            }
        }

        line.push_str(&comments);
        new_lines.push(line);
    }
    new_lines
}

#[allow(unused)]
pub fn split_text_to_vector(text: &str) -> Vec<String> {
    text.split('\n').map(str::to_string).collect()
}
