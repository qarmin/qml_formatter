use std::path::Path;
use std::{env, process};
use walkdir::WalkDir;

pub fn collect_files_to_check() -> (bool, Vec<String>, Vec<String>) {
    let arguments = env::args();
    let mut interactive_mode = true;

    let mut included_directories = Vec::new();
    let mut excluded_directories = Vec::new();
    for argument in arguments.into_iter().skip(1) {
        if argument == "NO_QUESTION" {
            interactive_mode = false
        } else if let Some(directory) = argument.strip_prefix("-e") {
            if Path::new(directory).exists() {
                excluded_directories.push(directory.to_string())
            } else {
                eprintln!("Path {} not exists!", directory);
            }
        } else if Path::new(&argument).exists() {
            included_directories.push(argument)
        } else {
            eprintln!("Path {} not exists!", argument);
        }
    }
    if included_directories.is_empty() {
        eprintln!("You must provide at least 1 directory to check");
        process::exit(1);
    }

    let mut collected_files = Vec::new();
    for dir in included_directories.clone() {
        for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
            let path: &Path = entry.path();
            let path_canon = path.canonicalize().unwrap();
            if !path.is_file() {
                continue;
            }
            let path_str = path.to_string_lossy().to_string();
            if !path_str.ends_with(".qml") {
                continue;
            }

            if excluded_directories.iter().any(|f| path_canon.to_string_lossy().to_string().starts_with(f)) {
                continue; // Excluded item
            };

            collected_files.push(path_str);
        }
    }

    collected_files.sort_unstable();
    included_directories.sort_unstable();
    (interactive_mode, included_directories, collected_files)
}

pub fn calculate_empty_spaces_at_start(line: &str) -> i32 {
    let mut counter = 0;
    for charr in line.chars() {
        if charr == ' ' {
            counter += 1;
        } else {
            break;
        }
    }
    return counter;
}

pub fn split_into_normal_and_comment_part(line: &str) -> (String, String) {
    let mut separator: Option<char> = None;
    let mut normal_text = "".to_string();
    let mut comment_part = "".to_string();
    if !line.contains("//") {
        normal_text = line.to_string();
    } else {
        let characters: Vec<_> = line.chars().collect();
        let mut found_comment = false;
        for (index, charr) in characters.iter().enumerate() {
            match charr {
                '\'' | '"' | '`' => {
                    if let Some(sep_char) = separator {
                        if sep_char == *charr {
                            separator = None;
                        }
                    } else {
                        separator = Some(*charr);
                    }
                }
                '/' => {
                    if separator.is_none() && characters.get(index + 1) == Some(&'/') {
                        let ind = if index > 0 && characters.get(index - 1) == Some(&' ') { index - 1 } else { index };
                        for (i, character) in characters.iter().enumerate() {
                            if i < ind {
                                normal_text.push(*character);
                            } else {
                                comment_part.push(*character);
                            }
                        }
                        found_comment = true;
                        break;
                    }
                }
                _ => {}
            }
        }
        if !found_comment {
            normal_text = line.to_string();
        }
    }

    return (normal_text, comment_part);
}

pub fn check_for_multi_comment(line: &str, is_comment: &mut bool) -> bool {
    let line_trimmed = line.trim();
    if line_trimmed.starts_with("/*") {
        *is_comment = true;
        true
    } else if line_trimmed.ends_with("*/") {
        *is_comment = false;
        true
    } else {
        *is_comment
    }
}

#[test]
pub fn test_check_for_multi_comment() {
    let input_line = r##"input_text"##;
    let mut is_commented = false;
    let current_line_is_commented = check_for_multi_comment(input_line, &mut is_commented);
    assert_eq!(current_line_is_commented, false);
    assert_eq!(is_commented, false);

    let input_line = r##"input_text"##;
    let mut is_commented = true;
    let current_line_is_commented = check_for_multi_comment(input_line, &mut is_commented);
    assert_eq!(current_line_is_commented, true);
    assert_eq!(is_commented, true);

    let input_line = r##"/*input_text"##;
    let mut is_commented = false;
    let current_line_is_commented = check_for_multi_comment(input_line, &mut is_commented);
    assert_eq!(current_line_is_commented, true);
    assert_eq!(is_commented, true);

    let input_line = r##"input_text*/"##;
    let mut is_commented = false;
    let current_line_is_commented = check_for_multi_comment(input_line, &mut is_commented);
    assert_eq!(current_line_is_commented, true);
    assert_eq!(is_commented, false);
}

#[test]
pub fn test_calculate_empty_spaces_at_start() {
    let input = r##"input_text"##;
    let output = 0;
    assert_eq!(output, calculate_empty_spaces_at_start(input));

    let input = r##"    input_text"##;
    let output = 4;
    assert_eq!(output, calculate_empty_spaces_at_start(input));
}

#[test]
pub fn test_split_into_normal_and_comment_part() {
    let input = r##"input_text"##;
    let output = (r##"input_text"##.to_string(), r##""##.to_string());
    assert_eq!(output, split_into_normal_and_comment_part(input));

    let input = r##"input_text // ABC"##;
    let output = (r##"input_text"##.to_string(), r##" // ABC"##.to_string());
    assert_eq!(output, split_into_normal_and_comment_part(input));

    let input = r##""input_text//" // ABC"##;
    let output = (r##""input_text//""##.to_string(), r##" // ABC"##.to_string());
    assert_eq!(output, split_into_normal_and_comment_part(input));

    let input = r##"image: "qrc://image.svg""##;
    let output = (r##"image: "qrc://image.svg""##.to_string(), r##""##.to_string());
    assert_eq!(output, split_into_normal_and_comment_part(input));
}
