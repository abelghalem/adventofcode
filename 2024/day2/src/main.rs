use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./input.txt");
    let mut safe_count: u32 = 0;
    let mut dampered_safe_count: u32 = 0;

    for line in lines {
        if is_safe(line.to_owned()) {
            safe_count += 1;
        }

        let numbers: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();
        if is_safe_with_dampener(numbers.to_owned(), true) {
            dampered_safe_count += 1
        } else {
            println!("{:?}", numbers);
        }
    }

    println!("safe count: {}", safe_count);
    println!("dampered safe count: {}", dampered_safe_count);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect()
}

fn is_safe(line: String) -> bool {
    let numbers: Vec<i32> = line.split(" ").map(|n| n.parse::<i32>().unwrap()).collect();

    let mut last_nbr: i32 = numbers[0];

    if last_nbr - numbers[1] > 0 {
        for i in 1..numbers.len() {
            if last_nbr - numbers[i] > 3 || last_nbr - numbers[i] < 1 {
                return false;
            }
            last_nbr = numbers[i];
        }
    } else {
        for i in 1..numbers.len() {
            if numbers[i] - last_nbr > 3 || numbers[i] - last_nbr < 1 {
                return false;
            }
            last_nbr = numbers[i];
        }
    }

    return true;
}

fn is_safe_with_dampener(mut numbers: Vec<i32>, remove: bool) -> bool {
    let mut zero_index_test_numbers: Vec<i32> = numbers.clone();
    let mut next_index_test_numbers: Vec<i32> = numbers.clone();
    let mut last_nbr: i32 = numbers[0];

    if last_nbr - numbers[1] > 0 {
        for i in 1..numbers.len() {
            if last_nbr - numbers[i] > 3 || last_nbr - numbers[i] < 1 {
                if remove {
                    numbers.remove(i);
                    next_index_test_numbers.remove(i - 1);
                    zero_index_test_numbers.remove(0);
                    return is_safe_with_dampener(numbers, false)
                        | is_safe_with_dampener(next_index_test_numbers, false)
                        | is_safe_with_dampener(zero_index_test_numbers, false);
                }
                return false;
            }
            last_nbr = numbers[i];
        }
    } else {
        for i in 1..numbers.len() {
            if numbers[i] - last_nbr > 3 || numbers[i] - last_nbr < 1 {
                if remove {
                    numbers.remove(i);
                    next_index_test_numbers.remove(i - 1);
                    zero_index_test_numbers.remove(0);
                    return is_safe_with_dampener(numbers, false)
                        | is_safe_with_dampener(next_index_test_numbers, false)
                        | is_safe_with_dampener(zero_index_test_numbers, false);
                }
                return false;
            }
            last_nbr = numbers[i];
        }
    }

    return true;
}
