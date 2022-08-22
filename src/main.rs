fn main() {
    convert_file("AB".to_string());
}

fn convert_file(file: String) -> String {
    let mut lines: Vec<_> = file.split("\n").map(|e| e.to_string()).collect();

    lines = remove_empty_space_on_end_of_line(lines);
    lines = skip_start_end_empty_lines(lines);
    lines = move_elements_inside(lines);
    lines = remove_useless_spaces_around_colon(lines);

    lines.join("\n")
}

fn remove_useless_spaces_around_colon(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    for line in lines {
        let mut new_line;
        if line.contains(":") {
            new_line = "".to_string();
            'start_spaces_loop: loop {
                for char in line.chars() {
                    if char == ' ' {
                        new_line.push(' ')
                    } else {
                        break 'start_spaces_loop;
                    }
                }
            }

            let mut quote_is_started: bool = false;
            let mut previous_character_is_slash = false;
            for char in line.chars() {
                if char == ' ' {
                    previous_character_is_slash = false;
                    if quote_is_started {
                        new_line.push(' ')
                    }
                } else if char == ':' {
                    previous_character_is_slash = false;
                    new_line.push(char);
                    new_line.push(' ');
                } else if char == '"' {
                    if !previous_character_is_slash {
                        quote_is_started = !quote_is_started;
                    }
                    previous_character_is_slash = false;
                    new_line.push(char);
                } else if char == '\\' {
                    previous_character_is_slash = true;
                    new_line.push(char);
                } else {
                    previous_character_is_slash = false;
                    new_line.push(char);
                }
            }
        } else {
            new_line = line;
        }

        new_lines.push(new_line);
    }
    new_lines
}

// Function
fn move_elements_inside(lines: Vec<String>) -> Vec<String> {
    let mut current_bracket_number = 0u32;
    let mut new_lines: Vec<String> = Vec::new();
    for line in lines {
        if current_bracket_number == 0 {
            new_lines.push(line.clone());
        } else {
            let mut new_line = "".to_string();
            let spaces_to_add = if line.trim() == "}" {
                current_bracket_number - 1
            } else {
                current_bracket_number
            };
            for _ in 0..spaces_to_add {
                new_line.push_str("    ") // 4 spaces is default for QML
            }
            new_line.push_str(&line);
            new_lines.push(new_line.trim_end().to_string()); // Trimming, because empty line with only spaces could be here
        }

        if line.contains("{") {
            current_bracket_number += 1;
        }
        if line.contains("}") {
            if current_bracket_number == 0 {
                println!("Mismatched number of {{ brackets, probably QML is broken");
            } else {
                current_bracket_number -= 1;
            }
        }
    }
    new_lines
}

fn remove_empty_space_on_end_of_line(lines: Vec<String>) -> Vec<String> {
    lines.into_iter().map(|e| e.trim().to_string()).collect()
}

fn skip_start_end_empty_lines(mut lines: Vec<String>) -> Vec<String> {
    while !lines.is_empty() && lines[0].is_empty() {
        lines.remove(0);
    }
    while !lines.is_empty() && lines[lines.len() - 1].is_empty() {
        lines.pop();
    }
    lines
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_conversion() {
        let test_dir = "src/test_data/";
        let tests = [
            "1_empty_space_at_the_end",
            "2_empty_lines_at_start_end",
            "3_mismatched_space",
            "4_not_mismatched_space",
            "5_not_mismatched_space_inside",
            "6_space_after_colon",
            "7_space_after_colon_in_string",
            "8_space_after_colon_in_string_with_c",
        ];
        for test_name in tests {
            println!("Testing {}", test_name);
            let input_name = format!("{}{}_INPUT.qml", test_dir, test_name);
            let output_name = format!("{}{}_OUTPUT.qml", test_dir, test_name);

            let input_content = fs::read_to_string(input_name).unwrap();
            let output_content = fs::read_to_string(output_name).unwrap();

            assert_eq!(convert_file(input_content), output_content);
        }
    }
}
