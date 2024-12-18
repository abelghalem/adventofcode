use std::fs::read_to_string;

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

    let (page_orders, update_page_numbers): (Vec<Vec<u32>>, Vec<Vec<u32>>) =
        get_page_order_rules(lines.to_owned());

    for i in 0..update_page_numbers.len() {
        let page_numbers = update_page_numbers.get(i).unwrap();
        let mut printed_page_numbers: Vec<u32> = Vec::new();
        let mut is_valid: bool = true;

        for page_number in page_numbers {
            for rules in page_orders.to_owned() {
                if rules.get(0).unwrap() == page_number
                    && printed_page_numbers.contains(rules.get(1).unwrap())
                {
                    is_valid = false;
                    break;
                }
            }

            if !is_valid {
                break;
            }

            printed_page_numbers.push(*page_number);
        }

        if is_valid {
            let page = update_page_numbers.get(i).unwrap();
            result += page.get(page.len() / 2).unwrap();
        }
    }

    println!("part 1 result: {}", result);
}

fn second_part(lines: Vec<String>) {
    let mut result: u32 = 0;

    let (page_orders, update_page_numbers): (Vec<Vec<u32>>, Vec<Vec<u32>>) =
        get_page_order_rules(lines.to_owned());

    let mut new_update_page_numbers: Vec<Vec<u32>> = update_page_numbers.to_owned();

    let mut haha: Vec<usize> = Vec::new();

    for i in 0..new_update_page_numbers.to_owned().len() {
        for _ in 0..15 {
            let a = new_update_page_numbers.to_owned();
            let page_numbers = a.get(i).unwrap();
            let mut printed_page_numbers: Vec<u32> = Vec::new();

            for page_number_index in 0..page_numbers.len() {
                let page_number = page_numbers[page_number_index];

                for rules in page_orders.to_owned() {
                    if *rules.get(0).unwrap() == page_number
                        && printed_page_numbers.contains(rules.get(1).unwrap())
                    {
                        if !haha.contains(&i) {
                            haha.push(i);
                        }
                        let mut tmp: Vec<u32> = new_update_page_numbers[i].to_owned();
                        let index = tmp.iter().position(|x| x == rules.get(1).unwrap()).unwrap();
                        move_value(&mut tmp, index, new_update_page_numbers[i].len() - 1);
                        new_update_page_numbers[i] = tmp;
                    }
                }

                printed_page_numbers.push(page_number);
            }
        }
    }

    for i in haha {
        let page = new_update_page_numbers.get(i).unwrap();
        result += page.get(page.len() / 2).unwrap();
    }

    println!("part 2 result: {}", result);
}

fn get_page_order_rules(lines: Vec<String>) -> (Vec<Vec<u32>>, Vec<Vec<u32>>) {
    let mut page_orders: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();
    let mut page_updates: Vec<Vec<u32>> = Vec::<Vec<u32>>::new();

    let mut page_orders_mode = true;

    for line in lines.to_owned() {
        if line == "" {
            page_orders_mode = false;
            continue;
        }

        if page_orders_mode == true {
            page_orders.push(line.split("|").map(|s| s.parse::<u32>().unwrap()).collect());
        } else {
            page_updates.push(line.split(",").map(|s| s.parse::<u32>().unwrap()).collect());
        }
    }

    return (page_orders, page_updates);
}

fn move_value(vec: &mut Vec<u32>, from: usize, to: usize) {
    if from >= vec.len() || to >= vec.len() {
        panic!("Index out of bounds");
    }
    // Remove the value from the original position
    let value = vec.remove(from);
    // Insert the value at the new position
    vec.insert(to, value);
}
