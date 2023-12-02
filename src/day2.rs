use regex::{Captures, Regex};

#[derive(Eq, PartialEq, Debug)]
pub struct Game {
    pub id: i32,
    pub red: Vec<i32>,
    pub green: Vec<i32>,
    pub blue: Vec<i32>,
}

impl Game {
    pub fn new(id: i32, red: Vec<i32>, green: Vec<i32>, blue: Vec<i32>) -> Self {
        Game {
            id,
            red,
            green,
            blue,
        }
    }

    pub fn from_input_row(input: &str) -> Game {
        let game_expr = Regex::new(r"Game ([0-9]+):").unwrap();
        let red_expr = Regex::new(r"([0-9]+) red").unwrap();
        let green_expr = Regex::new(r"([0-9]+) green").unwrap();
        let blue_expr = Regex::new(r"([0-9]+) blue").unwrap();
        let transform_caps =
            |c: Captures| c.get(1).map_or("0", |m| m.as_str()).parse::<i32>().unwrap();

        let game_id: i32 = game_expr
            .captures(input)
            .unwrap()
            .get(1)
            .map_or("0", |m| m.as_str())
            .parse()
            .unwrap();

        let red = red_expr.captures_iter(input).map(transform_caps).collect();
        let green = green_expr
            .captures_iter(input)
            .map(transform_caps)
            .collect();
        let blue = blue_expr.captures_iter(input).map(transform_caps).collect();

        Game::new(game_id, red, green, blue)
    }
}

#[cfg(test)]
mod tests {
    use crate::day2::Game;

    #[test]
    fn test_game_all_colors() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let expected = Game::new(1, vec![4, 1], vec![2, 2], vec![3, 6]);
        assert_eq!(expected, Game::from_input_row(input))
    }

    #[test]
    fn test_game_one_color() {
        let input = "Game 1: 3 blue; 1 blue; 6 blue";
        let expected = Game::new(1, vec![], vec![], vec![3, 1, 6]);
        assert_eq!(expected, Game::from_input_row(input))
    }
}
