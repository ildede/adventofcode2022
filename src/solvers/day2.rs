use std::cmp::Ordering;
use crate::Part;

pub fn solve(part: Part, lines: Vec<String>) -> String {
    let scores: Vec<u32> = lines.into_iter()
        .map(Game::new)
        .map(Game::score)
        .collect();

    match part {
        Part::A => {
            let result: u32 = scores.iter().sum();
            String::from(format!("{}", result))
        }
        Part::B => {
            unimplemented!()
        }
    }
}

struct Game {
    opponent: Values,
    me: Values,
}

impl Game {
    fn new(line: String) -> Game {
        let mut split = line.split(' ');
        return Game {
            opponent: Values::from(split.next().unwrap()),
            me: Values::from(split.next().unwrap()),
        };
    }

    fn score(self) -> u32 {
        return if self.me == self.opponent {
            self.me.shape_value() + 3
        } else if self.me > self.opponent {
            self.me.shape_value() + 6
        } else {
            self.me.shape_value()
        }
    }
}

#[derive(Debug, PartialEq)]
enum Values {
    Rock,
    Paper,
    Scissors,
}

impl Values {
    // Opponent: A for Rock, B for Paper, and C for Scissors
    // Me: X for Rock, Y for Paper, and Z for Scissors
    fn from(s: &str) -> Values {
        match s {
            "A" => Values::Rock,
            "X" => Values::Rock,
            "B" => Values::Paper,
            "Y" => Values::Paper,
            "C" => Values::Scissors,
            "Z" => Values::Scissors,
            _ => {
                panic!("invalid value");
            }
        }
    }
    fn shape_value(self) -> u32 {
        match self {
            Values::Rock => 1,
            Values::Paper => 2,
            Values::Scissors => 3
        }
    }
}

impl PartialOrd for Values {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            Values::Rock => {
                match other {
                    Values::Rock => Some(Ordering::Equal),
                    Values::Paper => Some(Ordering::Less),
                    Values::Scissors => Some(Ordering::Greater)
                }
            }
            Values::Paper => {
                match other {
                    Values::Rock => Some(Ordering::Greater),
                    Values::Paper => Some(Ordering::Equal),
                    Values::Scissors => Some(Ordering::Less)
                }
            }
            Values::Scissors => {
                match other {
                    Values::Rock => Some(Ordering::Less),
                    Values::Paper => Some(Ordering::Greater),
                    Values::Scissors => Some(Ordering::Equal)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Part;
    use crate::solvers::day2::solve;
    use crate::solvers::day2::{Game, Values};

    #[test]
    fn solve_example_part_a() {
        let lines = vec![
            "A Y".to_string(),
            "B X".to_string(),
            "C Z".to_string(),
        ];
        let result = solve(Part::A, lines);
        assert_eq!(result, "15");
    }

    #[test]
    fn it_creates_game_of_rocks() {
        let game = Game::new(String::from("A X"));
        assert_eq!(game.opponent, Values::Rock);
        assert_eq!(game.me, Values::Rock);
    }

    #[test]
    fn it_creates_game_of_papers() {
        let game = Game::new(String::from("B Y"));
        assert_eq!(game.opponent, Values::Paper);
        assert_eq!(game.me, Values::Paper);
    }

    #[test]
    fn it_creates_game_of_scissors() {
        let game = Game::new(String::from("C Z"));
        assert_eq!(game.opponent, Values::Scissors);
        assert_eq!(game.me, Values::Scissors);
    }

    #[test]
    fn it_gives_to_each_values_proper_shape_score() {
        assert_eq!(Values::Rock.shape_value(), 1);
        assert_eq!(Values::Paper.shape_value(), 2);
        assert_eq!(Values::Scissors.shape_value(), 3);
    }

    #[test]
    fn it_gives_the_total_score_during_draws() {
        let rocks = Game { opponent: Values::Rock, me: Values::Rock };
        assert_eq!(rocks.score(), 4);
        let papers = Game { opponent: Values::Paper, me: Values::Paper };
        assert_eq!(papers.score(), 5);
        let scissors = Game { opponent: Values::Scissors, me: Values::Scissors };
        assert_eq!(scissors.score(), 6);
    }

    #[test]
    fn it_gives_the_total_score_during_wins() {
        let rocks = Game { opponent: Values::Rock, me: Values::Paper };
        assert_eq!(rocks.score(), 8);
        let papers = Game { opponent: Values::Paper, me: Values::Scissors };
        assert_eq!(papers.score(), 9);
        let scissors = Game { opponent: Values::Scissors, me: Values::Rock };
        assert_eq!(scissors.score(), 7);
    }

    #[test]
    fn it_gives_the_total_score_during_loss() {
        let rocks = Game { opponent: Values::Rock, me: Values::Scissors };
        assert_eq!(rocks.score(), 3);
        let papers = Game { opponent: Values::Paper, me: Values::Rock };
        assert_eq!(papers.score(), 1);
        let scissors = Game { opponent: Values::Scissors, me: Values::Paper };
        assert_eq!(scissors.score(), 2);
    }


}