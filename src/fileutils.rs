use std::fs;
use std::path::{Path, PathBuf};

/// Collect all files recursively from a directory
pub fn collect_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                files.extend(collect_files(&path));
            } else {
                files.push(path);
            }
        }
    }
    files
}

/// Check if two files are equal using the `cmp` command
pub fn files_are_equal(file1: &Path, file2: &Path) -> bool {
    // Debugging statement to show the paths of the files being compared
    println!(
        "Debug: Comparing files: '{}' and '{}'",
        file1.display(),
        file2.display()
    );

    std::process::Command::new("cmp")
        .arg("-s")
        .arg(file1)
        .arg(file2)
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

/// Clean and sanitize filenames
pub fn clean_name(original_name: &str) -> String {
    let cleaned_name = original_name.replace(
        |c: char| !c.is_ascii_alphanumeric() && c != '.' && c != '_',
        "_",
    );

    // Debugging statement to show the cleaned name before checking its length
    //    println!("Debug: cleaned_name before trimming: '{}'", cleaned_name);

    if cleaned_name.is_empty() {
        let generated_name = format!("file_{}", chrono::Local::now().format("%Y%m%d%H%M%S"));

        // Debugging statement to show the generated name
        //        println!(
        //            "Debug: cleaned_name was too short, generated name: '{}'",
        //            generated_name
        //        );

        generated_name
    } else {
        let final_name = cleaned_name.trim_matches('_').to_string();

        // Debugging statement to show the final cleaned name
        //        println!("Debug: final cleaned_name after trimming: '{}'", final_name);

        final_name
    }
}
