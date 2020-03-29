use std::io::{self, BufRead, BufReader, BufWriter};
use std::io::Write;
use std::env;

fn columns(line: &String, column_numbers: &Vec<usize>) -> String {
    column_numbers.iter()
        .filter_map( |n| line.split_whitespace().nth(*n))
        .collect::<Vec<&str>>()
        .join("\t")
}

fn main() {
    let args: Vec<usize> = env::args()
        .map(|arg| arg.parse::<usize>())
        .filter_map(Result::ok)
        .collect();

    let input = io::stdin();
    let mut reader = BufReader::new(input.lock());

    let output = io::stdout();
    let mut writer = BufWriter::new(output.lock());
    
    let mut buf = String::new();

    while reader.read_line(&mut buf).unwrap() > 0 {
        writeln!(writer, "{}", columns(&buf, &args)).unwrap();
        buf.clear();
    }
}
