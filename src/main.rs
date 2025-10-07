use std::path::Path;

use clap::Parser;

use docx_rust::document::{BodyContent, Paragraph, ParagraphContent, Run, RunContent, BreakType};
use docx_rust::{Docx, DocxFile};
use regex::Regex;

/// Work document formatter for assignments
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Input file
    #[arg(short, long)]
    input_file: String,

    /// Output file
    #[arg(short, long)]
    output_file: Option<String>
}

fn main() {
    // Parse arguments
    let args: Args = Args::parse();

    // Get argument values
    let in_path: &Path = Path::new(&args.input_file);

    let out_path: &Path;

    // Check if output_file was provided
    if let Some(output_file) = args.output_file.as_deref() {
        out_path = Path::new(output_file);
    } else {
        out_path = in_path;
    }

    // Try to get DocxFile
    let docx_file = DocxFile::from_file(in_path);

    // Define Docx variable
    let mut docx: Docx;

    // Check if the file is a valid word (.docx) file
    match &docx_file {
        Ok(value) => docx = value.parse().unwrap(),
        Err(value) => {
            println!("{}",value);
            return;
        }
    }

    // Regex match variable
    let re = Regex::new(r"^Q(\d+)[.:]\s*(.*)").unwrap();

    // New paragraph vector
    let mut new_paragraphs: Vec<Paragraph> = Vec::new();

    for content in &docx.document.body.content {
        // Check if the BodyContent is type Paragraph
        if let BodyContent::Paragraph(para) = content {
            // New paragraph string
            let mut paragraph_text = String::new();

            // Loop through ParagraphContent vector
            for para_content in &para.content {
                // Check if the ParagraphContent type is Vec<RunContent>
                if let ParagraphContent::Run(Run { content, .. }) = para_content {
                    // Loop through each element in RunContent vector
                    for run_content in content {
                        // Check if the RunContent type is text
                        if let RunContent::Text(t) = run_content {
                            // Push the text value of the paragraph to paragraph string
                            paragraph_text.push_str(&t.text);
                        }
                    }
                }
            }

            // Build new paragraph if it matches regex defined above
            if let Some(caps) = re.captures(&paragraph_text) {
                let qnum = &caps[1];
                let question = &caps[2];

                // Create new paragraph
                let mut new_para = Paragraph::default();

                // Create a run for the question line
                let mut run_q = Run::default();
                
                run_q = run_q.push_text(format!("Q{}: {}", qnum, question));
                run_q = run_q.push_break(BreakType::TextWrapping); // actual newline

                // Create a run for the answer line
                let mut run_a = Run::default();
                run_a = run_a.push_text(format!("A{}:", qnum));
                run_a = run_a.push_break(BreakType::TextWrapping);

                // Push both runs into the paragraph
                new_para = new_para.push(run_q);
                new_para = new_para.push(run_a);

                // Push to paragraph vector
                new_paragraphs.push(new_para);
            } else {
                // Append paragraph to vector if it doesn't match regex
                new_paragraphs.push(Paragraph::default().push_text(paragraph_text));
            }
        }
    }

    // Clear inital document contents
    docx.document.body.content.clear();

    // Build document contents
    for para in new_paragraphs {
        docx.document.push(para);
    }
    
    // Write the file
    docx.write_file(out_path).unwrap();
}
