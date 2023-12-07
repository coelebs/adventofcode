use aoclib::aocinit;
use itertools::Itertools;
use log::{debug, info};
use std::cmp::Ordering;

/* Scores
 * 0 = high card
 * 1 = one pair
 * 2 = two pair
 * 3 = three of a kind
 * 4 = full house
 * 5 = four of a kind
 * 6 = five of a kind
 */

fn score(a: &str) -> u8 {
    let list: Vec<char> = a.chars().collect();
    let mut max: u8 = 0;
    let mut maxchar = ' ';
    for c in &list {
        let cnt: u8 = list
            .iter()
            .filter(|x| **x == *c)
            .count()
            .try_into()
            .unwrap();

        if cnt > max {
            max = cnt;
            maxchar = *c;
        }
    }

    if max > 3 {
        // return 6 for a
        return max + 1;
    }

    if max == 3 {
        if list.iter().filter(|x| **x != maxchar).dedup().count() == 1 {
            return 4;
        } else {
            return 3;
        }
    }

    return match list.iter().sorted().dedup().count() {
        3 => 2,
        4 => 1,
        5 => 0,
        _ => panic!("WTF"),
    };
}

static KINDS: &'static [char] = &['A', 'K', 'Q', 'J', 'T', '9', '8', '7', '6', '5', '4', '3', '2'];
fn compare(a: &str, b: &str) -> Ordering {
    let ascore = score(a);
    let bscore = score(b);

    if ascore > bscore {
        return Ordering::Greater;
    } else if ascore < bscore {
        return Ordering::Less;
    }

    let cmb = a.chars().zip(b.chars());

    for ab in cmb {
        if ab.0 == ab.1 {
            continue;
        }

        let aval = KINDS.iter().position(|&c| c == ab.0).unwrap();
        let bval = KINDS.iter().position(|&c| c == ab.1).unwrap();
        debug!("Comparing {a} and {b}:");
        debug!(" a: {aval}");
        debug!(" b: {bval}");

        return bval.cmp(&aval);
    }

    return Ordering::Equal;
}

fn day7(input: String) {
    let mut camel_deck: Vec<(&str, u64)> = input
        .lines()
        .map(|x| x.split_ascii_whitespace().collect_tuple().unwrap())
        .map(|x: (&str, &str)| (x.0, x.1.parse::<u64>().unwrap()))
        .collect();
    debug!("Unsorted: {:#?}", camel_deck);
    camel_deck.sort_by(|a, b| compare(a.0, b.0));
    debug!("Sorted: {:#?}", camel_deck);
    debug!("{}", camel_deck.len());

    let result1: u64 = camel_deck
        .iter()
        .enumerate()
        .map(|x| (x.0 as u64 + 1, x.1))
        .map(|x| x.0 * (x.1 .1))
        .sum();
    info!("Camel card winnings: {result1}");
}

fn main() {
    aocinit(day7);
}
