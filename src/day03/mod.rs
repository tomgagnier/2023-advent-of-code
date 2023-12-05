use std::ops::Range;
use regex::Regex;

#[derive(Debug, Eq, PartialEq)]
struct Position {
    i: i32,
    j: i32,
}

impl Position {
    pub fn adjacent_to(&self, other: &Position) -> bool {
        (self.i - other.i).abs() <= 1 && (self.j - other.j).abs() <= 1
    }
}


#[derive(Debug, Eq, PartialEq)]
struct Symbol {
    symbol: char,
    pos: Position,
}

impl Symbol {}

fn symbols_in(input: &str) -> Vec<Symbol> {
    let mut symbols = Vec::new();
    for (j, line) in input.lines().rev().enumerate() {
        for (i, c) in line.chars().enumerate() {
            if c != '.' && !c.is_digit(10) {
                let i1 = i as i32;
                let j1 = j as i32;
                symbols.push(Symbol { symbol: c, pos: Position { i: i1, j: j1 } });
            }
        }
    }
    symbols
}

#[derive(Debug, Eq, PartialEq)]
struct Number {
    value: u64,
    i: Range<usize>,
    j: usize,
}

fn numbers_in(input: &str) -> Vec<Number> {
    let regex = Regex::new(r"([0-9]+)").unwrap();
    let mut numbers = Vec::new();
    for (j, line) in input.lines().rev().enumerate() {
        for matcher in regex.find_iter(line) {
            let range = matcher.range();
            let value = matcher.as_str().parse::<u64>().unwrap();
            numbers.push(Number { i: range, j, value });
        }
    }
    numbers
}

impl Number {
    fn adjacent_to(&self, pos: &Position) -> bool {
        self.i.clone().any(|i| {
            pos.adjacent_to(&Position { i: i as i32, j: self.j as i32 })
        })
    }
}

fn part_1(input: &str) -> u64 {
    let symbols = symbols_in(input);
    numbers_in(input)
        .iter()
        .filter(|number| symbols.iter()
            .any(|s| number.adjacent_to(&s.pos)))
        .map(|number| number.value)
        .sum()
}

fn part_2(input: &str) -> u64 {
    let numbers = numbers_in(input);
    symbols_in(input).iter()
        .map(|s| numbers.iter()
            .filter(|n| n.adjacent_to(&s.pos))
            .map(|n| n.value)
            .collect::<Vec<_>>())
        .filter(|values| values.len() == 2)
        .map(|numbers| numbers.iter().product::<u64>())
        .sum()
}


mod tests {
    use super::*;

    const EXAMPLE: &str =
        "467..114..\n\
         ...*......\n\
         ..35..633.\n\
         ......#...\n\
         617*......\n\
         .....+.58.\n\
         ..592.....\n\
         ......755.\n\
         ...$.*....\n\
         .664.598..";

    #[test]
    fn test_symbols_in() {
        assert_eq!(
            vec!(
                Symbol { symbol: '$', pos: Position { i: 3, j: 1 } },
                Symbol { symbol: '*', pos: Position { i: 5, j: 1 } },
                Symbol { symbol: '+', pos: Position { i: 5, j: 4 } },
                Symbol { symbol: '*', pos: Position { i: 3, j: 5 } },
                Symbol { symbol: '#', pos: Position { i: 6, j: 6 } },
                Symbol { symbol: '*', pos: Position { i: 3, j: 8 } },
            ),
            symbols_in(EXAMPLE)
        );
    }

    #[test]
    fn test_numbers_in() {
        assert_eq!(
            vec!(
                Number { value: 467, i: 0..3, j: 0 },
                Number { value: 114, i: 5..8, j: 0 },
                Number { value: 35, i: 2..4, j: 2 },
                Number { value: 633, i: 6..9, j: 2 },
                Number { value: 617, i: 0..3, j: 4 },
                Number { value: 58, i: 7..9, j: 5 },
                Number { value: 592, i: 2..5, j: 6 },
                Number { value: 755, i: 6..9, j: 7 },
                Number { value: 664, i: 1..4, j: 9 },
                Number { value: 598, i: 5..8, j: 9 }
            ),
            numbers_in(EXAMPLE)
        )
    }

    #[test]
    fn test_adjacent_to_457() {
        let _467 = Number { i: 0..3, j: 0, value: 467 };
        assert_eq!(true, _467.adjacent_to(&Position{ i: 3, j: 1}));
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(4361, part_1(EXAMPLE));
    }

    #[test]
    fn test_part_1_input() {
        // 414782 too low
        print!("Part 1: {} ", part_1(&include_str!("input.txt")))
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(467835, part_2(EXAMPLE));
    }

    #[test]
    fn test_part_2_input() {
        print!("Part 2: {} ", part_2(&include_str!("input.txt")))
    }

}