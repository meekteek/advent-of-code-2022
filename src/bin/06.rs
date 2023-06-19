pub fn part_one_solve(input: &str) -> Option<u32> {
    let mut output_index = 0;
    for (index, value) in input.chars().enumerate() {
        if index >= 3 {
            let test = input.chars().nth(index-3).unwrap();
            let test2 = input.chars().nth(index-2).unwrap();
            let test3 = input.chars().nth(index-1).unwrap();
            let test4 = value;
            if !(test == test2 || test2 == test3 || test3 == test4 || test4 == test || test2 == test4 || test3 == test)
            {
                output_index = index as u32 + 1;
                break;
            }
        }
    };
    Some(output_index)
}

pub fn part_two_solve(input: &str) -> Option<u32> {
    let mut total_chars: Vec<char> = input.chars().collect();

    for i in 14..total_chars.len() {
        let mut start_of_packet = true;
        let chars = &total_chars[i-14..i];
        
        for char in chars {
            if chars.iter().filter(|c| c == &char).count() > 1 {
                start_of_packet = false;
                break;
            }
        }
        if start_of_packet {
            return Some(i as u32);
        }
    }
    return None;
}

pub fn part_one(input: &str) -> Option<u32> {
    println!("{:?}", part_one_solve(input));
    return part_one_solve(input);
}

pub fn part_two(input: &str) -> Option<u32> {
    return part_two_solve(input);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);
        println!("{:?}", part_one(&input));
        assert_ne!(part_one(&input), None);
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);
        println!("{:?}", part_two(&input));
        assert_ne!(part_two(&input), None);
    }
}
