use crate::Part;

pub fn solve(part: Part, stacks: Vec<Vec<char>>, lines: Vec<String>) -> String {
    let instructions: Vec<Instruction> = lines
        .into_iter()
        .filter(|l| l.starts_with("move"))
        .map(Instruction::from)
        .collect();

    let result = match part {
        Part::A => apply_instruction_a(stacks, instructions),
        Part::B => apply_instruction_b(stacks, instructions)
    };
    String::from(get_last_elements(result))
}

#[derive(PartialEq, Debug)]
struct Instruction {
    quantity: u32,
    from: usize,
    to: usize,
}

impl Instruction {
    fn from(line: String) -> Instruction {
        let mut splited = line.split(' ');
        let _ = splited.next();
        let quantity = splited.next();
        let _ = splited.next();
        let from = splited.next();
        let _ = splited.next();
        let to = splited.next();

        Instruction {
            quantity: quantity.unwrap().parse::<u32>().unwrap(),
            from: from.unwrap().parse::<usize>().unwrap(),
            to: to.unwrap().parse::<usize>().unwrap(),
        }
    }
}

fn apply_instruction_a(stack: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Vec<Vec<char>> {
    let mut cur_stack = stack.clone();
    for instruction in instructions {
        for _ in 0..instruction.quantity {
            let x = cur_stack[instruction.from-1].pop();
            match x {
                None => {}
                Some(el) => cur_stack[instruction.to-1].push(el)
            }

        }
    }
    cur_stack
}

fn apply_instruction_b(stack: Vec<Vec<char>>, instructions: Vec<Instruction>) -> Vec<Vec<char>> {
    let mut cur_stack = stack.clone();
    for instruction in instructions {
        let new_len = cur_stack[instruction.from-1].len() - instruction.quantity as usize;
        let x = cur_stack[instruction.from-1].split_off(new_len);
        for removed_element in x {
            cur_stack[instruction.to-1].push(removed_element);
        }
    }
    cur_stack
}

fn get_last_elements(stack: Vec<Vec<char>>) -> String{
    let x: Vec<char> = stack.into_iter()
        .map(|e| e.clone().pop().unwrap())
        .collect();
    let ex = x.into_iter().fold(String::new(), |acc: String, el| {
        format!("{}{}", acc, el)
    });
    ex
}

#[cfg(test)]
mod tests {
    use crate::Part;
    use crate::solvers::day5::{apply_instruction_a, apply_instruction_b, get_last_elements, Instruction, solve};

    #[test]
    fn solve_example_part_a() {
        let stacks = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let instructions = vec![
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        let result = solve(Part::A, stacks, instructions);
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn solve_example_part_b() {
        let stacks = vec![
            vec!['Z', 'N'],
            vec!['M', 'C', 'D'],
            vec!['P'],
        ];
        let instructions = vec![
            "move 1 from 2 to 1".to_string(),
            "move 3 from 1 to 3".to_string(),
            "move 2 from 2 to 1".to_string(),
            "move 1 from 1 to 2".to_string(),
        ];
        let result = solve(Part::B, stacks, instructions);
        assert_eq!(result, "MCD");
    }

    #[test]
    fn it_translate_line_to_instruction() {
        let instruction = Instruction::from("move 1 from 2 to 3".to_string());

        assert_eq!(
            instruction,
            Instruction {
                quantity: 1,
                from: 2,
                to: 3,
            }
        )
    }

    #[test]
    fn it_apply_instruction_a() {
        let initial_stacks = vec![
            vec!['Z', 'N'],
            vec!['N', 'C', 'D'],
            vec!['P'],
        ];

        let result = apply_instruction_a(
            initial_stacks,
            vec![Instruction {
                quantity: 1,
                from: 2,
                to: 3,
            }]);

        assert_eq!(result, vec![
            vec!['Z', 'N'],
            vec!['N', 'C'],
            vec!['P', 'D'],
        ])
    }

    #[test]
    fn it_apply_instructions_a() {
        let initial_stacks = vec![
            vec!['Z', 'N'],
            vec!['N', 'C', 'D'],
            vec!['P'],
        ];

        let result = apply_instruction_a(
            initial_stacks,
            vec![
                Instruction { quantity: 1, from: 2, to: 3 },
                Instruction { quantity: 2, from: 3, to: 1 },
                Instruction { quantity: 3, from: 1, to: 2 }
            ]);

        assert_eq!(result, vec![
            vec!['Z'],
            vec!['N', 'C', 'P', 'D', 'N'],
            vec![],
        ])
    }

    #[test]
    fn it_apply_instruction_b() {
        let initial_stacks = vec![
            vec!['Z', 'N'],
            vec!['N', 'C', 'D'],
            vec!['P'],
        ];

        let result = apply_instruction_b(
            initial_stacks,
            vec![Instruction {
                quantity: 1,
                from: 2,
                to: 3,
            }]);

        assert_eq!(result, vec![
            vec!['Z', 'N'],
            vec!['N', 'C'],
            vec!['P', 'D'],
        ])
    }

    #[test]
    fn it_apply_instructions_b() {
        let initial_stacks = vec![
            vec!['Z', 'N'],
            vec!['N', 'C', 'D'],
            vec!['P'],
        ];

        let result = apply_instruction_b(
            initial_stacks,
            vec![
                Instruction { quantity: 1, from: 2, to: 3 },
                Instruction { quantity: 2, from: 3, to: 1 },
                Instruction { quantity: 3, from: 1, to: 2 }
            ]);

        assert_eq!(result, vec![
            vec!['Z'],
            vec!['N', 'C', 'N', 'P', 'D'],
            vec![],
        ])
    }

    #[test]
    fn it_get_last_elements() {
        let stacks = vec![
            vec!['Z', 'N'],
            vec!['N', 'C', 'D'],
            vec!['P'],
        ];

        get_last_elements(stacks);
    }
}