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

pub fn parse_two(input: &str) -> u32 {
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
    let mut rope_pieces: Vec<(i32, i32)> = vec![(0, 0); 10];
    let mut positions: HashMap<(i32, i32), String> = HashMap::new();
    positions.insert((0, 0), String::from(""));

    for (direction, distance) in directions.iter().zip(distances.iter()) {
        for _ in 0..*distance {
            rope_pieces[0] = (
                rope_pieces[0].0 + direction_map[direction].0,
                rope_pieces[0].1 + direction_map[direction].1,
            );

            for i in 1..rope_pieces.len() {
                let (tail_left, tail_right) = rope_pieces.split_at_mut(i);
                let (tail_parent, tail_right) = (tail_left[i - 1], &mut tail_right[0]);

                if tail_parent.1.abs_diff(tail_right.1) > 1
                    || tail_parent.0.abs_diff(tail_right.0) > 1
                {
                    let diff_position =
                        (tail_right.0 - tail_parent.0, tail_right.1 - tail_parent.1);
                    let larger_diff = diff_position.0.abs().max(diff_position.1.abs());
                    let add_diff = (diff_position.0 / larger_diff, diff_position.1 / larger_diff);
                    *tail_right = (tail_parent.0 + add_diff.0, tail_parent.1 + add_diff.1);
                    if i == 9 {
                        positions.insert(*tail_right, String::from(""));
                    }
                } else {
                    break;
                }
            }
        }
    }
    return positions.len() as u32;
}
pub fn part_one(input: &str) -> Option<u32> {
    Some(parse(input))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(parse_two(input))
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
        assert_eq!(part_two(&input), Some(1));
    }
}
