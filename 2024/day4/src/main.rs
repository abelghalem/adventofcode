use std::fs::read_to_string;
use strum::IntoEnumIterator;
use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, EnumIter, Clone, Copy)]
enum Direction {
    UpLeft,
    Up,
    UpRight,
    Left,
    Right,
    DownLeft,
    Down,
    DownRight,
}

fn main() {
    let lines: Vec<String> = read_lines("./input.txt");

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

fn first_part(lines: Vec<String>) {
    let mut result: u32 = 0;

    for i in 0..lines.len() {
        for (j, c) in lines[i].char_indices() {
            if c == 'X' {
                for direction in Direction::iter() {
                    if is_valid_xmas(lines.to_owned(), i, j, direction, 'M', true) {
                        result += 1;
                    }
                }
            }
        }
    }

    println!("part 1 result: {}", result);
}

fn second_part(lines: Vec<String>) {
    let mut result: u32 = 0;

    for i in 1..lines.len() - 1 {
        for (j, c) in lines[i].char_indices() {
            if j == 0 || j == lines[i].len() - 1 {
                continue;
            }

            if c == 'A' {
                let mut mas_count: u32 = 0;

                if is_valid_xmas(lines.to_owned(), i, j, Direction::UpLeft, 'M', false)
                    && is_valid_xmas(lines.to_owned(), i, j, Direction::DownRight, 'S', false)
                {
                    mas_count += 1;
                }
                if is_valid_xmas(lines.to_owned(), i, j, Direction::UpRight, 'M', false)
                    && is_valid_xmas(lines.to_owned(), i, j, Direction::DownLeft, 'S', false)
                {
                    mas_count += 1;
                }

                if is_valid_xmas(lines.to_owned(), i, j, Direction::DownLeft, 'M', false)
                    && is_valid_xmas(lines.to_owned(), i, j, Direction::UpRight, 'S', false)
                {
                    mas_count += 1;
                }
                if is_valid_xmas(lines.to_owned(), i, j, Direction::DownRight, 'M', false)
                    && is_valid_xmas(lines.to_owned(), i, j, Direction::UpLeft, 'S', false)
                {
                    mas_count += 1;
                }

                if mas_count == 2 {
                    result += 1;
                }
            }
        }
    }

    println!("part 2 result: {}", result);
}

fn is_valid_xmas(
    lines: Vec<String>,
    i: usize,
    j: usize,
    direction: Direction,
    next_char: char,
    search_full: bool,
) -> bool {
    let (plus_i, plus_j) = get_direction_calculation(direction);

    match lines.get(((i as i32) + plus_i) as usize) {
        None => false,
        Some(line) => {
            match line
                .chars()
                .collect::<Vec<char>>()
                .get(((j as i32) + plus_j) as usize)
            {
                None => false,
                Some(a) => {
                    if *a == next_char {
                        if next_char == 'S' || !search_full {
                            return true;
                        }
                        return is_valid_xmas(
                            lines,
                            ((i as i32) + plus_i) as usize,
                            ((j as i32) + plus_j) as usize,
                            direction,
                            get_next_char(next_char),
                            search_full,
                        );
                    }

                    return false;
                }
            }
        }
    };
    return false;
}

fn get_direction_calculation(direction: Direction) -> (i32, i32) {
    match direction {
        Direction::UpLeft => (-1, -1),
        Direction::Up => (-1, 0),
        Direction::UpRight => (-1, 1),
        Direction::Left => (0, -1),
        Direction::Right => (0, 1),
        Direction::DownLeft => (1, -1),
        Direction::Down => (1, 0),
        Direction::DownRight => (1, 1),
    }
}

fn get_next_char(c: char) -> char {
    if c == 'M' {
        return 'A';
    } else if c == 'A' {
        return 'S';
    }

    return '0';
}
