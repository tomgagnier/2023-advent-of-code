use std::collections::HashSet;

struct Card {
    id: usize,
    winners: HashSet<usize>,
    numbers: Vec<usize>,
}

impl Card {
    fn from(input: &str) -> Self {
        let mut colon = input.split(":");
        let id = colon.next().unwrap()[5..].trim().parse::<usize>().unwrap();
        let mut bar = colon.next().unwrap().split(" | ");
        let winners = bar.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<HashSet<_>>();
        let numbers = bar.next().unwrap()
            .split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        Card { id, winners, numbers }
    }

    fn matches(&self) -> usize {
        self.numbers.iter().filter(|n| self.winners.contains(n)).count()
    }

    fn points(&self) -> usize {
        if self.matches() == 0 { 0 } else { 1 << (self.matches() - 1) }
    }
}

fn part_1(input: &str) -> usize {
    input.lines().map(|line| Card::from(line).points()).sum()
}

fn part_2(input: &str) -> usize {
    let reference: Vec<Card> = input.lines().map(|line| Card::from(line)).collect::<Vec<_>>();
    let mut todo = reference.iter().map(|card| card.id).collect::<Vec<_>>();
    let mut count = 0;
    while !todo.is_empty() {
        let start = todo.remove(0);
        count += 1;
        let card = &reference[start - 1];
        let end = start + card.matches();
        for i in start..end {
            if i >= reference.len() { break; }
            todo.push(reference.get(i).unwrap().id);
        }
    }
    count
}


mod tests {
    use super::*;

    const EXAMPLE: &str =
        "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
         Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
         Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
         Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
         Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
         Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_1_example() {
        assert_eq!(13, part_1(EXAMPLE));
    }

    #[test]
    fn test_part_1_input() {
        assert_eq!(23235, part_1(&include_str!("input.txt")))
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(30, part_2(EXAMPLE));
    }

    #[test]
    fn test_part_2_input() {
        assert_eq!(5920640, part_2(&include_str!("input.txt")))
    }
}
