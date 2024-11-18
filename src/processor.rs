use chrono::{Datelike, Local};
use clap::ArgMatches;
use rayon::prelude::*;
use std::collections::HashMap;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

use crate::fileutils;

/// Main logic to process files
pub fn run(matches: ArgMatches) {
    let source_dir = Path::new(matches.get_one::<String>("source").unwrap());
    let target_dir = Path::new(matches.get_one::<String>("target").unwrap());
    let verbose = matches.contains_id("verbose");
    let include_date = matches.contains_id("dated");

    let batch_size: usize = matches
        .get_one::<String>("batch")
        .map(|s| {
            s.parse().unwrap_or_else(|_| {
                eprintln!("Error: Invalid batch size specified.");
                std::process::exit(1);
            })
        })
        .unwrap_or(1);

    // Ensure the target directory exists
    ensure_directory(target_dir, matches.contains_id("force"));

    let file_size_map = Arc::new(Mutex::new(HashMap::new())); // Use Arc + Mutex for thread-safe sharing
    let source_files = fileutils::collect_files(source_dir);

    source_files
        .chunks(batch_size)
        .par_bridge()
        .for_each(|batch| {
            let file_size_map = Arc::clone(&file_size_map); // Clone Arc for the batch closure
            batch.iter().for_each(|source_file| {
                process_file(
                    source_file,
                    source_dir,
                    target_dir,
                    include_date,
                    &file_size_map,
                    verbose,
                );
            });
        });

    println!("File processing completed successfully.");
}

/// Ensures a directory exists, creating it if necessary.
/// If `force` is true, the directory is created without prompting.
/// Otherwise, the user is prompted to confirm creation.
fn ensure_directory(dir: &Path, force: bool) {
    if dir.exists() {
        return;
    }

    if force || prompt_to_create(dir) {
        fs::create_dir_all(dir).unwrap_or_else(|err| {
            eprintln!(
                "Error: Failed to create directory {}: {}",
                dir.display(),
                err
            );
            std::process::exit(1);
        });
    } else {
        eprintln!("Operation aborted.");
        std::process::exit(1);
    }
}

/// Prompts the user to confirm the creation of a directory.
/// Returns true if the user confirms, false otherwise.
fn prompt_to_create(dir: &Path) -> bool {
    print!(
        "Target directory {} does not exist. Create it? (y/n): ",
        dir.display()
    );
    io::stdout().flush().unwrap();
    let mut response = String::new();
    io::stdin().read_line(&mut response).unwrap();
    response.trim().eq_ignore_ascii_case("y")
}

fn process_file(
    source_file: &Path,
    source_dir: &Path,
    target_dir: &Path,
    include_date: bool,
    file_size_map: &Arc<Mutex<HashMap<u64, Vec<PathBuf>>>>,
    verbose: bool,
) {
    if let Ok(metadata) = source_file.metadata() {
        let size = metadata.len();

        // Check for duplicates in the file_size_map
        let is_duplicate = {
            let file_size_map = file_size_map.lock().unwrap();
            if let Some(target_files) = file_size_map.get(&size) {
                for target_file in target_files {
                    if fileutils::files_are_equal(source_file, target_file) {
                        if verbose {
                            println!(
                                "Skipped duplicate file. Source: {}, Target: {}",
                                source_file.display(),
                                target_file.display()
                            );
                        }
                        return;
                    }
                }
            }
            false
        };

        if is_duplicate {
            return;
        }

        let relative_path = source_file.strip_prefix(source_dir).unwrap_or(source_file);
        let mut dest_dir = target_dir.join("import");

        if include_date {
            let year = Local::now().year();
            let month = Local::now().month();
            dest_dir = dest_dir.join(format!("{}/{}", year, month));
        }

        dest_dir = dest_dir.join(relative_path.parent().unwrap_or_else(|| Path::new("")));

        ensure_directory(&dest_dir, true); // Ensure destination directory exists

        let dest_file = dest_dir.join(fileutils::clean_name(
            &source_file.file_name().unwrap().to_string_lossy(),
        ));

        fs::copy(source_file, &dest_file).unwrap();
        // Add the file to the map
        {
            let mut file_size_map = file_size_map.lock().unwrap();
            file_size_map
                .entry(size)
                .or_default()
                .push(dest_file.clone());
        }

        if verbose {
            println!(
                "Copied {} to {}",
                source_file.display(),
                dest_file.display()
            );
        }
    }
}
