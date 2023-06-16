use std::{collections::HashSet, hash::Hash};

pub fn convert(duplicate: char) -> u32 {
    match duplicate {
        'A'..='Z' => (duplicate as u8 - b'A' + 27) as u32,
        'a'..='z' => (duplicate as u8 - b'a' + 1) as u32,
        _ => panic!(),
    }
}

pub fn part_one_sum(input: &str) -> Option<u32> {
    let mut count = 0;
    for line in input.split("\n") {
        if line.len() > 0 {
            let (leftside, rightside) = line.split_at(line.len() / 2);
            let leftside_map: HashSet<char> = leftside.chars().collect();
            let same_char = rightside
                .chars()
                .find(|x| leftside_map.contains(x))
                .unwrap();
            count += convert(same_char);
        }
    }
    Some(count)
}

pub fn part_two_sum(input: &str) -> Option<u32> {
    let mut count = 0;
    let lines: Vec<&str> = input.split("\n").collect();
    for line in lines.chunks(3) {
        if line.len() > 0 {
            // put each chunk into a hashset to use .intersection function on each line
            let left_chunk: HashSet<char> = line[0].chars().collect();
            let middle_chunk: HashSet<char> = line[1].chars().collect();
            let mut right_chunk  = line[2].chars().clone();

            let first_matches: HashSet<_> = left_chunk.intersection(&middle_chunk).collect();           
            let same_char = right_chunk.find(|x| first_matches.contains(&x));
            count += convert(same_char.unwrap());
        }
    }
    Some(count)
}
pub fn part_one(input: &str) -> Option<u32> {
    part_one_sum(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_sum(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        println!("{:?}", part_one(&input));
        assert_eq!(part_one(&input), Some(7674));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Some(2805));
    }
}
