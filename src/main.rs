use sha2::{Sha256, Digest};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::io::stdin;
use std::fs;

/// `Summary`: File info structure
struct FileInfo {
    path: PathBuf,
    size: u64,
}

fn main() {
    let mut path = String::new(); 

    println!("Please provide the path: ");
    stdin().read_line(&mut path).expect("Failed to read input");

    let path = path.trim(); 
    let files = find_files(path);
    

    let mut files_by_size: HashMap<u64, Vec<FileInfo>> = HashMap::new();

    for file in files {
        files_by_size
            .entry(file.size)
            .or_default()
            .push(file);
    }

    for (size, grouped_files) in files_by_size {
        if grouped_files.len() < 2 {
            continue;
        }

        let mut files_by_hash: HashMap<String, Vec<PathBuf>> = HashMap::new();
        let mut duplicates_found = false;

        for file in grouped_files {
            let hash = calculate_hash(&file);

            files_by_hash
                .entry(hash)
                .or_default()
                .push(file.path);
        }

        for (hash, paths) in files_by_hash {
            if paths.len() >= 2 {
                duplicates_found = true;

                println!("\nDuplicate files, size {} bytes:", size);
                println!("SHA-256: {}", hash);

                for path in paths {
                    println!("{:?}", path);
                }
            }
        }

        if !duplicates_found {
            println!("No duplicate files found.");
        }
    }
}

/// `Summary`: Fuction for transforming bytes from a file into hex using SHA-256 algorithm
/// 
/// `Returns`: Bytes in hex
fn calculate_hash(file: &FileInfo) -> String {
    let bytes = fs::read(&file.path)
        .expect("Failed to read file");

    hex::encode(Sha256::digest(bytes))
}

/// `Summary`: Fuction for locating files in a given path
/// 
/// `Returns`: Vector of `FileInfo`
fn find_files(path_name: &str) -> Vec<FileInfo> {
    let pth = Path::new(path_name);

    let files = fs::read_dir(pth)
        .expect("Specified directory could not be opened");

    let file_infos: Vec<FileInfo> = files
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_file())
        .filter_map(|path| {
            let metadata = fs::metadata(&path).ok()?;

            Some(FileInfo {
                path,
                size: metadata.len(),
            })
        })
        .collect();

    println!("Found {} files", file_infos.len());

    file_infos
}