use std::fs::read_to_string;
use std::{thread, time};

use strum_macros::EnumIter; // 0.17.1

#[derive(Debug, Eq, PartialEq, EnumIter, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

fn main() {
    let lines: Vec<String> = read_lines("./input.txt");

    // part 1 result: 5067
    first_part(lines.to_owned());

    // part 2 result: 1793
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
    let mut map: Vec<String> = lines.clone();
    let is_debug = false;
    let delay = time::Duration::from_millis(50);

    let ((mut x, mut y), mut direction) = find_starting_position_direction(lines.to_owned());

    let mut exited = false;

    while !exited {
        let next_char = get_next_char(lines.to_owned(), x, y, direction);

        match next_char.as_deref() {
            Some("#") => match direction {
                Direction::Up => {
                    direction = Direction::Right;
                }
                Direction::Right => {
                    direction = Direction::Down;
                }
                Direction::Down => {
                    direction = Direction::Left;
                }
                Direction::Left => {
                    direction = Direction::Up;
                }
            },
            Some(_) => {
                let (new_x, new_y) = get_next_position(x, y, direction);
                map[x].replace_range(y..y + 1, "X");
                x = new_x;
                y = new_y;
                if !exited {
                    map[x].replace_range(y..y + 1, get_cursor_char(&direction));
                }

                if is_debug {
                    thread::sleep(delay);
                    std::process::Command::new("clear").status().unwrap();
                    for line in map.to_owned() {
                        println!("{}", line);
                    }
                }
            }
            None => exited = true,
        }
    }

    println!(
        "part 1 result: {}",
        map.join("").chars().filter(|c| *c == 'X').count()
    );
}

fn second_part(lines: Vec<String>) {
    let mut result: u32 = 0;
    let is_debug = false;
    let delay = time::Duration::from_millis(200);

    let mut exited = false;
    let mut map: Vec<String> = lines.clone();
    let ((mut x, mut y), mut direction) = find_starting_position_direction(lines.to_owned());

    while !exited {
        let next_char = get_next_char(lines.to_owned(), x, y, direction);

        match next_char.as_deref() {
            Some("#") => match direction {
                Direction::Up => {
                    direction = Direction::Right;
                }
                Direction::Right => {
                    direction = Direction::Down;
                }
                Direction::Down => {
                    direction = Direction::Left;
                }
                Direction::Left => {
                    direction = Direction::Up;
                }
            },
            Some(_) => {
                let (new_x, new_y) = get_next_position(x, y, direction);
                map[x].replace_range(y..y + 1, "X");
                x = new_x;
                y = new_y;
                if !exited {
                    map[x].replace_range(y..y + 1, get_cursor_char(&direction));
                }
            }
            None => exited = true,
        }
    }

    for i in 0..lines.len() {
        for j in 0..lines[i].len() {
            let mut new_lines = lines.clone();

            let mut position_save: Vec<((usize, usize), Direction)> = Vec::new();

            new_lines[i].replace_range(j..j + 1, "#");

            let ((mut x, mut y), mut direction) =
                find_starting_position_direction(new_lines.to_owned());
            let mut map: Vec<String> = new_lines.clone();
            let mut exited = false;

            while !exited {
                let next_char = get_next_char(new_lines.to_owned(), x, y, direction);
                if position_save.contains(&((x, y), direction)) {
                    result += 1;
                    break;
                }

                position_save.push(((x, y), direction));

                match next_char.as_deref() {
                    Some("#") => match direction {
                        Direction::Up => {
                            direction = Direction::Right;
                        }
                        Direction::Right => {
                            direction = Direction::Down;
                        }
                        Direction::Down => {
                            direction = Direction::Left;
                        }
                        Direction::Left => {
                            direction = Direction::Up;
                        }
                    },
                    Some(_) => {
                        if is_debug {
                            map[x].replace_range(y..y + 1, "X");
                        }

                        let (new_x, new_y) = get_next_position(x, y, direction);

                        x = new_x;
                        y = new_y;

                        if is_debug {
                            if !exited {
                                map[x].replace_range(y..y + 1, get_cursor_char(&direction));
                            }
                            thread::sleep(delay);
                            std::process::Command::new("clear").status().unwrap();
                            for line in map.to_owned() {
                                println!("{}", line);
                            }
                        }
                    }
                    None => exited = true,
                }
            }
        }
    }

    // for pos_dir in position_save {
    //     println!("({}, {}): {:?}", pos_dir.0 .0, pos_dir.0 .1, pos_dir.1);
    // }

    println!("part 2 result: {}", result);
}

fn find_starting_position_direction(lines: Vec<String>) -> ((usize, usize), Direction) {
    for i in 0..lines.len() {
        for (j, c) in lines[i].char_indices() {
            if c == '^' {
                return ((i, j), Direction::Up);
            } else if c == '>' {
                return ((i, j), Direction::Right);
            } else if c == 'v' {
                return ((i, j), Direction::Down);
            } else if c == '<' {
                return ((i, j), Direction::Down);
            }
        }
    }

    return ((0, 0), Direction::Up);
}

fn get_next_char(lines: Vec<String>, x: usize, y: usize, direction: Direction) -> Option<String> {
    let (new_x, new_y) = get_next_position(x, y, direction);

    return match lines.get(new_x) {
        Some(line) => {
            if new_y > line.len() {
                return None;
            }

            return match line.get(new_y..new_y + 1) {
                Some(c) => Some(String::from(c)),
                None => None,
            };
        }
        None => None,
    };
}

fn get_next_position(x: usize, y: usize, direction: Direction) -> (usize, usize) {
    return match direction {
        Direction::Up => ((x as i32 - 1) as usize, y),
        Direction::Right => (x, y + 1),
        Direction::Down => (x + 1, y),
        Direction::Left => (x, (y as i32 - 1) as usize),
    };
}

fn get_cursor_char(direction: &Direction) -> &str {
    return match direction {
        Direction::Up => "^",
        Direction::Right => ">",
        Direction::Down => "v",
        Direction::Left => "<",
    };
}
