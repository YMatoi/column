use std::io::{self, BufRead, BufReader, BufWriter};
use std::io::Write;
use std::env;

#[macro_use]
extern crate clap;
use clap::Arg;

fn columns(line: &String, column_numbers: &Vec<usize>)-> String {
    column_numbers.iter()
        .filter_map( |n| line.split_whitespace().nth(*n))
        .collect::<Vec<&str>>()
        .join("\t")
}

fn main_loop(func: &dyn Fn(&String) -> String) {
    let input = io::stdin();
    let mut reader = BufReader::new(input.lock());

    let output = io::stdout();
    let mut writer = BufWriter::new(output.lock());

    let mut buf = String::new();

    while reader.read_line(&mut buf).unwrap() > 0 {
        writeln!(writer, "{}", func(&buf)).unwrap();
        buf.clear();
    }
}

fn main() {
    let matches = app_from_crate!()
        .arg(Arg::with_name("columns")
             .multiple(true)
             .takes_value(true)
             .required(true)
        )
        .get_matches();

    let args: Vec<usize> = matches.values_of("columns").unwrap()
        .map(|arg| arg.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    let func = |s: &String| columns(s, &args);

    main_loop(&func);
}
