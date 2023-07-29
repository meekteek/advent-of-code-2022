use std::collections::HashMap;

#[derive(PartialEq, Eq, Hash, Clone)]
enum Direction {
    Up,
    Down,
    Right,
    Left,
}

pub fn parse(input: &str) -> u32 {
    let mut directions: Vec<Direction> = Vec::new();
    let mut distances: Vec<i32> = Vec::new();

    let direction_map: HashMap<Direction, (i32, i32)> = [
        (Direction::Up, (0, 1)),
        (Direction::Down, (0, -1)),
        (Direction::Right, (1, 0)),
        (Direction::Left, (-1, 0)),
    ]
    .iter()
    .cloned()
    .collect();

    input.lines().for_each(|line| {
        let chars_in_line: Vec<char> = line.chars().collect();
        directions.push(match chars_in_line[0] {
            'U' => Direction::Up,
            'D' => Direction::Down,
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!(),
        });
        let num_str: String = chars_in_line[2..].iter().collect();
        distances.push(num_str.parse().unwrap());
    });
    let mut head: (i32, i32) = (0, 0);
    let mut tail: (i32, i32) = (0, 0);

    let mut positions: HashMap<(i32, i32), String> = HashMap::new();
    positions.insert((0, 0), String::from(""));

    // for each direction, move the head and tail
    for (direction, distance) in directions.iter().zip(distances.iter()) {
        // keep moving in that direction and note positions
        match *direction {
            Direction::Up => {
                head.1 += 1 * distance;
            }
            Direction::Down => {
                head.1 -= 1 * distance;
            }
            Direction::Right => {
                head.0 += 1 * distance;
            }
            Direction::Left => {
                head.0 -= 1 * distance;
            }
        }
        if head.1.abs_diff(tail.1) > 1 || head.0.abs_diff(tail.0) > 1 {
            let diff_position = (
                head.0 - direction_map[direction].0,
                head.1 - direction_map[direction].1,
            );
            while diff_position != tail {
                tail.0 += (-(tail.0 - diff_position.0)).signum();
                tail.1 += (-(tail.1 - diff_position.1)).signum();

                positions.insert(tail, String::from(""));
            }
        }
    }
    return positions.len() as u32;
}

pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_one(&input), Some(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        assert_eq!(part_two(&input), None);
    }
}
