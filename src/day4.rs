#[derive(Eq, PartialEq, Debug, Clone)]
pub struct Card {
    pub id: u32,
    pub winners: Vec<u32>,
    pub numbers: Vec<u32>,
    pub score: u32,
    pub matches: u32,
}

impl Card {
    pub fn new(id: u32, winners: Vec<u32>, numbers: Vec<u32>) -> Self {
        let matches = numbers
            .iter()
            .fold(0, |acc, number| match winners.contains(number) {
                true => acc + 1,
                false => acc,
            });

        let score = 2u32.pow(matches) / 2;

        Card {
            id,
            winners,
            numbers,
            score,
            matches,
        }
    }

    pub fn create_from_input(input: &str) -> Option<Card> {
        let split: Vec<Vec<&str>> = input
            .split(':')
            .flat_map(|f| f.split('|').map(|g| g.split_whitespace().collect()))
            .collect();

        match split.len() == 3 {
            true => Some(Card::new(
                split[0][1].parse().unwrap_or(0),
                split[1].iter().map(|s| s.parse().unwrap_or(0)).collect(),
                split[2].iter().map(|s| s.parse().unwrap_or(0)).collect(),
            )),
            false => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::day4::Card;
    use std::fs::read_to_string;

    #[test]
    fn test_game_score() {
        let game = Card::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );

        assert_eq!(8, game.score)
    }

    #[test]
    fn test_create_from_input() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let expected_game = Card::new(
            1,
            vec![41, 48, 83, 86, 17],
            vec![83, 86, 6, 31, 17, 9, 48, 53],
        );
        let actual_game = Card::create_from_input(input);
        assert!(actual_game.is_some());
        assert_eq!(expected_game, actual_game.unwrap());
    }

    #[test]
    #[ignore]
    fn day_4_part_1() {
        let input = read_to_string("input/day4.txt").unwrap();
        let mut pile = Vec::new();
        input
            .lines()
            .for_each(|line| pile.push(Card::create_from_input(line).unwrap()));
        println!(
            "Day 4 Part 1: {}",
            pile.iter().map(|c| c.score).sum::<u32>()
        );
    }

    #[test]
    #[ignore]
    fn day_4_part_2() {
        let input = read_to_string("input/day4.txt").unwrap();
        let mut pile = Vec::new();
        input
            .lines()
            .for_each(|line| pile.push(Card::create_from_input(line).unwrap()));
        let mut counts: Vec<u128> = vec![1; pile.len()];

        for (i, _card) in pile.iter().enumerate() {
            let count = counts[i];
            counts[(i + 1).min(pile.len())..=(i + _card.matches as usize).min(pile.len())]
                .iter_mut()
                .for_each(|c| *c += count);
        }

        println!("Day 4 Part 2: {}", counts.iter().sum::<u128>());
    }
}
