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

fn add_p_end_tag(output_line: &mut String, mut _ptag: bool) {
    if _ptag {
        _ptag = false;
        output_line.push_str("</p>\n");
    }
}

fn add_p_open_tag(output_line: &mut String, mut _ptag: bool) {
    if !_ptag {
        _ptag = true;
        output_line.push_str("\n<p>");
    }
}

fn add_h_end_tag(output_line: &mut String, mut _htag: bool, level: i8) {
    if _htag {
        _htag = false;
        let _header_tag = String::from(format!("</{}{}>\n\n", "h", level));
        output_line.push_str(String::from(_header_tag).as_ref());
    }
}

fn add_h_open_tag(output_line: &mut String, mut _htag: bool, line_contents: &String, level: i8) -> () {
    _htag = true;
    let _header_tag = String::from(format!("h{}", level));
    output_line.push_str(format!("\n\n<{}>", _header_tag).as_ref());
    output_line.push_str(&line_contents[2..]);
}

fn parse_markdown_file(_filename: &str) {
    println!("{}", get_title());
    println!("[ INFO ] Parsing file {}...", _filename);

    let input_filename = Path::new(_filename);

    let file = match File::open(&input_filename) {
        Err(err) => panic!("[ ERROR ] Couldn't open: {}", err.description()),
        Ok(value) => value,
    };

    // Store all our tokens in here
    let mut tokens: Vec<String> = Vec::new();

    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line_contents = line.unwrap().to_string();
        process_line( &mut tokens, line_contents);
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

fn process_line(tokens: &mut Vec<String>, line_contents: String) {
    let mut _ptag: bool = false;
    let mut _htag: bool = false;
    let mut first_char: Vec<char> = line_contents.chars().take(1).collect();
    let mut output_line = String::new();
    let v = first_char.pop();

    match v {
        Some('#') => {
            add_p_end_tag(&mut output_line, _ptag);
            add_h_end_tag(&mut output_line, _htag, 1);
            add_h_open_tag(&mut output_line, _htag, &line_contents, 1)
        }
        _ => {
            add_p_open_tag(&mut output_line, _ptag);
            output_line.push_str(&line_contents);
        }
    }
    add_p_end_tag(&mut output_line, _ptag);
    add_h_end_tag(&mut output_line, _htag, 1);
    tokens.push(output_line);
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