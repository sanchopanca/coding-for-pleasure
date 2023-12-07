use aoc2023::*;

use counter::Counter;

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Copy, Clone, Debug)]
enum Card2 {
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

impl Card2 {
    fn from_char(c: char) -> Card2 {
        match c {
            'J' => Card2::Joker,
            '2' => Card2::Two,
            '3' => Card2::Three,
            '4' => Card2::Four,
            '5' => Card2::Five,
            '6' => Card2::Six,
            '7' => Card2::Seven,
            '8' => Card2::Eight,
            '9' => Card2::Nine,
            'T' => Card2::Ten,
            'Q' => Card2::Queen,
            'K' => Card2::King,
            'A' => Card2::Ace,
            _ => panic!("Invalid card"),
        }
    }
}

struct Hand {
    cards: [Card; 5],
}

struct Hand2 {
    cards: [Card2; 5],
}

trait CreatedFromStrAndOrdered: Ord {
    fn from_str(s: &str) -> Self;
}

impl CreatedFromStrAndOrdered for Hand {
    fn from_str(s: &str) -> Self {
        Self {
            cards: s
                .chars()
                .map(Card::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

impl CreatedFromStrAndOrdered for Hand2 {
    fn from_str(s: &str) -> Self {
        Self {
            cards: s
                .chars()
                .map(Card2::from_char)
                .collect::<Vec<_>>()
                .try_into()
                .unwrap(),
        }
    }
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Hash, Copy, Clone, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPairs,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_type<T>(cards: [T; 5]) -> HandType
where
    T: Ord,
    T: Copy,
{
    let mut h = cards;
    h.sort();
    if (0..4).all(|i| h[i] == h[i + 1]) {
        return HandType::FiveOfAKind;
    }
    if (0..3).all(|i| h[i] == h[i + 1]) || (1..4).all(|i| h[i] == h[i + 1]) {
        return HandType::FourOfAKind;
    }
    if (0..2).all(|i| h[i] == h[i + 1]) {
        if h[3] == h[4] {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if (2..4).all(|i| h[i] == h[i + 1]) {
        if h[0] == h[1] {
            return HandType::FullHouse;
        }
        return HandType::ThreeOfAKind;
    }
    if (1..3).all(|i| h[i] == h[i + 1]) {
        return HandType::ThreeOfAKind;
    }
    if h[0] == h[1] {
        if h[2] == h[3] || h[3] == h[4] {
            return HandType::TwoPairs;
        }
        return HandType::OnePair;
    }
    if h[1] == h[2] {
        if h[3] == h[4] {
            return HandType::TwoPairs;
        }
        return HandType::OnePair;
    }
    if (2..4).any(|i| h[i] == h[i + 1]) {
        HandType::OnePair
    } else {
        HandType::HighCard
    }
}

impl Hand {
    fn get_type(&self) -> HandType {
        get_type(self.cards)
    }
}

impl Hand2 {
    fn get_type(&self) -> HandType {
        let mut h = self.cards;
        let hand_without_jokers = h
            .iter()
            .filter(|&&c| c != Card2::Joker)
            .copied()
            .collect::<Counter<_>>();

        let most_common = if hand_without_jokers.len() > 0 {
            hand_without_jokers.k_most_common_ordered(1)[0].0
        } else {
            Card2::Ace
        };
        h.sort();
        let h = h
            .into_iter()
            .map(|card| {
                if card == Card2::Joker {
                    most_common
                } else {
                    card
                }
            })
            .collect::<Vec<_>>();

        get_type(h.try_into().unwrap())
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl PartialEq for Hand2 {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Eq for Hand {}

impl Eq for Hand2 {}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let by_type = self.get_type().cmp(&other.get_type());
        if by_type != std::cmp::Ordering::Equal {
            return by_type;
        }

        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl Ord for Hand2 {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let by_type = self.get_type().cmp(&other.get_type());
        if by_type != std::cmp::Ordering::Equal {
            return by_type;
        }

        for i in 0..5 {
            if self.cards[i] != other.cards[i] {
                return self.cards[i].cmp(&other.cards[i]);
            }
        }

        std::cmp::Ordering::Equal
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialOrd for Hand2 {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    part1();
    part2();
}

fn part1() {
    process_hands::<Hand>();
}

fn part2() {
    process_hands::<Hand2>();
}

fn process_hands<T>()
where
    T: CreatedFromStrAndOrdered,
{
    let input = read_input_to_lines(7);
    let mut hands = Vec::new();
    for line in input {
        let (hand, bid) = line.split_once(' ').unwrap();
        let hand = T::from_str(hand);
        let bid: u64 = bid.parse().unwrap();
        hands.push((hand, bid));
    }

    hands.sort_by(|a, b| a.0.cmp(&b.0));

    let mut total = 0;
    for (i, (_, b)) in hands.into_iter().enumerate() {
        total += (i as u64 + 1) * b;
    }

    println!("{total}");
}
