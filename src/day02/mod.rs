use std::cmp::max;

#[derive(Debug, Eq, PartialEq)]
struct Bag {
    red: u32,
    blue: u32,
    green: u32,
}

impl From<&str> for Bag {
    fn from(input: &str) -> Self {
        let mut bag = Bag { red: 0, green: 0, blue: 0 };
        for counts in input.split(", ") {
            let mut tally = counts.split(" ");
            let count = tally.next().unwrap().parse::<u32>().unwrap();
            let color = tally.next().unwrap();
            match color {
                "red" => bag.red += count,
                "blue" => bag.blue += count,
                "green" => bag.green += count,
                _ => panic!("Unknown color: {}", color),
            }
        }
        bag
    }
}

impl Bag {
    fn holds(&self, other: &Bag) -> bool {
        self.red >= other.red && self.blue >= other.blue && self.green >= other.green
    }

    fn max(&self, other: &Bag) -> Bag {
        Bag {
            red: max(self.red, other.red),
            blue: max(self.blue, other.blue),
            green: max(self.green, other.green),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Game {
    id: u32,
    rounds: Vec<Bag>,
}

impl From<&str> for Game {
    fn from(input: &str) -> Self {
        let mut game_rounds = input.split(": ");
        let id = game_rounds.next().unwrap()[5..].parse::<u32>().unwrap();
        let rounds = game_rounds.next().unwrap().split("; ")
            .map(|entry| Bag::from(entry))
            .collect::<Vec<_>>();
        Game { id, rounds }
    }
}

fn parse_games(input: &str) -> Vec<Game> {
    input.lines()
        .map(|line| Game::from(line))
        .collect::<Vec<_>>()
}

fn score(limit: &Bag, input: &&str) -> u32 {
    let score: u32 = parse_games(input)
        .iter()
        .filter(|game| game.rounds.iter().all(|bag| limit.holds(&bag)))
        .map(|game| game.id)
        .sum();
    score
}

fn power(input: &&str) -> u32 {
    parse_games(input).iter().map(|game| {

        let mut power = Bag { red: 1, blue: 1, green: 1 };
        for bag in &game.rounds {
            power = power.max(bag);
        }
        power.red * power.blue * power.green
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bag_from_str() {
        assert_eq!(Bag { red: 4, blue: 3, green: 0 }, Bag::from("3 blue, 4 red"));
    }

    #[test]
    fn test_game_from_str() {
        let expected = Game {
            id: 2,
            rounds: vec!(Bag { red: 0, green: 2, blue: 1 },
                         Bag { red: 1, green: 3, blue: 4 },
                         Bag { red: 0, green: 1, blue: 1 }),
        };

        assert_eq!(expected,
                   Game::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"));
    }

    const EXAMPLE: &'static str =
        "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
         Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
         Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
         Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
         Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

    #[test]
    fn test_part_1_example() {
        let score = score(
            &Bag { red: 12, green: 13, blue: 14 },
            &EXAMPLE,
        );
        assert_eq!(8, score);
    }

    #[test]
    fn test_part_1() {
        let score = score(&Bag { red: 12, green: 13, blue: 14 },
                          &include_str!("input.txt"));
        assert_eq!(3035, score);
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(2286, power(&EXAMPLE));
    }

    #[test]
    fn test_part_2() {
        let power = power(&include_str!("input.txt"));

        assert_eq!(66027, power);
    }
}