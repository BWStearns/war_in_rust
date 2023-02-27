use rand::prelude::*;
use rand::Rng;

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
enum Cards {
    Ace,
    King,
    Queen,
    Jack,
    Number(u8),
}

enum Winner {
    Player1,
    Player2,
    Draw,
}

impl Cards {
    fn value(&self) -> u8 {
        match self {
            Cards::Ace => 14,
            Cards::King => 13,
            Cards::Queen => 12,
            Cards::Jack => 11,
            Cards::Number(n) => *n,
        }
    }

    fn compare(&self, other: &Cards) -> Winner {
        if self.value() > other.value() {
            Winner::Player1
        } else if self.value() < other.value() {
            Winner::Player2
        } else {
            Winner::Draw
        }
    }
}

// struct GameState {
//     hand1: Vec<Cards>,
//     hand2: Vec<Cards>,
//     at_risk: Vec<Cards>,
// }

fn main() {
    let mut deck = get_deck();
    deck.shuffle(&mut thread_rng());
    // println!("{:?}", deck);
    let mut hand_1: Vec<Cards> = vec![Cards::Number(2); 26];
    let mut hand_2: Vec<Cards> = vec![Cards::Number(2); 26];
    hand_1[..26].clone_from_slice(deck[..26].as_ref());
    hand_2[..26].clone_from_slice(deck[26..].as_ref());
    let mut at_risk: Vec<Cards> = vec![];
    loop {
        if hand_1.is_empty() && hand_2.is_empty() {
            println!("Draw!");
            break;
        } else
        if hand_1.is_empty() {
            println!("Player 2 wins!");
            break;
        } else if hand_2.is_empty() {
            println!("Player 1 wins!");
            break;
        }

        let p1_card = hand_1.pop().unwrap();
        let p2_card = hand_2.pop().unwrap();
        at_risk.push(p1_card);
        at_risk.push(p2_card);
        match p1_card.compare(&p2_card) {
            Winner::Player1 => {
                at_risk.iter().for_each(|card| hand_1.push(*card));
                at_risk.clear();
            },
            Winner::Player2 => {
                at_risk.iter().for_each(|card| hand_2.push(*card));
                at_risk.clear();
            },
            Winner::Draw => {},
        }
        println!("At risk: {:?}, Player 1: {:?} vs Player 2: {:?}", at_risk.len(), hand_1.len(), hand_2.len());
    }

}



fn get_deck() -> Vec<Cards> {
    let deck = vec![
        Cards::Ace,
        Cards::King,
        Cards::Queen,
        Cards::Jack,
        Cards::Number(10),
        Cards::Number(9),
        Cards::Number(8),
        Cards::Number(7),
        Cards::Number(6),
        Cards::Number(5),
        Cards::Number(4),
        Cards::Number(3),
        Cards::Number(2),
        Cards::Ace,
        Cards::King,
        Cards::Queen,
        Cards::Jack,
        Cards::Number(10),
        Cards::Number(9),
        Cards::Number(8),
        Cards::Number(7),
        Cards::Number(6),
        Cards::Number(5),
        Cards::Number(4),
        Cards::Number(3),
        Cards::Number(2),
        Cards::Ace,
        Cards::King,
        Cards::Queen,
        Cards::Jack,
        Cards::Number(10),
        Cards::Number(9),
        Cards::Number(8),
        Cards::Number(7),
        Cards::Number(6),
        Cards::Number(5),
        Cards::Number(4),
        Cards::Number(3),
        Cards::Number(2),
        Cards::Ace,
        Cards::King,
        Cards::Queen,
        Cards::Jack,
        Cards::Number(10),
        Cards::Number(9),
        Cards::Number(8),
        Cards::Number(7),
        Cards::Number(6),
        Cards::Number(5),
        Cards::Number(4),
        Cards::Number(3),
        Cards::Number(2),
    ];
    deck
}