use std::fs;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone)]
pub struct BingoCard {
    pub board: Vec<Vec<usize>>,
    pub called: Vec<(usize, usize)>,
}

pub trait Mark {
    fn mark(&mut self, number: usize) -> usize;
}

impl Mark for BingoCard {
    fn mark(&mut self, number: usize) -> usize {
        for (y, row) in self.board.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if item == &number {
                    self.called.push((x, y));
                    return 1;
                }
            }
        }
        0
    }
}

pub trait CheckWinner {
    fn check_winner(&self) -> bool;
}

impl CheckWinner for BingoCard {
    fn check_winner(&self) -> bool {
        for i in 0..5 {
            if self
                .called
                .iter()
                .filter(|x| x.0 == i)
                .collect::<Vec<_>>()
                .len()
                == 5
            {
                return true;
            }

            if self
                .called
                .iter()
                .filter(|x| x.1 == i)
                .collect::<Vec<_>>()
                .len()
                == 5
            {
                return true;
            }
        }
        false
    }
}

pub trait GetScore {
    fn get_score(&self, last_called: usize) -> usize;
}

impl GetScore for BingoCard {
    fn get_score(&self, last_called: usize) -> usize {
        let mut sum: usize = 0;

        for (y, row) in self.board.iter().enumerate() {
            for (x, item) in row.iter().enumerate() {
                if self
                    .called
                    .iter()
                    .filter(|coordinates| coordinates.0 == x && coordinates.1 == y)
                    .collect::<Vec<_>>()
                    .len()
                    == 0
                {
                    sum += item
                }
            }
        }
        sum * last_called
    }
}

pub fn get_numbers(file: &str) -> Vec<usize> {
    let path = format!("./input/{}.txt", file);

    let f = fs::File::open(path).expect("Unable to open file");
    let mut f = BufReader::new(f);

    let mut buf = String::new();

    f.read_line(&mut buf).expect("reading line failed");

    buf.split(",")
        .map(|s| s.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>()
}

pub fn get_cards(file: &str) -> Vec<BingoCard> {
    let path = format!("./input/{}.txt", file);

    let f = fs::File::open(path).expect("Unable to open file");
    let f = BufReader::new(f);

    let mut result = Vec::<BingoCard>::new();

    for line in f.lines().skip(1) {
        let string = line.unwrap();
        if string.len() == 0 {
            result.push(BingoCard {
                board: Vec::new(),
                called: Vec::new(),
            })
        } else {
            let idx = result.len() - 1;
            result[idx].board.push(
                string
                    .split_whitespace()
                    .filter_map(|w| w.parse().ok())
                    .collect(),
            );
        }
    }

    result
}

pub fn find_winner(cards: &Vec<BingoCard>, numbers: &Vec<usize>) -> usize {
    let mut bingo_cards = cards.clone();

    for number in numbers {
        for card in bingo_cards.iter_mut() {
            let found = card.mark(*number) == 1;
            if found {
                if card.check_winner() {
                    return card.get_score(*number);
                }
            }
        }
    }
    0
}

pub fn find_last_winner(cards: &Vec<BingoCard>, numbers: &Vec<usize>) -> usize {
    let mut bingo_cards = cards.clone();
    let mut win_count: usize = 0;

    for number in numbers {
        for card in bingo_cards.iter_mut() {
            if !card.check_winner() {
                let found = card.mark(*number) == 1;
                if found {
                    if card.check_winner() {
                        win_count += 1;
                        if win_count == cards.len() {
                            return card.get_score(*number);
                        }
                    }
                }
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    const DAY: &str = "day-4";

    #[test]
    fn it_maintains_correct_answers() {
        let numbers = get_numbers(DAY);
        let cards = get_cards(DAY);
        assert_eq!(38594, find_winner(&cards, &numbers));
        assert_eq!(21184, find_last_winner(&cards, &numbers));
    }
}
