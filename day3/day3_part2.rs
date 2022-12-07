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

    let elf_groups: Vec<Vec<&str>> = rucksacks.chunks(3).map(|c| c.to_vec()).collect();

    fn get_badge_priority(elf_group: &Vec<&str>, priorities_map: &HashMap<char, i32>) -> i32 {
        let mut seen_items = HashMap::new();
        for elf in elf_group {
            let mut elf_items = HashSet::new();

            for item in elf.chars() {
                if elf_items.contains(&item) {
                    continue;
                }
                elf_items.insert(item);
                let count = seen_items.entry(item).or_insert(0);
                *count += 1;
                if *count == elf_group.len() {
                    return priorities_map[&item];
                }
            }
        }
        return 0;
    }

    let priorities_sum = elf_groups
        .iter()
        .map(|elf_group| get_badge_priority(elf_group, &priorities))
        .sum::<i32>();

    println!("Sum of priorities: {}", priorities_sum);
}
