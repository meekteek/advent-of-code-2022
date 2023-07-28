use std::ops::Range;
struct Grid {
    plot: Vec<Vec<usize>>,
}

impl Grid {
    pub fn new(plot: Vec<Vec<usize>>) -> Grid {
        Grid { plot }
    }
    pub fn test_row(&self, range: Range<usize>, row_index: usize, tree_size: usize) -> bool {
        let mut visible = false;
        let mut largest_tree_size_in_row: usize = 0;

        range.for_each(|index| {
            largest_tree_size_in_row = largest_tree_size_in_row.max(self.plot[row_index][index]);
        });
        if largest_tree_size_in_row < tree_size {
            visible = true;
        }
        return visible;
    }

    pub fn test_column(&self, range: Range<usize>, column_index: usize, tree_size: usize) -> bool {
        let mut visible = false;
        let mut largest_tree_size_in_row: usize = 0;

        range.for_each(|index| {
            largest_tree_size_in_row = largest_tree_size_in_row.max(self.plot[index][column_index]);
        });
        if largest_tree_size_in_row < tree_size {
            visible = true;
        }
        return visible;
    }

    pub fn get_row_count(&self, range: Range<usize>, row_index: usize, tree_size: usize) -> usize {
        let range: Box<dyn Iterator<Item = usize>> = if range.start == 0 {
            Box::new(range.rev())
        } else {
            Box::new(range)
        };
        let mut trees_seen: usize = 0;
        for index in range {
            if tree_size <= self.plot[row_index][index] {
                return trees_seen + 1;
            }
            trees_seen += 1;
        }
        return trees_seen;
    }
    pub fn get_column_count(
        &self,
        range: Range<usize>,
        column_index: usize,
        tree_size: usize,
    ) -> usize {
        let new_range: Box<dyn Iterator<Item = usize>> = if range.start == 0 {
            Box::new(range.rev())
        } else {
            Box::new(range)
        };
        let mut trees_seen: usize = 0;
        for index in new_range {
            if tree_size <= self.plot[index][column_index] {
                return trees_seen + 1;
            }
            trees_seen += 1;
        }
        return trees_seen;
    }
}

pub fn parse_part_one(input: &str) -> Option<u32> {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let plot_of_trees = Grid::new(map.clone());
    let mut visible_trees: u32 = (map.len() * 2 + (map[0].len() - 2) * 2) as u32;

    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            let mut visible = false;
            let tree_size = map[i][j] as usize;

            // for each position, check if visible from row or column
            let left_range = 0..j;
            let right_range = j + 1..map[i].len();
            let top_range = 0..i;
            let bottom_range = i + 1..map.len();
            visible = plot_of_trees.test_row(left_range, i, tree_size)
                || plot_of_trees.test_row(right_range, i, tree_size)
                || plot_of_trees.test_column(top_range, j, tree_size)
                || plot_of_trees.test_column(bottom_range, j, tree_size);

            if visible {
                visible_trees += 1;
            }
        }
    }
    return Some(visible_trees);
}

pub fn parse_part_two(input: &str) -> Option<u32> {
    let map: Vec<Vec<usize>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();
    let plot_of_trees = Grid::new(map.clone());

    let mut max_trees = 1;
    for i in 1..map.len() - 1 {
        for j in 1..map[i].len() - 1 {
            let tree_size = map[i][j] as usize;
            let mut trees_count = 1;
            // check for trees seen
            let left_range = 0..j;
            let right_range = j + 1..map[i].len();
            let top_range = 0..i;
            let bottom_range = i + 1..map.len();
            trees_count *= plot_of_trees.get_row_count(left_range, i, tree_size)
                * plot_of_trees.get_row_count(right_range, i, tree_size)
                * plot_of_trees.get_column_count(top_range, j, tree_size)
                * plot_of_trees.get_column_count(bottom_range, j, tree_size);
            max_trees = max_trees.max(trees_count);
        }
    }
    return Some(max_trees as u32);
}

pub fn part_one(input: &str) -> Option<u32> {
    parse_part_one(input)
}

pub fn part_two(input: &str) -> Option<u32> {
    parse_part_two(input)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        println!("{}", part_one(&input).unwrap());
        assert_eq!(part_one(&input), Some(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        println!("{}", part_two(&input).unwrap());
        assert_eq!(part_two(&input), Some(8));
    }
}
