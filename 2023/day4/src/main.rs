pub fn main() {
    let data = include_str!("input.txt");
    println!("Part 1: {}", part_one(data));
}

fn part_one(data: &str) -> usize {
    let mut total_cards: usize = 0;
    let mut cards: Vec<&str> = data.lines().collect();
    let num_cards = cards.len();
    let mut card_multiplier: Vec<usize> = vec![1; num_cards];
    let cards = cards.into_iter();
    for card in cards {
        let card_count: usize = match card_multiplier.get(0 as usize) {
            Some(count) => *count,
            None => 1,
        };
        total_cards += card_count;
        if card_multiplier.len() > 1 {
            card_multiplier = card_multiplier[1..].to_vec();
        } else {
            card_multiplier.clear();
        }
        let points = process_card(card);
        for i in 0..points {
            match card_multiplier.get(i) {
                Some(_) => card_multiplier[i] += card_count,
                None => card_multiplier.push(1 + card_count),
            }
            dbg!(&card_multiplier);
        }
    }
    return total_cards;
}

fn process_card(card: &str) -> usize {
    let mut res: usize = 0;
    let mut card_parts = card.split(':');
    let _card_title = card_parts.next().unwrap();
    let card_hands = card_parts.next().unwrap();
    let mut card_hands_split = card_hands.split('|');
    let hand_one = card_hands_split.next().unwrap();
    let hand_two = card_hands_split.next().unwrap();
    let hand_two_cards: Vec<&str> = hand_two.split(' ').filter(|x| !x.is_empty()).into_iter().collect();
    let hand_one_cards = hand_one.split(' ').filter(|x| !x.is_empty());
    for c in hand_one_cards {
        if hand_two_cards.contains(&c) {
            if c != "" {
                res += 1
            }
        }
    }

    return res;

}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn one() {
        let data = include_str!("test.txt");
        assert_eq!(30, part_one(data));
    }

//    #[test]
//    fn two() {
//        let data = include_str!("test.txt");
//        assert_eq!(, part_two(data));
//    }
}
