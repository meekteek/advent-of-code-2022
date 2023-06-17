#[derive(Debug)]
pub struct Move {
    from: usize,
    to: usize,
    count: usize,
}

pub fn parse(input:&str) -> (Vec<Move>, Vec<Vec<char>>) {
    let stack_size_max = input
    .lines()
    .position(|line| line.chars().any(|c| c.is_digit(10)))
    .unwrap();
let stack_count = input
    .lines()
    .nth(stack_size_max)
    .unwrap()
    .to_string()
    .split_whitespace()
    .count();

let mut stacks: Vec<Vec<char>> = vec![Vec::new(); stack_count];
for line in input.lines().take(stack_size_max) {
    for (index, element) in line
        .chars()
        .skip(1)
        .step_by(4)
        .enumerate()
    {
        if !(element.is_whitespace()) {
            stacks[index].push(element);
        }
    }
}
for stack in stacks.iter_mut() {
    stack.reverse();
}

let mut steps: Vec<Move> = Vec::new();
for line in input.lines().skip(stack_size_max + 1) {
    if !line.is_empty() {
        let words: Vec<&str> = line.split_whitespace().collect();
        let step = Move {
            count: words[1].parse::<usize>().unwrap(),
            to: words[5].parse::<usize>().unwrap(),
            from: words[3].parse::<usize>().unwrap(),
        };
        steps.push(step);
    }
}
return (steps, stacks);
}

pub fn part_one_solve(steps: Vec<Move>, mut stacks: Vec<Vec<char>>) -> Option<String> {
    steps.iter().for_each(|step| {
        let mut count = step.count;
        while count != 0 {         
            let popped_value: char = stacks[step.from-1].pop().unwrap();
            stacks[step.to-1].push(popped_value); 
            count -= 1;
        }
    });
    let word: Vec<char>= stacks.iter().map(|letters| *letters.last().unwrap()).collect();
    let words = word.into_iter().collect();
    return Some(words);
}

pub fn part_two_solve(steps: Vec<Move>, mut stacks: Vec<Vec<char>>) -> Option<String> {
    steps.iter().for_each(|step| {
            let stack = &mut stacks[step.from-1];
            let popped_off = stack.split_off(stack.len() - step.count);     
            popped_off.iter().for_each(|value| {
                stacks[step.to-1].push(*value);
            });
    });
    let word: Vec<char>= stacks.iter().map(|letters| *letters.last().unwrap()).collect();
    let words = word.into_iter().collect();
    return Some(words);
}

pub fn part_one(input: &str) -> Option<String> {
    let (steps, stacks) = parse(input);
    let message = part_one_solve(steps, stacks);
    return Some(message.unwrap());
}

pub fn part_two(input: &str) -> Option<String> {
    let (steps, stacks) = parse(input);
    let message = part_two_solve(steps, stacks);
    return Some(message.unwrap());
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_ne!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_ne!(part_two(&input), None);
    }
}
