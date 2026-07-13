<div align="center">

### 🦀 Rust Duplicate File Finder

### Fast Duplicate File Detection using SHA-256

Scan a directory and identify duplicate files by comparing both file size and SHA-256 hash.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Rust](https://img.shields.io/badge/Rust-stable-orange)
![SHA--256](https://img.shields.io/badge/Hash-SHA256-green)
![CLI](https://img.shields.io/badge/Application-CLI-lightgrey)

</div>

---

#### Overview

Rust Duplicate File Finder is a lightweight command-line utility that scans a directory and detects duplicate files.

To maximize performance, files are first grouped by their size. Only files with identical sizes are hashed using the SHA-256 algorithm, significantly reducing unnecessary hash calculations.

The application prints every detected duplicate group together with its SHA-256 hash.

Perfect for:

- Cleaning duplicate files
- Organizing downloads
- Managing backups
- Verifying copied files
- Saving disk space

---

### Features

### Duplicate Detection

- Scan any directory
- Compare files by size
- SHA-256 hash verification
- Detect identical files
- Fast grouping algorithm
- Human-readable output

---

### How It Works

The program performs duplicate detection in two stages:

1. Scan all files inside the selected directory.
2. Group files by their size.
3. Calculate SHA-256 hashes only for files with matching sizes.
4. Compare hashes.
5. Print duplicate file groups.

This approach is considerably faster than hashing every file immediately.

---

### Example Output

```text
Please provide the path:

C:\Users\Michal\Downloads

Found 125 files

Duplicate files, size 204800 bytes:
SHA-256:
6c3d6a5e2d2d3d7d4ab48f0f53b9f8e69f1d61f17d4d8d6ab3cfbd5ef8a2d731

"C:\\Users\\Michal\\Downloads\\image.png"
"C:\\Users\\Michal\\Downloads\\Copy\\image.png"
```

---

### Building

```bash
cargo build
```

Release build:

```bash
cargo build --release
```

---

### Running

```bash
cargo run
```

After launching the application, enter the directory you want to scan.

Example:

```text
Please provide the path:

C:\Users\User\Documents
```

---

### Technologies

- Rust
- SHA-256
- std::fs
- HashMap
- PathBuf

---

### Author: Michal Švrček

<div align="center">

Made with ❤️ using Rust

</div>
