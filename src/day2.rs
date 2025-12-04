use aoc_runner_derive::aoc;

#[derive(Debug)]
struct GameRecord {
    id: usize,
    hands: Vec<Hand>,
}
impl GameRecord {
    fn new(input: &str) -> GameRecord {
        let (game_id, input) = input.split_once(": ").unwrap();
        let id = (&game_id[5..]).parse().unwrap();
        let hands = input.split("; ").map(Hand::new).collect();
        GameRecord { id, hands }
    }
}

#[derive(Debug)]
struct Hand {
    red: usize,
    green: usize,
    blue: usize,
}
impl Hand {
    fn new(input: &str) -> Hand {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for amount_and_color in input.split(", ") {
            let (amount, color) = amount_and_color.split_once(" ").unwrap();
            let amount = amount.parse().unwrap();
            match color {
                "red" => {
                    red = amount;
                }
                "green" => {
                    green = amount;
                }
                "blue" => {
                    blue = amount;
                }
                _ => panic!("invalid hand"),
            }
        }
        Hand { red, green, blue }
    }
}

#[aoc(day2, part1)]
fn part1(input: &str) -> usize {
    let games: Vec<_> = input.split("\n").map(GameRecord::new).collect();
    games
        .into_iter()
        .filter(|game| {
            game.hands
                .iter()
                .all(|hand| hand.red <= 12 && hand.green <= 13 && hand.blue <= 14)
        })
        .map(|game| game.id)
        .sum()
}

#[aoc(day2, part2)]
fn part2(input: &str) -> usize {
    let games: Vec<_> = input.split("\n").map(GameRecord::new).collect();
    games
        .into_iter()
        .map(|game| {
            let mut max_hand = Hand {
                red: 0,
                green: 0,
                blue: 0,
            };
            for hand in &game.hands {
                max_hand.red = max_hand.red.max(hand.red);
                max_hand.green = max_hand.green.max(hand.green);
                max_hand.blue = max_hand.blue.max(hand.blue);
            }
            max_hand.red * max_hand.green * max_hand.blue
        })
        .sum()
}
