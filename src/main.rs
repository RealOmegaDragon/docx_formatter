use std::path::Path;

use clap::Parser;

use docx_rust::document::Paragraph;
use docx_rust::{Docx, DocxError, DocxFile};

/// Work document formatter for assignments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input_file: String,

    /// Output file
    #[arg(short, long)]
    output_file: String
}

fn read_docx(path: &Path) -> Result<DocxFile, DocxError> {
    return DocxFile::from_file(path);
}

fn main() {
    let args: Args = Args::parse();

    let in_path: &Path = Path::new(&args.input_file);
    let out_path: &Path = Path::new(&args.output_file);
    let docx_file = read_docx(in_path);

    let mut docx: Docx;
    match &docx_file {
        Ok(value) => docx = value.parse().unwrap(),
        Err(value) => {
            println!("{}",value);
            return;
        }
    }

    let para = Paragraph::default().push_text("This is a test");
    docx.document.push(para);

    docx.write_file(out_path).unwrap();
}
