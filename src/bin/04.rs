use std::ops::Range;

pub fn create_range(input: &str) -> Range<u32> {
    let (left, right) = input.split_once('-').unwrap();
    let start: u32 = left.parse().unwrap();
    let end: u32 = right.parse().unwrap();
    return start..end;
}
pub fn part_one_parse(input: &str) -> Option<u32> {
    let mut count = 0;
    input.split('\n').for_each(|items| {
        let (first, second) = items.split_once(',').unwrap();
        let left_range = create_range(first);
        let right_range = create_range(second);
        if left_range.start <= right_range.start && left_range.end >= right_range.end
            || left_range.start >= right_range.start && left_range.end <= right_range.end
        {
            count += 1;
        }
    });
    return Some(count);
}

pub fn part_two_parse(input: &str) -> Option<u32> {
    let mut count = 0;
    input.split('\n').for_each(|line| {
        let (left, right) = line.split_once(',').unwrap();
        let left_range = create_range(left);
        let right_range = create_range(right);
        if (left_range.start >= right_range.start && left_range.start <= right_range.end)
        || (left_range.end >= right_range.start && left_range.end <= right_range.end)
        || (right_range.start >= left_range.start && right_range.start <= left_range.end)
        || (right_range.end >= left_range.start && right_range.end <= left_range.end) {
            count += 1;
        }
    });
    return Some(count);
}

pub fn part_one(input: &str) -> Option<u32> {
    part_one_parse(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    part_two_parse(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_ne!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_ne!(part_two(&input), None);
    }
}
