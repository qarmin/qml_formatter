extern crate core;

mod help_functions;
mod rules;
mod split_text_into_parts_to_read;
mod tests;

use crate::help_functions::*;
use crate::rules::*;

use std::fs::OpenOptions;
use std::io::Write;

use std::{fs, io, process};

fn main() {
    let (interactive_mode, included_directories, files_to_check) = collect_files_to_check();

    println!(
        "{} files from \"{}\" will be checked - {}",
        files_to_check.len(),
        included_directories.join(", "),
        files_to_check.join("\n")
    );
    if interactive_mode {
        loop {
            println!("Are you sure, that you want to convert this files? y/N");
            let mut ret = String::new();
            io::stdin().read_line(&mut ret).expect("Failed to read from stdin");
            ret = ret.trim().to_string();
            if ret == "y" || ret == "Y" {
                break;
            } else if ret == "N" || ret == "n" || ret.is_empty() {
                process::exit(0);
            }
        }
    }

    for file_to_check in files_to_check {
        match fs::read_to_string(&file_to_check) {
            Ok(input) => match OpenOptions::new().truncate(true).write(true).open(&file_to_check) {
                Ok(mut file_handler) => {
                    let output = convert_file(input);
                    write!(file_handler, "{}", output).unwrap();
                }
                Err(e) => {
                    eprintln!("Failed to write file {}, reason {}", file_to_check, e);
                }
            },
            Err(e) => {
                eprintln!("Failed to read file {}, reason {}", file_to_check, e);
            }
        }
    }
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
    lines = space_before_bracket(lines);

    // Always at the end, before lines are guaranteed to start not with whitespace
    lines = move_elements_inside(lines);

    // Small fixes to already converted project
    lines = if_movement(lines);

    lines.join("\n")
}
