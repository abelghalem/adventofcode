use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./example.txt");

    first_part(lines.to_owned());
    second_part(lines.to_owned());
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect()
}

fn first_part(_lines: Vec<String>) {
    let result: u32 = 0;
    println!("part 1 result: {}", result);
}

fn second_part(_lines: Vec<String>) {
    let result: u32 = 0;
    println!("part 2 result: {}", result);
}
