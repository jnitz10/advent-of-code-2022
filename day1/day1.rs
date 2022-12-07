use std::fs;
use std::cmp::max;

fn main() {
    let file_contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");

    let lines: Vec<&str> = file_contents.lines().collect();

    let mut highest = 0;
    let mut current = 0;

    let mut vec: Vec<i32> = vec![];

    for line in &lines {
        if line.is_empty() {
            vec.push(current);
            current = 0;
            continue;
        }

        current += line.parse::<i32>().unwrap();
        highest = max(highest, current);
    }

    vec.sort_by(|a, b| b.cmp(a));

    let highest_3 = &vec[0..3].iter().sum::<i32>();

    println!("Highest: {}\nHighest 3: {}", highest, highest_3);

}