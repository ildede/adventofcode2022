use crate::Part;
use std::collections::HashSet;
use std::hash::Hash;

pub fn solve(part: Part, line: String) -> String {
    let result = match part {
        Part::A => find_marker(line),
        Part::B => find_generic_marker(line, 14),
    };
    format!("{}", result)
}

fn find_marker(input: String) -> usize {
    find_generic_marker(input, 4)
}

fn find_generic_marker(input: String, packet_length: usize) -> usize {
    for i in packet_length..input.len() {
        let mut mark = Vec::new();
        for j in (1..packet_length + 1).rev() {
            mark.push(input.chars().nth(i - j));
        }
        if has_unique_elements(mark) {
            return i;
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
    use crate::solvers::day6::{find_generic_marker, find_marker, solve};
    use crate::Part;

    #[test]
    fn solve_examples_part_a() {
        assert_eq!(
            solve(Part::A, "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            "5"
        );
        assert_eq!(
            solve(Part::A, "nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            "6"
        );
        assert_eq!(
            solve(Part::A, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            "10"
        );
        assert_eq!(
            solve(Part::A, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            "11"
        );
    }

    #[test]
    fn solve_examples_part_b() {
        assert_eq!(
            solve(Part::B, "mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string()),
            "19"
        );
        assert_eq!(
            solve(Part::B, "bvwbjplbgvbhsrlpgdmjqwftvncz".to_string()),
            "23"
        );
        assert_eq!(
            solve(Part::B, "nppdvjthqldpwncqszvftbrmjlhg".to_string()),
            "23"
        );
        assert_eq!(
            solve(Part::B, "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            "29"
        );
        assert_eq!(
            solve(Part::B, "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            "26"
        );
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
        assert_eq!(
            find_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string()),
            10
        );
    }

    #[test]
    fn test_find_marker_with_example_4() {
        assert_eq!(
            find_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string()),
            11
        );
    }

    #[test]
    fn test_find_generic_marker_with_example_1() {
        assert_eq!(
            find_generic_marker("mjqjpqmgbljsphdztnvjfqwrcgsmlb".to_string(), 14),
            19
        );
    }

    #[test]
    fn test_find_generic_marker_with_example_2() {
        assert_eq!(
            find_generic_marker("bvwbjplbgvbhsrlpgdmjqwftvncz".to_string(), 14),
            23
        );
    }

    #[test]
    fn test_find_generic_marker_with_example_3() {
        assert_eq!(
            find_generic_marker("nppdvjthqldpwncqszvftbrmjlhg".to_string(), 14),
            23
        );
    }

    #[test]
    fn test_find_generic_marker_with_example_4() {
        assert_eq!(
            find_generic_marker("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg".to_string(), 14),
            29
        );
    }

    #[test]
    fn test_find_generic_marker_with_example_5() {
        assert_eq!(
            find_generic_marker("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw".to_string(), 14),
            26
        );
    }
}
