use std::fs;

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Error reading file");
    let lines = input.lines();

    fn format_input(lines: Vec<&str>) -> (Vec<Vec<char>>, Vec<Instruction>) {
        let mut i = 0;
        for line in &lines {
            i += 1;
            if line.len() == 0 {
                break;
            }
        }
        let mut initial = lines[..i - 1].to_vec();
        initial.reverse();
        (get_stacks(initial), get_instructions(lines[i..].to_vec()))
    }

    let (stacks, instructions) = format_input(lines.collect());

    let new_stacks = execute_instructions(instructions, stacks);

    let mut answer = String::new();

    for stack in new_stacks {
        answer.push(stack.last().unwrap().clone());
    }

    println!("{}", answer);
}

/* turns stack diagram from input into a vec of stacks.
    [D]
[N] [C]
[Z] [M] [P]
 1   2   3
becomes [[Z,N],[M,C,D],[P]]*/
fn get_stacks(input: Vec<&str>) -> Vec<Vec<char>> {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    for n in 0..input[0].len() {
        if input[0].chars().nth(n).unwrap().is_numeric() {
            let mut stack: Vec<char> = Vec::new();
            for i in 1..input.len() {
                if input[i].chars().nth(n).unwrap() == ' ' {
                    continue;
                }
                stack.push(input[i].chars().nth(n).unwrap());
            }
            stacks.push(stack);
        }
    }
    stacks
}

// turns instructions from input into a vec of instructions
fn get_instructions(input: Vec<&str>) -> Vec<Instruction> {
    let mut instructions: Vec<Instruction> = Vec::new();
    for line in input {
        let split: Vec<&str> = line.split_whitespace().collect();
        let count = split[1].parse::<usize>().unwrap();
        let from = split[3].parse::<usize>().unwrap();
        let to = split[5].parse::<usize>().unwrap();
        instructions.push(Instruction { count, from, to });
    }
    instructions
}

fn execute_instructions(instructions: Vec<Instruction>, stacks: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut stacks = stacks;
    for instruction in instructions {
        let mut count = instruction.count;
        let from = instruction.from - 1;
        let to = instruction.to - 1;
        let mut temp: Vec<char> = Vec::new();
        let mut temp_count = count;
        while temp_count > 0 {
            temp.push(stacks[from].pop().unwrap());
            temp_count -= 1;
        }
        while count > 0 {
            stacks[to].push(temp.pop().unwrap());
            count -= 1;
        }
    }
    stacks
}
