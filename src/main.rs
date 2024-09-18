use biblatex::{Bibliography, ChunksExt, Entry, Person};
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

    let bibliography = Bibliography::parse(&bib_content).unwrap();

    bibliography.iter().for_each(|entry| {
        println!("{}", entry.key);

        let output_file = format!("{}/{}.md", &args.output, entry.key);
        if let Err(e) = fs::write(&output_file, entry_to_markdown(&entry)) {
            eprintln!("Error writing output file {}: {}", output_file, e);
            exit(1);
        }
    });

    println!(
        "Successfully converted .bib entries to Markdown in {}",
        &args.output
    );
}

fn get_author_string(entry: &Entry) -> String {
    match entry.author() {
        Ok(authors) => format_authors(&authors),
        Err(_) => "Unknown".to_string(),
    }
}

fn format_authors(authors: &[Person]) -> String {
    authors
        .iter()
        .map(|author| format!("{}", author))
        .collect::<Vec<String>>()
        .join(" and ")
}

fn entry_to_markdown(entry: &Entry) -> String {
    let mut md_content = String::new();
    md_content.push_str(&format!("# {}\n\n", entry.key));
    md_content.push_str(&format!(
        "## {}: {}\n\n",
        get_author_string(entry),
        entry.title().unwrap().format_verbatim()
    ));

    entry.fields.iter().for_each(|field| {
        md_content.push_str(&format!("- {}: {}\n", field.0, field.1.format_verbatim()));
    });

    md_content
}
