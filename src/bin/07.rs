use std::{cell::RefCell, collections::HashMap, rc::Rc};

// note: had first created file struct and only during get_total would add to size of struct
//       but was extra overhead not necessary for solving problem. Might reintroduce for cleanliness later

const DIRECTORY_CAPACITY: usize = 100000;
const TOTAL_SPACE: usize = 70000000;
const REQUIRED_SPACE: usize = 30000000;

struct BaseDirectory {
    root: Rc<Directory>,
}

impl BaseDirectory {
    pub fn new() -> Self {
        return BaseDirectory {
            root: Rc::new(Directory {
                name: String::from("root"),
                size: 0.into(),
                parent: None,
                subdirs: RefCell::new(HashMap::new()),
            }),
        };
    }
    pub fn parse_input(&self, input: &str) {
        let mut current_directory = self.root.clone();
        for line in input.lines().skip(1) {
            let args = line.split(" ").collect::<Vec<&str>>();
            if line.contains("$ cd ..") {
                current_directory = Rc::clone(&current_directory.parent.as_ref().unwrap());
            } else if line.contains("$ cd") {
                let new_directory = current_directory
                    .subdirs
                    .borrow()
                    .get(args[2])
                    .unwrap()
                    .clone();
                current_directory = new_directory;
            } else if line.contains("$ ls") {
                continue;
            } else {
                if args[0] == "dir" {
                    current_directory.subdirs.borrow_mut().insert(
                        String::from(args[1]),
                        Rc::new(Directory {
                            name: String::from(args[1]),
                            size: 0.into(),
                            parent: Some(Rc::clone(&current_directory)),
                            subdirs: RefCell::new(HashMap::new()),
                        }),
                    );
                } else {
                    *current_directory.size.borrow_mut() += args[0].parse::<usize>().unwrap();
                }
            }
        }
    }
}

#[derive(Clone, Default)]
struct Directory {
    name: String,
    size: RefCell<usize>,
    parent: Option<Rc<Directory>>,
    subdirs: RefCell<HashMap<String, Rc<Directory>>>,
}
impl Directory {
    pub fn get_total(&self) -> usize {
        return *self.size.borrow()
            + self
                .subdirs
                .borrow()
                .values()
                .fold(0, |acc, subdir| acc + subdir.get_total());
    }
}

pub fn part_one(input: &str) -> Option<u32> {
    let base_directory = BaseDirectory::new();
    base_directory.parse_input(input);
    let mut directories = vec![Rc::clone(&base_directory.root)];
    let mut total = 0;
    while let Some(dir) = directories.pop() {
        for subdir in dir.subdirs.borrow().values() {
            directories.push(subdir.clone());
        }
        let size = dir.get_total();
        if size <= DIRECTORY_CAPACITY as usize {
            total += size;
        }
    }
    return Some(total as u32);
}

pub fn smallest_dir_required(total: usize) -> usize {
    let free_space = TOTAL_SPACE - total;
    return REQUIRED_SPACE - free_space;
}
pub fn part_two(input: &str) -> Option<u32> {
    let base_directory = BaseDirectory::new();
    base_directory.parse_input(input);
    let required_space = smallest_dir_required(base_directory.root.get_total());

    let mut directories = vec![Rc::clone(&base_directory.root)];

    let mut smallest_required = usize::MAX;
    while let Some(dir) = directories.pop() {
        for subdir in dir.subdirs.borrow().values() {
            directories.push(subdir.clone());
        }
        let size = dir.get_total();
        if size >= required_space {
            smallest_required = smallest_required.min(size);
        }
    }
    return Some(smallest_required as u32);
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 7);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 7);
        println!("{:?}", part_one(&input));
        assert_eq!(part_one(&input), Some(95437));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 7);
        println!("{:?}", part_two(&input));
        assert_eq!(part_two(&input), Some(24933642));
    }
}
