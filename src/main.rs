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

fn remove_useless_spaces_around_colon(lines: Vec<String>) -> Vec<String> {
    let mut new_lines = Vec::new();
    for line in lines {
        let new_line;
        if line.contains(':') {
            let chars_to_check: Vec<char> = line.trim_start().chars().collect();
            let mut collected_chars: Vec<char> = Vec::new();

            let mut quote_is_started = false;
            for item in chars_to_check {
                if item == ' ' {
                    if quote_is_started || collected_chars.last() != Some(&' ') {
                        collected_chars.push(' ');
                    }
                } else if item == ':' {
                    if !quote_is_started && collected_chars.last() == Some(&' ') {
                        collected_chars.pop();
                    }
                    collected_chars.push(item);
                } else if item == '"' {
                    // TODO add support for ', because is probably supported
                    if collected_chars.last() != Some(&'\\') {
                        quote_is_started = !quote_is_started;
                    }
                    collected_chars.push(item);
                } else {
                    collected_chars.push(item)
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
fn move_elements_inside(lines: Vec<String>) -> Vec<String> {
    let mut current_bracket_number = 0u32;
    let mut new_lines: Vec<String> = Vec::new();
    for line in lines {
        if current_bracket_number == 0 {
            new_lines.push(line.clone());
        } else {
            let mut new_line = "".to_string();
            let spaces_to_add = if line.trim() == "}" { current_bracket_number - 1 } else { current_bracket_number };
            for _ in 0..spaces_to_add {
                new_line.push_str("    ") // 4 spaces is default for QML
            }
            new_line.push_str(&line);
            new_lines.push(new_line.trim_end().to_string()); // Trimming, because empty line with only spaces could be here
        }

        if line.contains('{') {
            current_bracket_number += 1;
        }
        if line.contains('}') {
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

fn move_single_open_bracket(lines: Vec<String>) -> Vec<String> {
    let mut collected_lines: Vec<String> = Vec::new();
    for line in lines {
        if line == "{" {
            if let Some(last_line) = collected_lines.last_mut() {
                last_line.push_str(" {");
            }
        } else {
            collected_lines.push(line);
        }
    }

    collected_lines
}

fn connect_multiple_empty_lines_into_one(lines: Vec<String>) -> Vec<String> {
    let mut collected_lines: Vec<String> = Vec::new();
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
        collected_lines.push(line);
    }
    collected_lines
}

fn remove_empty_line_before_close_bracket(lines: Vec<String>) -> Vec<String> {
    let mut collected_lines: Vec<String> = Vec::new();
    for line in lines {
        if line == "}" && collected_lines.last() == Some(&"".to_string()) {
            collected_lines.pop();
        }
        collected_lines.push(line);
    }
    collected_lines
}

#[allow(unused)]
fn split_text_to_vector(text: &str) -> Vec<String> {
    text.split('\n').map(str::to_string).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_remove_empty_line_before_close_bracket() {
        let input = r#"Text {
text: "TODO"

}"#;
        let expected_output = r#"Text {
text: "TODO"
}"#;
        assert_eq!(remove_empty_line_before_close_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_connect_multiple_empty_lines_into_one() {
        let input = r#"
Text {}






Text {}
"#;
        let expected_output = r#"
Text {}

Text {}
"#;
        assert_eq!(connect_multiple_empty_lines_into_one(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_move_single_open_bracket() {
        let input = r#"Text
{
}"#;
        let expected_output = r#"Text {
}"#;
        assert_eq!(move_single_open_bracket(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_remove_useless_spaces_around_colon() {
        let input = r#"property var roman   :    ABCD"#;
        let expected_output = r#"property var roman: ABCD"#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"property var roman   :    "ABCD    :    ABCD""#;
        let expected_output = r#"property var roman: "ABCD    :    ABCD""#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));

        let input = r#"text: "ABCD \" \" \", \":   \" \" \" \" \" \"""#;
        let expected_output = r#"text: "ABCD \" \" \", \":   \" \" \" \" \" \"""#;
        assert_eq!(remove_useless_spaces_around_colon(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_move_elements_inside() {
        let input = r#"
Text {
Label {
Text {}
}
}
"#;
        let expected_output = r#"
Text {
    Label {
        Text {}
    }
}
"#;
        assert_eq!(move_elements_inside(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_remove_empty_space_on_end_of_line() {
        let input = r#"
        
Text {}         
                 
"#;
        let expected_output = r#"

Text {}

"#;
        assert_eq!(remove_empty_space_on_end_of_line(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

    #[test]
    fn test_skip_start_end_empty_lines() {
        let input = r#"

Text {}

"#;
        let expected_output = r#"Text {}"#;
        assert_eq!(skip_start_end_empty_lines(split_text_to_vector(input)), split_text_to_vector(expected_output));
    }

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
            "8_space_after_colon_in_string_with_slash",
            "9_single_open_bracket",
            "10_multiple_empty_lines",
            "11_empty_line_before_ending",
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
