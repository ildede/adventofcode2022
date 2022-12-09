pub fn solve(lines: Vec<String>) -> String {
    let carried_calories: Vec<u32> = lines.into_iter()
        .fold(vec![0], |mut acc, b| {
            if b.is_empty() {
                acc.push(0);
            } else {
                let last_calories = acc.last_mut().unwrap();
                *last_calories += b.parse::<u32>().unwrap();
            }
            acc
        });
    let vec = carried_calories.iter().max().unwrap();
    String::from(format!("{}", vec))
}

#[cfg(test)]
mod tests {
    use crate::solvers::day1::solve;

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
            "10000".to_string()
        ];
        let result = solve(lines);
        assert_eq!(result, "24000");
    }
}