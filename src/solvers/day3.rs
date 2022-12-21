use crate::Part;
use std::collections::HashSet;

pub fn solve(part: Part, lines: Vec<String>) -> String {
    let scores: Vec<u32> = match part {
        Part::A => lines
            .into_iter()
            .map(split_in_compartments)
            .map(found_common_element)
            .map(convert_to_priority)
            .collect(),
        Part::B => found_badges(lines)
            .into_iter()
            .map(convert_to_priority)
            .collect(),
    };

    let result: u32 = scores.iter().sum();
    String::from(format!("{}", result))
}

fn found_badges(rucksacks: Vec<String>) -> Vec<char> {
    let mut badges: Vec<char> = Vec::new();
    let mut current_badge: Vec<char> = Vec::new();
    for rucksack in rucksacks {
        if current_badge.is_empty() {
            rucksack.chars().for_each(|e| {
                if !current_badge.contains(&e) {
                    current_badge.push(e);
                }
            });
        } else {
            let prev_elements = current_badge.clone();
            current_badge = Vec::new();
            rucksack
                .chars()
                .filter(|e| prev_elements.contains(e))
                .for_each(|e| {
                    if !current_badge.contains(&e) {
                        current_badge.push(e);
                    }
                })
        }
        if current_badge.len() == 1 {
            badges.push(*current_badge.first().unwrap());
            current_badge = Vec::new();
        }
    }
    badges
}

fn split_in_compartments(rucksacks_content: String) -> (String, String) {
    let split_index = rucksacks_content.len() / 2;
    let x = rucksacks_content.split_at(split_index);
    (x.0.to_string(), x.1.to_string())
}

fn found_common_element(compartment_elements: (String, String)) -> char {
    let set: HashSet<char> = compartment_elements.0.chars().collect();
    let commons: Vec<char> = compartment_elements
        .1
        .chars()
        .filter(|c| set.contains(c))
        .collect();
    commons.first().unwrap().clone()
}

fn convert_to_priority(element: char) -> u32 {
    let i = element as u32;
    if i >= 97 {
        i - 96
    } else {
        i - 38
    }
}

#[cfg(test)]
mod tests {
    use crate::solvers::day3::{found_badges, found_common_element, solve, split_in_compartments};
    use crate::Part;

    #[test]
    fn solve_example_part_a() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        let result = solve(Part::A, lines);
        assert_eq!(result, "157");
    }

    #[test]
    fn solve_example_part_b() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        let result = solve(Part::B, lines);
        assert_eq!(result, "70");
    }

    #[test]
    fn it_split_a_string_in_half() {
        let result = split_in_compartments(String::from("AAAABBBB"));

        assert_eq!(result, ("AAAA".to_string(), "BBBB".to_string()));
    }

    #[test]
    fn it_found_common_element() {
        let result = found_common_element(("AxAA".to_string(), "BBBx".to_string()));

        assert_eq!(result, 'x');
    }

    #[test]
    fn it_found_badges_in_single_group() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
        ];
        let result = found_badges(lines);
        assert_eq!(result, vec!['r']);
    }

    #[test]
    fn it_found_badges() {
        let lines = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp".to_string(),
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string(),
            "PmmdzqPrVvPwwTWBwg".to_string(),
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string(),
            "ttgJtRGJQctTZtZT".to_string(),
            "CrZsJsPPZsGzwwsLwLmpwMDw".to_string(),
        ];
        let result = found_badges(lines);
        assert_eq!(result, vec!['r', 'Z']);
    }
}
