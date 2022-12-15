use crate::Part;

pub fn solve(part: Part, lines: Vec<String>) -> String {
    let result = lines.into_iter()
        .map(split_to_pair)
        .map(|p| (expand_to_sections(p.0), expand_to_sections(p.1)))
        .map(|p| match part {
            Part::A => have_complete_overlap(p),
            Part::B => have_partial_overlap(p)
        })
        .filter(|&e| e == true)
        .count()
        .to_string();

    String::from(format!("{}", result))
}

fn split_to_pair(line: String) -> (String, String) {
    match line.split_once(',') {
        None => panic!("invalid value"),
        Some(el) => (el.0.to_string(), el.1.to_string())
    }
}

fn expand_to_sections(assignment: String) -> Vec<u32> {
    if let Some((min, max)) = assignment.split_once('-') {
        let start: u32 = min.parse::<u32>().unwrap();
        let end: u32 = max.parse::<u32>().unwrap() + 1;
        let mut vec = Vec::new();
        for i in start..end {
            vec.push(i);
        }
        return vec;
    } else {
        return vec![];
    }
}

fn have_complete_overlap(pair_assignment: (Vec<u32>, Vec<u32>)) -> bool {
    (pair_assignment.0.first().unwrap() <= pair_assignment.1.first().unwrap() && pair_assignment.0.last().unwrap() >= pair_assignment.1.last().unwrap())
        ||
        (pair_assignment.0.first().unwrap() >= pair_assignment.1.first().unwrap() && pair_assignment.0.last().unwrap() <= pair_assignment.1.last().unwrap())
}

fn have_partial_overlap(pair_assignment: (Vec<u32>, Vec<u32>)) -> bool {
    pair_assignment.0.first().unwrap() <= pair_assignment.1.last().unwrap() && pair_assignment.0.last().unwrap() >= pair_assignment.1.first().unwrap()
}

#[cfg(test)]
mod tests {
    use crate::Part;
    use crate::solvers::day4::{solve, split_to_pair, expand_to_sections, have_complete_overlap, have_partial_overlap};

    #[test]
    fn solve_example_part_a() {
        let lines = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        let result = solve(Part::A, lines);
        assert_eq!(result, "2");
    }

    #[test]
    fn solve_example_part_b() {
        let lines = vec![
            "2-4,6-8".to_string(),
            "2-3,4-5".to_string(),
            "5-7,7-9".to_string(),
            "2-8,3-7".to_string(),
            "6-6,4-6".to_string(),
            "2-6,4-8".to_string(),
        ];
        let result = solve(Part::B, lines);
        assert_eq!(result, "4");
    }

    #[test]
    fn it_split_to_pair_assignment() {
        let result = split_to_pair("2-4,6-8".to_string());

        assert_eq!(result, ("2-4".to_string(), "6-8".to_string()));
    }

    #[test]
    fn it_expand_assignment_to_sections() {
        let result = expand_to_sections("2-4".to_string());

        assert_eq!(result, vec![2, 3, 4]);
    }

    #[test]
    fn it_returns_if_two_ranges_have_a_complete_overlap() {
        assert_eq!(have_complete_overlap((vec![2], vec![2])), true);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![2])), true);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![3])), true);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![4])), true);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![2, 3])), true);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![3, 4])), true);
        assert_eq!(have_complete_overlap((vec![2], vec![2, 3, 4])), true);
        assert_eq!(have_complete_overlap((vec![3], vec![2, 3, 4])), true);
        assert_eq!(have_complete_overlap((vec![4], vec![2, 3, 4])), true);
        assert_eq!(have_complete_overlap((vec![2, 3], vec![2, 3, 4])), true);
        assert_eq!(have_complete_overlap((vec![3, 4], vec![2, 3, 4])), true);

        assert_eq!(have_complete_overlap((vec![2], vec![3])), false);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![1, 2])), false);
        assert_eq!(have_complete_overlap((vec![2, 3, 4], vec![4, 5])), false);
    }

    #[test]
    fn it_returns_if_two_ranges_have_a_partial_overlap() {
        assert_eq!(have_partial_overlap((vec![2], vec![2])), true);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![2])), true);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![3])), true);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![4])), true);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![2, 3])), true);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![3, 4])), true);
        assert_eq!(have_partial_overlap((vec![2], vec![2, 3, 4])), true);
        assert_eq!(have_partial_overlap((vec![3], vec![2, 3, 4])), true);
        assert_eq!(have_partial_overlap((vec![4], vec![2, 3, 4])), true);
        assert_eq!(have_partial_overlap((vec![2, 3], vec![2, 3, 4])), true);
        assert_eq!(have_partial_overlap((vec![3, 4], vec![2, 3, 4])), true);

        assert_eq!(have_partial_overlap((vec![2], vec![3])), false);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![1, 2])), true);
        assert_eq!(have_partial_overlap((vec![2, 3, 4], vec![4, 5])), true);
    }
}