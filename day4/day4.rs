use std::fs;

struct SectionAssignment {
    section_start: usize,
    section_end: usize,
}

impl SectionAssignment {
    fn new(start: usize, end: usize) -> SectionAssignment {
        SectionAssignment {
            section_start: start,
            section_end: end,
        }
    }
    fn contains_other(&self, other: &SectionAssignment) -> bool {
        self.section_start <= other.section_start && self.section_end >= other.section_end
    }
}

struct AssignmentPair {
    assignment1: SectionAssignment,
    assignment2: SectionAssignment,
}

impl AssignmentPair {
    fn new(input: &str) -> AssignmentPair {
        let sections: Vec<&str> = input.split(',').collect();
        let section1: Vec<&str> = sections[0].split('-').collect();
        let section2: Vec<&str> = sections[1].split('-').collect();
        AssignmentPair {
            assignment1: SectionAssignment::new(
                section1[0].parse().unwrap(),
                section1[1].parse().unwrap(),
            ),
            assignment2: SectionAssignment::new(
                section2[0].parse().unwrap(),
                section2[1].parse().unwrap(),
            ),
        }
    }
    fn contains(&self) -> i32 {
        if self.assignment1.contains_other(&self.assignment2) {
            return 1;
        } else if self.assignment2.contains_other(&self.assignment1) {
            return 1;
        } else {
            return 0;
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading input.txt");

    let lines: Vec<&str> = input.lines().collect();

    let pairs: Vec<AssignmentPair> = lines.iter().map(|line| AssignmentPair::new(line)).collect();

    let count: i32 = pairs.iter().map(|pair| pair.contains()).sum();

    println!("Count: {}", count);
}
