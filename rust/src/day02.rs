#![allow(dead_code)]

use std::usize;

#[derive(Debug)]
struct Hand {
    blue: usize,
    green: usize,
    red: usize,
}
#[derive(Debug)]
struct Game {
    nr: usize,
    hands: Vec<Hand>,
}
impl Game {
    fn from(line: &str) -> Game {
        let mut game = Game { nr: 0, hands: vec![] };

        let mut top_it = line.split(':');
        let nr = top_it
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .parse::<usize>()
            .unwrap();
        for hand_str in top_it.next().unwrap().split(';') {
            let mut hand = Hand { blue: 0, green: 0, red: 0};
            for ball in hand_str.split(',') {
                let mut ball_it = ball.split_whitespace();
                let amount = ball_it.next().unwrap().parse::<usize>().unwrap();
                let color = ball_it.next().unwrap();
                if color == "blue" {
                    hand.blue = amount;
                } else if color == "red" {
                    hand.red=  amount;
                } else if color == "green" {
                    hand.green = amount;
                }
            }
            game.hands.push(hand);
        }
        game.nr = nr;
        game
    }
}

pub fn task1(input: &str) -> u64 {
    let mut games: Vec<_> = Vec::new();
    for line in input.lines() {
        let new_game = Game::from(line);
        games.push(new_game);
    }

    return games.iter().filter(|game| {
                       for hand in game.hands.iter() {
                           if hand.red> 12 {
                               return false;
                           }
                           if hand.green > 13 {
                               return false;
                           }
                           if hand.blue > 14 {
                               return false;
                           }
                       }
                       return true;})
        .map(|game| game.nr as u64).sum();
}

pub fn task2(input: &str) -> u64 {
    let mut games: Vec<_> = Vec::new();
    for line in input.lines() {
        let new_game = Game::from(line);
        games.push(new_game);
    }

    return games.iter().map(|game| {
        let mut max_blue = 0;
        let mut max_green = 0;
        let mut max_red = 0;

        for hand in game.hands.iter() {
            max_blue = std::cmp::max(max_blue, hand.blue);
            max_green = std::cmp::max(max_green, hand.green);
            max_red = std::cmp::max(max_red, hand.red);
        }
        (max_red*max_green*max_blue) as u64
    }).sum(); 
}

#[test]
fn task1_test() {
    let data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
    assert_eq!(task1(data), 8);
}

#[test]
fn task2_test() {
    let data = r#"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
"#;
    assert_eq!(task2(data), 2286);
}
