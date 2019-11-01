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

fn parse_markdown_file(_file: &str) {
    println!("{}", get_title());
    println!("[ INFO ] Trying to parse {}...", _file);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.len() {
        2 => parse_markdown_file(&args[1]),
        _ => {
            println!("[ ERROR ] Invalid invocation (you done goofed!)");
            usage()
        },
    }
}