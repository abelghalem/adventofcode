use regex::Regex;
use std::fs::read_to_string;
fn main() {
    let lines: Vec<String> = read_lines("./input.txt");

    first_part(lines.to_owned());
    second_part(lines.to_owned());
}

fn first_part(lines: Vec<String>) {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut result: u32 = 0;

    for line in lines {
        for (_, [first, second]) in re.captures_iter(&line).map(|c| c.extract()) {
            let f: u32 = first.parse().unwrap();
            let s: u32 = second.parse().unwrap();
            result += f * s;
        }
    }

    // First part result is: 159833790
    println!("First part result is: {}", result);
}

fn second_part(lines: Vec<String>) {
    let mut should_do: bool = true;
    let mut result: u32 = 0;

    for line in lines {
        for (i, c) in line.char_indices() {
            if c == 'd' {
                if &line[i..i + 4] == "do()" {
                    should_do = true;
                } else if &line[i..i + 7] == "don't()" {
                    should_do = false;
                }
            } else if c == 'm' {
                let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
                let caps_option = re.captures_at(&line, i).map(|c| c);

                if !caps_option.is_none() {
                    let caps = caps_option.unwrap();
                    if caps.get(0).unwrap().start() == i && should_do {
                        let f: u32 = caps.get(1).unwrap().as_str().parse().unwrap();
                        let s: u32 = caps.get(2).unwrap().as_str().parse().unwrap();
                        result += f * s;
                    }
                }
            }
        }
    }

    // Second part result is: 89349241
    println!("Second part result is: {}", result);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect()
}
