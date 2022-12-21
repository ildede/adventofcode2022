use crate::Part;

pub fn solve(part: Part, lines: Vec<String>) -> String {
    let mut carried_calories: Vec<u32> = lines.into_iter().fold(vec![0], |mut acc, b| {
        if b.is_empty() {
            acc.push(0);
        } else {
            let last_calories = acc.last_mut().unwrap();
            *last_calories += b.parse::<u32>().unwrap();
        }
        acc
    });
    match part {
        Part::A => {
            let result = carried_calories.iter().max().unwrap();
            String::from(format!("{}", result))
        }
        Part::B => {
            carried_calories.sort();
            carried_calories.reverse();
            let result: u32 = carried_calories[..3].iter().sum();
            String::from(format!("{}", result))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::solvers::day1::solve;
    use crate::Part;

    #[test]
    fn solve_example_part_a() {
        let lines = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        let result = solve(Part::A, lines);
        assert_eq!(result, "24000");
    }

    #[test]
    fn solve_example_part_b() {
        let lines = vec![
            "1000".to_string(),
            "2000".to_string(),
            "3000".to_string(),
            "".to_string(),
            "4000".to_string(),
            "".to_string(),
            "5000".to_string(),
            "6000".to_string(),
            "".to_string(),
            "7000".to_string(),
            "8000".to_string(),
            "9000".to_string(),
            "".to_string(),
            "10000".to_string(),
        ];
        let result = solve(Part::B, lines);
        assert_eq!(result, "45000");
    }
}
