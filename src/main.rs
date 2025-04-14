use std::fs::File;
use std::io::Write;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("),\n");
    the_title.push_str("---\n");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    the_title
}

fn parse_markdown_file(_filename: &str) {
    print_short_banner();
    println!("[ INFO ] Trying to parse {} ...", _filename);

    let input_filename = Path::new(_filename);

    // Expect keyword is used to remove verbosity around rust's Result type.
    // Creates  a Panic
    let file = File::open(&input_filename).expect("[ ERROR ] Failed to open file");

    let mut tokens: Vec<String> = Vec::new();
    let mut ptag: bool = false;
    let mut htag: bool = false;

    let reader = BufReader::new(file);

    let mut output_filename = String::new();
    output_filename.push_str(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outfile =
        File::create(&output_filename).expect("[ ERROR ] Could not create output file");

    for line in reader.lines() {
        let mut output_line = String::new();

        let line_contents = line.unwrap();

        // Rust will convert the chars iterator into a Take<char> object, a special kind of iterator.
        let first_char: char = line_contents.chars().nth(0).unwrap_or(' ');

        match first_char {
            '#' => {
                if ptag {
                    ptag = false;
                    output_line.push_str("</p>\n");
                }
                if htag {
                    htag = false;
                    output_line.push_str("</h1>\n");
                }
                htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..])
            }
            _ => {
                if !ptag {
                    ptag = true;
                    output_line.push_str("<p>");
                }
                output_line.push_str(&line_contents);
            }
        }
        if ptag {
            ptag = false;
            output_line.push_str("</p>\n");
        }
        if htag {
            htag = false;
            output_line.push_str("<h1>\n")
        }
        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    for line in &tokens {
        outfile
            .write_all(line.as_bytes())
            .expect("[ ERROR ] Could not write to output file");
    }

    println!("[ INFO ] Parsing Complete!");
}

fn print_short_banner() {
    let title = get_title();
    println!("{}", title);
}

fn print_long_banner() {
    print_short_banner();
    println!("Written by: {}", env!("CARGO_PKG_AUTHORS"));
    println!("Homepage: {}", env!("CARGO_PKG_HOMEPAGE"));
    println!("Usage: tinymd <somefile>.md");
}

fn usage() {
    println!("[ ERROR] You forgot to specify the markdown file to parse!");
    print_long_banner();
}

fn main() {
    // Collect all arguments in a vector
    let args: Vec<String> = std::env::args().collect();

    // firt element of args is always the program name
    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => usage(),
    }
}
