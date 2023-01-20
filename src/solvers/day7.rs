use crate::Part;
use std::collections::HashMap;
use std::path::PathBuf;

pub fn solve(part: Part, lines: Vec<String>) -> String {
    let result = match part {
        Part::A => {
            let dirs = get_dirs(lines);
            dirs.into_values()
                .filter(|size| *size <= 100_000)
                .sum::<u32>()
        }
        Part::B => {
            let dirs = get_dirs(lines);
            let disk = 70_000_000;
            let needed = 30_000_000;
            let root = dirs.get(&PathBuf::from("/")).unwrap();
            let available = disk - root;

            dirs.into_values()
                .filter(|size| available + size >= needed)
                .min()
                .unwrap()
        }
    };
    format!("{}", result)
}

fn get_dirs(lines: Vec<String>) -> HashMap<PathBuf, u32> {
    let mut sizes = HashMap::new();
    let mut affected = Vec::new();

    for line in lines {
        if line.starts_with("$ ls") || line.starts_with("dir") {
            continue;
        }

        let parts: Vec<_> = line.split_whitespace().collect();
        match parts[..] {
            ["$", "cd", ".."] => {
                affected.pop();
            }
            ["$", "cd", name] => {
                affected.push(String::from(name));
            }
            [size, _name] => {
                let size: u32 = size.parse().unwrap();
                for idx in 0..affected.len() {
                    let path = PathBuf::from_iter(&affected[..=idx]);
                    *sizes.entry(path).or_insert(0) += size;
                }
            }
            _ => {}
        };
    }
    sizes
}

#[cfg(test)]
mod tests {
    use crate::solvers::day7::{get_dirs, solve};
    use crate::Part;
    use std::collections::HashMap;
    use std::iter::Map;
    use std::path::PathBuf;
    use std::vec::Drain;

    #[test]
    fn solve_examples_part_a() {
        let lines = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "14848514 b.txt".to_string(),
            "8504156 c.dat".to_string(),
            "dir d".to_string(),
            "$ cd a".to_string(),
            "$ ls".to_string(),
            "dir e".to_string(),
            "29116 f".to_string(),
            "2557 g".to_string(),
            "62596 h.lst".to_string(),
            "$ cd e".to_string(),
            "$ ls".to_string(),
            "584 i".to_string(),
            "$ cd ..".to_string(),
            "$ cd ..".to_string(),
            "$ cd d".to_string(),
            "$ ls".to_string(),
            "4060174 j".to_string(),
            "8033020 d.log".to_string(),
            "5626152 d.ext".to_string(),
            "7214296 k".to_string(),
        ];
        let result = solve(Part::A, lines);
        assert_eq!(result, "95437");
    }

    #[test]
    fn it_translate_lines_to_directories_with_size() {
        let lines = vec![
            "$ cd /".to_string(),
            "$ ls".to_string(),
            "dir a".to_string(),
            "100 b.txt".to_string(),
            "$ cd a".to_string(),
            "100 c.txt".to_string(),
        ];
        let dirs = get_dirs(lines);
        let mut expected = HashMap::new();
        expected.insert(PathBuf::from("/"), 200);
        expected.insert(PathBuf::from("/a"), 100);

        assert_eq!(dirs, expected);
    }
}
