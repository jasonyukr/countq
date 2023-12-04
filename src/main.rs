use std::io::{self, BufRead, BufWriter, Write};
use std::env;
use substring::Substring;

fn parse_input_string(input_string: &str, quote_char: char) -> Vec<String> {
    let mut result = Vec::new();
    let mut start = 0;
    let mut end = 0;
    let mut in_quotes = false;
    let mut chars = input_string.chars();
    while let Some(c) = chars.next() {
        if c == quote_char {
            in_quotes = !in_quotes;
            if !in_quotes {
                result.push(input_string.substring(start, end + 1).to_string());
                start = end + 2;
            }
        } else if c == ' ' && !in_quotes {
            if end > start {
                result.push(input_string.substring(start, end).to_string());
            }
            start = end + 1;
        }
        end += 1;
    }
    if end > start {
        result.push(input_string.substring(start, end).to_string());
    }
    result
}

fn print_usage() {
    println!("{}", "Usage : countq {-q}");
    println!("  -q   -> use single-quote character. note that double-quote is default.");
}

fn main() {
    // parse argument
    let mut quote_char = '"';
    let args: Vec<String> = env::args().collect();
    for arg in args {
        if arg == "-q" {
            quote_char = '\'';
        } else if arg == "-h" {
            print_usage();
            return;
        }
    }

    let stdout = io::stdout();
    let mut out = BufWriter::new(stdout);

    let stdin = io::stdin();
    for ln in stdin.lock().lines() {
        let line;
        match ln {
            Ok(data) => line = data,
            Err(_) => continue
        }

        let v = parse_input_string(&line, quote_char);
        writeln!(out, "{}", v.len()).unwrap();
    }
    out.flush().unwrap();
}
