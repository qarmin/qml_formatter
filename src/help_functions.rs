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
