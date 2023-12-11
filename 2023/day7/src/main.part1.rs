use std::cmp::Ordering;

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Debug)]
enum Card {
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    Ace = 14,
    Bad = 0,
}

#[derive(Eq, Ord, PartialEq, PartialOrd, Debug)]
enum HandType {
    HighCard = 1,
    OnePair = 2,
    TwoPair = 3,
    ThreeOfAKind = 4,
    FullHouse = 5,
    FourOfAKind = 6,
    FiveOfAKind = 7,
}

#[derive(Eq, Debug)]
struct Hand {
    cards: Vec<Card>,
    wager: usize,
    hand_type: HandType,
}

fn process_hand_type(mut cards: Vec<Card>) -> HandType {
    let mut i: usize = 0;
    let mut hand_type = HandType::HighCard;
    cards.sort();
    while i < cards.len() {
        let mut count: usize = 1;
        while i + count < cards.len() && cards[i] == cards[i + count] {
            count += 1;
        }
        if count == 2 && hand_type == HandType::HighCard {
            hand_type = HandType::OnePair;
        } else if count == 2 && hand_type == HandType::OnePair {
            return HandType::TwoPair;
        } else if count == 2 && hand_type == HandType::ThreeOfAKind {
            return HandType::FullHouse;
        } else if count == 3 && hand_type == HandType::HighCard {
            hand_type = HandType::ThreeOfAKind;
        } else if count == 3 && hand_type == HandType::OnePair {
            return HandType::FullHouse;
        } else if count == 4 {
            return HandType::FourOfAKind;
        } else if count == 5 {
            return HandType::FiveOfAKind;
        }
        i += count;
    }
    return hand_type;
}

impl Hand {
    fn new(cards_chars: Vec<char>, wager: usize) -> Self {
        let mut cards: Vec<Card> = vec![];
        for c in cards_chars {
            let card = match c {
                'A' => Card::Ace,
                'K' => Card::King,
                'Q' => Card::Queen,
                'J' => Card::Jack,
                'T' => Card::Ten,
                '9' => Card::Nine,
                '8' => Card::Eight,
                '7' => Card::Seven,
                '6' => Card::Six,
                '5' => Card::Five,
                '4' => Card::Four,
                '3' => Card::Three,
                '2' => Card::Two,
                _ => Card::Bad,
            };
            cards.push(card);
        }
        let hand_type: HandType = process_hand_type(cards.clone()); 
        return Self {
            cards,
            wager,
            hand_type,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
    
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}


impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type != other.hand_type {
            return self.hand_type.cmp(&other.hand_type);
        } else {
            let mut i = 0;
            while i < self.cards.len() {
                if self.cards[i] != other.cards[i] {
                    return self.cards[i].cmp(&other.cards[i]);
                } else {
                    i += 1;
                }
            }
        }
        return self.hand_type.cmp(&other.hand_type);
    }
}

fn process_hand(line: &str) -> Hand {
    let mut parts = line.split(' ');
    let cards_chars: Vec<char> = parts.next().unwrap().chars().collect();
    let wager: usize = parts.next().unwrap().parse().unwrap();
    Hand::new(cards_chars, wager)
}


pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    let mut all_hands: Vec<Hand> = vec![];
    for line in data.lines() {
        all_hands.push(process_hand(line));
    }
    all_hands.sort();
    dbg!(&all_hands);
    let mut total_wagers: usize = 0;
    for rank in 0..all_hands.len() {
        total_wagers += all_hands[rank].wager * (rank + 1);
    }
    return total_wagers;
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(6440, part_one(data));
    }

}
