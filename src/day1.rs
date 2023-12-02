use std::collections::HashMap;

pub fn find_digits_including_words(line: &str) -> u32 {
    let lookup: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9),
    ]);

    let (_, first) = lookup
        .iter()
        .fold((line.len(), 0), |(li, v), (key, value)| {
            match line.find(key) {
                None => (li, v),
                Some(i) => {
                    if i < li {
                        (i, *value)
                    } else {
                        (li, v)
                    }
                }
            }
        });

    let (_, second) = lookup
        .iter()
        .fold((0, 0), |(li, v), (key, value)| match line.rfind(key) {
            None => (li, v),
            Some(i) => {
                if i >= li {
                    (i, *value)
                } else {
                    (li, v)
                }
            }
        });

    first * 10 + second
}

pub fn find_digits(line: &str) -> u32 {
    return line
        .chars()
        .find(|c| c.is_ascii_digit())
        .unwrap()
        .to_digit(10)
        .unwrap()
        * 10
        + line
            .chars()
            .rev()
            .find(|c| c.is_ascii_digit())
            .unwrap()
            .to_digit(10)
            .unwrap();
}

#[cfg(test)]
mod tests {
    use crate::day1::{find_digits, find_digits_including_words};

    #[test]
    fn only_digits() {
        let input = "12";
        assert_eq!(12, find_digits(input))
    }

    #[test]
    fn multiple_digits() {
        let input = "1234";
        assert_eq!(14, find_digits(input))
    }

    #[test]
    fn one_digit() {
        let input = "1";
        assert_eq!(11, find_digits(input))
    }

    #[test]
    fn examples_from_description() {
        let inputs = [
            "two1nine",
            "eighttwothree",
            "abcone2threexyz",
            "xtwoone3four",
            "4nineeightseven2",
            "zoneight234",
            "7pqrstsixteen",
        ];
        let expected_answers = [29, 83, 13, 24, 42, 14, 76];
        let actual_answers = inputs.map(find_digits_including_words);
        assert_eq!(expected_answers, actual_answers)
    }

    #[test]
    fn only_one_word() {
        let input = "seven";
        assert_eq!(77, find_digits_including_words(input))
    }
}
