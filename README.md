# docx_formatter
*A lightweight Rust command-line tool for automatically formatting question and answer pairs in Word documents.*

## Overview
**docx_formatter** is a simple CLI utility built in **Rust** that streamlines the process of formatting **question and answer pairs** in `.docx` files. Instead of manually editing and aligning text for school or work assignments, the tool automatically parses each question line, detects question numbers, and inserts properly formatted **Question (Q)** and **Answer (A)** sections.

## Purpose
This project was created to remove the **tedium of repetitive document formatting**, especially when preparing assignments that use a question-and-answer structure. It demonstrates how **Rust’s safety, performance, and clarity** can be leveraged to automate everyday text processing tasks while keeping the workflow fast and consistent.

## Features
- Parses `.docx` files and identifies question patterns like `Q1.`, `Q2:` etc.  
- Automatically reformats each into a clean:
   ```
   Q1: [Question text]
   A1:
   ```
- Preserves non-question paragraphs as-is.  
- Outputs to a new formatted Word document.  
- Reduces manual editing time for academic or professional documents.  

## Built With
- **Rust** (Edition 2024)  
- Command-line argument parsing via [`clap`](https://crates.io/crates/clap)  
- Word document handling with [`docx-rust`](https://crates.io/crates/docx-rust)  
- Regular expression parsing through [`regex`](https://crates.io/crates/regex)

## Project Structure
```
docx_formatter/
│
├── src/
│ └── main.rs # Main CLI logic and document formatting
│
├── Cargo.toml # Project manifest and dependencies
└── Cargo.lock # Dependency resolution (auto-generated)
```

## Requirements
- **Rust 1.80+**  
- A valid `.docx` input file  

## Running the Project
1. Clone the repository:  
   ```bash
   git clone https://github.com/RealOmegaDragon/docx_formatter.git
   cd docx_formatter
   ```
2. Build the binary:
   ```bash
   cargo build --release
   ```
3. Run the formatter
   ```bash
   ./target/release/docx_formatter -i input.docx -o formatted.docx
   ```
The tool reads your input Word file, detects question patterns, and writes a new formatted version to the specified output path.

## Example
### Input (file contents):
```
Q1. What is Rust?
Q2: Why is it safe?
```
### Output:
```
Q1: What is Rust?
A1:

Q2: Why is it safe?
A2:
```

## What I Learned
Developing **docx_formatter** helped me strengthen:
- Practical experience with **Rust CLI application design**.
- Understanding of **file I/O and structured document processing**.
- Proficiency using **regex-based text parsing**.
- Integration of **third-party crates** for real-world automation tasks.

## Author
**Braxton Newhall**  
Social: [LinkedIn](https://linkedin.com/in/braxton-newhall-128597333) • [GitHub](https://github.com/RealOmegaDragon)  
Email: braxtonnewhall@gmail.com

## License
This project is open-source under the [MIT License](LICENSE).
