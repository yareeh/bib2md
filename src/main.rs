use clap::Parser;
use std::fs;
use std::path::Path;
use std::process::exit;

/// Simple command-line tool to convert a .bib file into .md files.
#[derive(Parser, Debug)]
#[command(name = "bib2md")]
#[command(author = "Jari Makela <jari@iki.fi>")]
#[command(version = "1.0")]
#[command(about = "Converts BibTeX (.bib) entries to Markdown (.md)", long_about = None)]
struct Args {
    /// Input BibTeX file
    #[arg(short, long)]
    input: String,

    /// Output directory for Markdown files
    #[arg(short, long)]
    output: String,
}

fn main() {
    // Parse command-line arguments
    let args = Args::parse();

    // Check if the input file exists
    if !Path::new(&args.input).exists() {
        eprintln!("Error: Input file {} does not exist.", &args.input);
        exit(1);
    }

    // Create output directory if it doesn't exist
    if !Path::new(&args.output).exists() {
        if let Err(e) = fs::create_dir_all(&args.output) {
            eprintln!("Error creating output directory: {}", e);
            exit(1);
        }
    }

    // Read the input BibTeX file
    let bib_content = match fs::read_to_string(&args.input) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading input file: {}", e);
            exit(1);
        }
    };

    // Simple parsing and conversion logic
    for (i, entry) in bib_content.split("\n@").enumerate() {
        // Skip header
        if i == 0 {
            continue;
        }

        let entry = format!("@{}", entry); // Add @ back to each BibTeX entry

        // Create a markdown file for each BibTeX entry
        let output_file = format!("{}/entry_{}.md", &args.output, i);
        if let Err(e) = fs::write(&output_file, bibtex_to_markdown(&entry)) {
            eprintln!("Error writing output file {}: {}", output_file, e);
            exit(1);
        }
    }

    println!("Successfully converted .bib entries to Markdown in {}", &args.output);
}

// Basic function to convert BibTeX entry to Markdown format
fn bibtex_to_markdown(bibtex_entry: &str) -> String {
    let mut md_content = String::new();
    md_content.push_str("# BibTeX Entry\n\n");
    md_content.push_str("```bibtex\n");
    md_content.push_str(bibtex_entry);
    md_content.push_str("\n```\n");

    md_content
}
