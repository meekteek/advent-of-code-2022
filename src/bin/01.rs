pub fn parse_input(input: &str) -> Option<Vec<u32>> {
    let mut values = Vec::new();
    let mut total_calories = 0;
    for line in input.lines() {
        if line.is_empty() {
            values.push(total_calories);
            total_calories = 0;
        } else {
            total_calories += line.parse::<u32>().ok()?;
        }
    }
    values.sort();
    return Some(values);
}

pub fn part_one(input: &str) -> Option<u32> {
    let outputs = parse_input(input)?;
    let largest_calories = outputs.last().copied();
    return largest_calories;
}

pub fn part_two(input: &str) -> Option<u32> {
    let elves: Vec<u32> = parse_input(input)?;
    let biggest_elves = elves[elves.len() - 3..].to_vec();
    let total_calories = biggest_elves.iter().sum();
    return Some(total_calories);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        // assert not equal
        assert_ne!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_ne!(part_two(&input), None);
    }
}
