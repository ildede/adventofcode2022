use std::collections::HashSet;
use std::hash::Hash;
use crate::Part;

pub fn solve(part: Part, line: String) -> String {
    let result = match part {
        Part::A => find_marker(line),
        Part::B => unimplemented!()
    };
    format!("{}", result)
}

fn find_marker(input: String) -> usize {
    for i in 4..input.len() {
        let mark_1 = input.chars().nth(i - 4);
        let mark_2 = input.chars().nth(i - 3);
        let mark_3 = input.chars().nth(i - 2);
        let mark_4 = input.chars().nth(i - 1);
        if has_unique_elements(vec![mark_1, mark_2, mark_3, mark_4]) {
            return i
        }
    }
    0
}

fn has_unique_elements<T>(iter: T) -> bool
    where
        T: IntoIterator,
        T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

#[cfg(test)]
mod tests {
    use crate::Part;
    use crate::solvers::day6::{solve, find_marker};

    #[test]
    fn solve_examples_part_a() {
        assert_eq!(solve(Part::A, "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), "5");
        assert_eq!(solve(Part::A, "nppdvjthqldpwncqszvftbrmjlhg".to_string()), "6");
        assert_eq!(solve(Part::A, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), "10");
        assert_eq!(solve(Part::A, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), "11");
    }

    #[test]
    fn test_find_marker_with_example_1() {
        assert_eq!(find_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()), 5);
    }

    #[test]
    fn test_find_marker_with_example_2() {
        assert_eq!(find_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string()), 6);
    }

    #[test]
    fn test_find_marker_with_example_3() {
        assert_eq!(find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()), 10);
    }

    #[test]
    fn test_find_marker_with_example_4() {
        assert_eq!(find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()), 11);
    }

}