use crate::time_it;
use crate::utils::read_lines;
use colored::Colorize;
use std::collections::{BTreeSet, HashMap};
use std::fmt::Debug;
use std::vec;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Ranks<T> {
    HighCard(Cards<T>),
    OnePair(Cards<T>),
    TwoPair(Cards<T>),
    ThreeOfAKind(Cards<T>),
    FullHouse(Cards<T>),
    FourOfAKind(Cards<T>),
    FiveOfAKind(Cards<T>),
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card1 {
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

impl From<char> for Card1 {
    fn from(c: char) -> Self {
        match c {
            '2' => Card1::Two,
            '3' => Card1::Three,
            '4' => Card1::Four,
            '5' => Card1::Five,
            '6' => Card1::Six,
            '7' => Card1::Seven,
            '8' => Card1::Eight,
            '9' => Card1::Nine,
            'T' => Card1::Ten,
            'J' => Card1::Jack,
            'Q' => Card1::Queen,
            'K' => Card1::King,
            'A' => Card1::Ace,
            _ => todo!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
enum Card2 {
    Jack,
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

impl From<char> for Card2 {
    fn from(c: char) -> Self {
        match c {
            '2' => Card2::Two,
            '3' => Card2::Three,
            '4' => Card2::Four,
            '5' => Card2::Five,
            '6' => Card2::Six,
            '7' => Card2::Seven,
            '8' => Card2::Eight,
            '9' => Card2::Nine,
            'T' => Card2::Ten,
            'J' => Card2::Jack,
            'Q' => Card2::Queen,
            'K' => Card2::King,
            'A' => Card2::Ace,
            _ => todo!(),
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash)]
struct Cards<T>(T, T, T, T, T);

fn get_rank<T>(cards_str: &str, joker: bool) -> Ranks<T>
where
    T: std::hash::Hash + std::cmp::Eq + std::cmp::Ord + From<char> + Copy + Debug,
{
    let mut cards: Vec<T> = vec![];
    for c in cards_str.chars() {
        cards.push(c.into());
    }

    let card_tuple: Cards<T> = Cards(cards[0], cards[1], cards[2], cards[3], cards[4]);
    let mut card_counts: HashMap<T, i32> = HashMap::new();
    for card in cards {
        *card_counts.entry(card).or_insert(0) += 1;
    }

    if joker {
        let joker_count = *card_counts.get(&T::from('J')).unwrap_or(&0);
        if joker_count == 5 {
            return Ranks::FiveOfAKind(card_tuple);
        }
        card_counts.remove_entry(&T::from('J'));
        let temp = card_counts.clone();
        let max = temp
            .iter()
            .max_by(|a, b| a.1.cmp(b.1))
            .map(|(k, _v)| k)
            .unwrap();
        *card_counts.get_mut(max).unwrap() += joker_count;
    }

    let mut reverse_map: HashMap<i32, BTreeSet<T>> = HashMap::new();
    for (k, v) in card_counts.iter() {
        reverse_map
            .entry(*v)
            .or_insert_with(|| BTreeSet::from([*k]))
            .insert(*k);
    }

    let count_set = BTreeSet::from_iter(reverse_map.keys());

    if count_set.len() == 1 {
        if count_set.contains(&1) {
            return Ranks::HighCard(card_tuple);
        }

        if count_set.contains(&5) {
            return Ranks::FiveOfAKind(card_tuple);
        }
    }

    if count_set.len() == 2 {
        if count_set.contains(&4) {
            return Ranks::FourOfAKind(card_tuple);
        }

        if count_set.contains(&3) && count_set.contains(&2) {
            return Ranks::FullHouse(card_tuple);
        }

        if count_set.contains(&3) && count_set.contains(&1) {
            return Ranks::ThreeOfAKind(card_tuple);
        }

        if count_set.contains(&2) && reverse_map.get(&2).unwrap().len() == 2 {
            return Ranks::TwoPair(card_tuple);
        }
    }

    Ranks::OnePair(card_tuple)
}

fn problem1(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut result: u64 = 0;

    let mut ranks: Vec<(Ranks<Card1>, u64)> = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let cards = split[0];
        let bid: u64 = split[1].parse().unwrap();
        ranks.push((get_rank::<Card1>(cards, false), bid));
    }
    ranks.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, (_, bid)) in ranks.iter().enumerate() {
        result += (i as u64 + 1) * bid;
    }
    result
}

fn problem2(f: &str) -> u64 {
    let lines = read_lines(f);
    let mut result: u64 = 0;

    let mut ranks: Vec<(Ranks<Card2>, u64)> = vec![];

    for line in lines {
        let split: Vec<&str> = line.split(' ').collect();
        let cards = split[0];
        let bid: u64 = split[1].parse().unwrap();
        ranks.push((get_rank::<Card2>(cards, true), bid));
    }
    ranks.sort_by(|a, b| a.0.cmp(&b.0));
    for (i, (_, bid)) in ranks.iter().enumerate() {
        result += (i as u64 + 1) * bid;
    }
    result
}
pub fn solve() {
    // https://adventofcode.com/2023/day/7
    time_it!(7, 1, problem1("files/7.txt"));
    time_it!(7, 2, problem2("files/7.txt"));
}

mod tests {

    #[test]
    fn test_problem1() {
        assert_eq!(crate::day7::problem1("files/7_test.txt"), 6440);
    }

    #[test]
    fn test_problem2() {
        assert_eq!(crate::day7::problem2("files/7_test.txt"), 5905);
    }
}
