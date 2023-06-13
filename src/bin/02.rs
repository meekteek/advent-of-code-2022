use std::collections::HashMap;

pub fn part_one_input(input: &str) -> Option<u32> {
    let rps_tie: HashMap<char, char> = [('A', 'X'), ('B', 'Y'), ('C', 'Z')]
        .iter()
        .cloned()
        .collect();
    let rps_win: HashMap<char, char> = [('A', 'Y'), ('B', 'Z'), ('C', 'X')]
        .iter()
        .cloned()
        .collect();
    let output = input.split('\n').fold(0, |acc, line: &str| {
        let mut counter = 0;
        let mut chars = line.chars().filter(|&c| !c.is_whitespace());
        if let Some(elf_move) = chars.next() {
            if let Some(player_move) = chars.next_back() {
                if rps_tie.get(&elf_move) == Some(&player_move) {
                    counter += 3;
                } else if rps_win.get(&elf_move) == Some(&player_move) {
                    counter += 6;
                }
                match player_move {
                    'X' => counter += 1,
                    'Y' => counter += 2,
                    'Z' => counter += 3,
                    _ => {}
                };
            }
        };
        return acc + counter;
    });
    return Some(output as u32);
}

pub fn part_one(input: &str) -> Option<u32> {
    return part_one_input(input);
}

pub fn part_two_input(input: &str) -> Option<u32> {
    let rps_tie: HashMap<char, char> = [('A', 'X'), ('B', 'Y'), ('C', 'Z')]
        .iter()
        .cloned()
        .collect();
    let rps_win: HashMap<char, char> = [('A', 'Y'), ('B', 'Z'), ('C', 'X')]
        .iter()
        .cloned()
        .collect();
    let output = input.split('\n').fold(0, |acc, line: &str| {
        let mut counter = 0;
        let mut chars = line.chars();

        if let Some(elf_move) = chars.next() {
            if let Some(player_direction) = chars.next_back() {
                let player_move = match elf_move {
                    'A' => match player_direction {
                        'X' => 'Z',
                        'Y' => 'X',
                        'Z' => 'Y',
                        _ => unreachable!(),
                    },
                    'B' => match player_direction {
                        'X' => 'X',
                        'Y' => 'Y',
                        'Z' => 'Z',
                        _ => unreachable!(),
                    },
                    'C' => match player_direction {
                        'X' => 'Y',
                        'Y' => 'Z',
                        'Z' => 'X',
                        _ => unreachable!(),
                    },
                    _ => panic!("Invalid shape: {}", elf_move),
                };
                if rps_tie.get(&elf_move) == Some(&player_move) {
                    counter += 3;
                } else if rps_win.get(&elf_move) == Some(&player_move) {
                    counter += 6;
                }
                match player_move {
                    'X' => counter += 1,
                    'Y' => counter += 2,
                    'Z' => counter += 3,
                    _ => {}
                };
            }
        };
        return acc + counter;
    });
    return Some(output as u32);
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_input(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 2);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_one(&input), Some(12645));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 2);
        assert_eq!(part_two(&input), Some(11756));
    }
}
