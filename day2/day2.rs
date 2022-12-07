use std::fs;

struct Game {
    opponent_hand: char,
    player_hand: char,
}

impl Game {
    fn new(opponent_hand: char, player_hand: char) -> Game {
        Game {
            opponent_hand,
            player_hand,
        }
    }

    fn evaluate_game(&self) -> i32 {
        match self.opponent_hand {
            'A' => match self.player_hand {
                'X' => 3 + 1,
                'Y' => 6 + 2,
                'Z' => 0 + 3,
                _ => 0,
            },
            'B' => match self.player_hand {
                'X' => 0 + 1,
                'Y' => 3 + 2,
                'Z' => 6 + 3,
                _ => 0,
            },
            'C' => match self.player_hand {
                'X' => 6 + 1,
                'Y' => 0 + 2,
                'Z' => 3 + 3,
                _ => 0,
            },
            _ => 0,
        }
    }
}
fn main() {

    let file_contents = fs::read_to_string("input.txt")
    .expect("Something went wrong reading the file");


    let games: Vec<Game> = file_contents.lines().map(|line| {
        let mut iter = line.split_whitespace();
        let opponent_hand = iter.next().unwrap().chars().next().unwrap();
        let player_hand = iter.next().unwrap().chars().next().unwrap();

        Game::new(opponent_hand, player_hand)
    }).collect();

    let score = games
                    .iter()
                    .map(|game| game.evaluate_game())
                    .sum::<i32>();

    println!("Score: {}", score);


}