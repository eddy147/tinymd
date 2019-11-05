use std::path::Path;
use std::fs::File;
use std::error::Error;
use std::io::{BufReader, BufRead};
use std::io::Write;

fn usage() {
    let usage = String::from("Usage: tinymd <somefile>.md");
    let title_and_usage = format!("{}\n{}", get_title(), usage);

    println!("{}", title_and_usage);
}

fn get_title() -> String {
    let the_title = format!(
        "{} (v{}), {} {}",
        env!("CARGO_PKG_NAME"),
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_DESCRIPTION"),
        env!("CARGO_PKG_HOMEPAGE")
    );

    return the_title;
}

fn parse_markdown_file(_filename: &str) {
    println!("{}", get_title());
    println!("[ INFO ] Parsing file {}...", _filename);

    let input_filename = Path::new(_filename);

    let file = match File::open(&input_filename) {
        Err(err) => panic!("[ ERROR ] Couldn't open: {}", err.description()),
        Ok(value) => value,
    };

    let mut _ptag: bool = false;
    let mut _htag: bool = false;

    // Store all our tokens in here
    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let mut line_contents = line.unwrap().to_string();
        let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
        let mut output_line = String::new();

        let v = first_char.pop();
        //eprintln!("v = {:?}", v);
        match v {
            Some('#') => {
                if _ptag {
                    _ptag = false;
                    output_line.push_str("</p>\n");
                }
                if _htag {
                    _htag = false;
                    output_line.push_str("</h1>\n");
                }
                _htag = true;
                output_line.push_str("\n\n<h1>");
                output_line.push_str(&line_contents[2..]);
            }
            _ => {
                if !_ptag {
                    _ptag = true;
                    output_line.push_str("\n<p>");
                }
                output_line.push_str(&line_contents);
            }
        }

        if _ptag {
            _ptag = false;
            output_line.push_str("</p>\n");
        }
        if _htag {
            _htag = false;
            output_line.push_str("</h1>\n");
        }

        if output_line != "<p></p>\n" {
            tokens.push(output_line);
        }
    }

    let mut output_filename = String::from(&_filename[.._filename.len() - 3]);
    output_filename.push_str(".html");

    let mut outputfile = File::create(output_filename.to_string())
        .expect("[ ERROR ] Could not create output file!");

    for t in &tokens {
        outputfile.write_all(t.as_bytes())
            .expect("[ ERROR ] Could not write to output file!")
    }

    println!("[ INFO ] Parsing complete!");
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage()
        }
    }
}