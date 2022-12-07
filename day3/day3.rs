use std::collections::{HashMap, HashSet};
use std::fs;

fn main() {
    let mut priorities = HashMap::new();

    let items = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    // generate priorities for each item
    for i in 0..items.len() {
        priorities.insert(items.as_bytes()[i] as char, (i + 1) as i32);
    }

    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let rucksacks: Vec<&str> = input.lines().collect();

    fn get_priority(rucksack: &str, priorities_map: &HashMap<char, i32>) -> i32 {
        // split rucksack in half
        let (compartment1, compartment2) = rucksack.split_at(rucksack.len() / 2);

        // find duplicate item
        let mut compartment1_items = HashSet::new();
        for item in compartment1.chars() {
            compartment1_items.insert(item);
        }

        for item in compartment2.chars() {
            if compartment1_items.contains(&item) {
                return priorities_map[&item];
            }
        }
        return 0;
    }

    let priorities_sum = rucksacks
        .iter()
        .map(|rucksack| get_priority(rucksack, &priorities))
        .sum::<i32>();

    println!("Sum of priorities: {}", priorities_sum);
}
