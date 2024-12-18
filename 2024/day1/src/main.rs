use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_lines("./src/input.txt");

    let mut first_list: Vec<u32> = Vec::with_capacity(lines.len());
    let mut second_list: Vec<u32> = Vec::with_capacity(lines.len());

    for line in lines {
        let (first_part, second_part) = line.split_once("   ").unwrap();
        first_list.push(first_part.parse::<u32>().unwrap());
        second_list.push(second_part.parse::<u32>().unwrap());
    }

    let sorted_first_list: Vec<u32> = quick_sort(first_list.to_owned());
    let sorted_second_list: Vec<u32> = quick_sort(second_list.to_owned());

    let difference: u32 =
        compute_distance(sorted_first_list.to_owned(), sorted_second_list.to_owned());

    println!("first part (difference): {}", difference);

    let similarity: u32 = compute_similarity(first_list.to_owned(), second_list.to_owned());

    println!("second part (similarity): {}", similarity);
}

fn read_lines(filename: &str) -> Vec<String> {
    read_to_string(filename)
        .unwrap()
        .lines()
        .map(String::from) // make each slice into a string
        .collect()
}

fn quick_sort(array: Vec<u32>) -> Vec<u32> {
    let mut newvec: Vec<u32> = array.clone();

    if newvec.len() <= 1 {
        return newvec;
    }

    let mut i: i32 = -1;
    let pivot_index: usize = newvec.len() - 1;
    let last_elem: u32 = newvec[pivot_index];

    for j in 0..pivot_index - 1 {
        if newvec[j] <= last_elem {
            i += 1;
            newvec.swap(i as usize, j);
        }
    }

    let swap_index: usize = (i + 1) as usize;
    newvec.swap(swap_index, pivot_index);

    let mut last_part: Vec<u32> = newvec.split_off(swap_index);
    let pivot: u32 = last_part.remove(0);

    return [quick_sort(newvec), [pivot].to_vec(), quick_sort(last_part)].concat();
}

fn compute_distance(first: Vec<u32>, second: Vec<u32>) -> u32 {
    let mut i: usize = 0;
    let mut distance: u32 = 0;

    while i < first.len() {
        let f: u32 = first[i];
        let s: u32 = second[i];
        let d: u32 = if f < s { s - f } else { f - s };

        distance += d;
        i += 1;
    }

    return distance;
}

fn compute_similarity(first: Vec<u32>, second: Vec<u32>) -> u32 {
    let mut similarity: u32 = 0;

    for i in 0..first.len() {
        let f: u32 = first[i];

        similarity += f * (second.iter().filter(|&x| *x == f).count() as u32);
    }

    similarity
}
