fn first_number(numbers: &[(&str, u32)], input: &str) -> u32 {
    numbers
        .iter()
        .filter(|(s, _)| input.find(s).is_some())
        .min_by(|(s1, _), (s2, _)| input.find(s1).cmp(&input.find(s2)))
        .unwrap()
        .1
}

fn last_number(numbers: &[(&str, u32)], input: &str) -> u32 {
    numbers
        .iter()
        .filter(|(s, _)| input.rfind(s).is_some())
        .max_by(|(s1, _), (s2, _)| input.rfind(s1).cmp(&input.rfind(s2)))
        .unwrap()
        .1
}

fn sum_of_first_last(numbers: &[(&str, u32)], input: &str) -> u32 {
    input
        .lines()
        .map(|line| first_number(numbers, line) * 10 + last_number(numbers, line))
        .sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    const NUMBERS1: [(&str, u32); 10] = [("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9)];

    #[test]
    fn test_first_number() {
        assert_eq!(1, first_number(&NUMBERS1, "1abc2"));
        assert_eq!(3, first_number(&NUMBERS1, "pqr3stu8vwx"));
        assert_eq!(1, first_number(&NUMBERS1, "a1b2c3d4e5f"));
        assert_eq!(7, first_number(&NUMBERS1, "treb7uchet"));
    }

    #[test]
    fn test_last_number() {
        assert_eq!(2, last_number(&NUMBERS1, "1abc2"));
        assert_eq!(8, last_number(&NUMBERS1, "pqr3stu8vwx"));
        assert_eq!(5, last_number(&NUMBERS1, "a1b2c3d4e5f"));
        assert_eq!(7, last_number(&NUMBERS1, "treb7uchet"));
    }

    #[test]
    fn test_sum_of_first_last() {
        let input = "1abc2\n\
                     pqr3stu8vwx\n\
                     a1b2c3d4e5f\n\
                     treb7uchet";
        assert_eq!(142, sum_of_first_last(&NUMBERS1, input))
    }

    #[test]
    fn part_1() {
        let contents = include_str!("input.txt");
        print!("{}", sum_of_first_last(&NUMBERS1, contents))
    }

    const NUMBERS2: [(&str, u32); 19] = [("1", 1), ("2", 2), ("3", 3), ("4", 4), ("5", 5), ("6", 6),
        ("6", 6), ("7", 7), ("8", 8), ("9", 9), ("one", 1), ("two", 2), ("three", 3), ("four", 4),
        ("five", 5), ("six", 6), ("seven", 7), ("eight", 8), ("nine", 9)];

    #[test]
    fn part_2() {
        let contents = include_str!("input.txt");
        print!("{}", sum_of_first_last(&NUMBERS2, contents))
    }
}
