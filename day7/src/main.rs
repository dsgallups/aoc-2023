use std::cmp::Ordering;

const CARDS: [&str; 13] = [
    "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
];

const CARDS_TWO: [&str; 13] = [
    "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
];

fn main() {
    let input = include_str!("./input.txt");

    pt1(input);
    pt2(input);
}
fn pt1(input: &str) {
    let mut lines = input
        .split('\n')
        .map(|l| {
            let (hand, bid) = l.trim().split_once(' ').unwrap();
            (hand.trim().to_string(), bid.parse().unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    lines.sort_by(|a, b| {
        //determine value of hand
        let a =
            a.0.split("")
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>();

        let b =
            b.0.split("")
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>();

        let mut a_count: Vec<(&str, u8)> = a.iter().fold(Vec::new(), |mut acc, card| {
            match acc.iter().position(|count| &count.0 == card) {
                Some(i) => {
                    let card_count = acc.get_mut(i).unwrap();

                    card_count.1 += 1;
                }
                None => acc.push((card, 1)),
            }
            acc
        });

        a_count.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        let mut b_count: Vec<(&str, u8)> = b.iter().fold(Vec::new(), |mut acc, card| {
            match acc.iter().position(|count| &count.0 == card) {
                Some(i) => {
                    let card_count = acc.get_mut(i).unwrap();

                    card_count.1 += 1;
                }
                None => acc.push((card, 1)),
            }
            acc
        });
        b_count.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

        match a_count[0].1.cmp(&b_count[0].1) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                if a_count[0].1 == 3 || a_count[0].1 == 2 {
                    match a_count[1].1.cmp(&b_count[1].1) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                    }
                }
            }
        }

        //same hand at type at this point

        for (a, b) in a.iter().zip(b.iter()) {
            let a_val = CARDS.iter().position(|v| v == a).unwrap();
            let b_val = CARDS.iter().position(|v| v == b).unwrap();

            match a_val.cmp(&b_val) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }

        panic!("Something went wrong, where a = {:?}, b = {:?}", a, b);
    });

    let mut total_val = 0;

    for (i, line) in lines.into_iter().enumerate() {
        total_val += line.1 as usize * (i + 1);
    }
    println!("final val: {}", total_val);
}

fn pt2(input: &str) {
    let mut lines = input
        .split('\n')
        .map(|l| {
            let (hand, bid) = l.trim().split_once(' ').unwrap();
            (hand.trim().to_string(), bid.parse().unwrap())
        })
        .collect::<Vec<(String, i32)>>();

    lines.sort_by(|a, b| {
        //determine value of hand
        let a =
            a.0.split("")
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>();

        let b =
            b.0.split("")
                .filter(|v| !v.is_empty())
                .collect::<Vec<&str>>();

        let a_count: Vec<(&str, u8)> = sort_cards(&a);

        let b_count = sort_cards(&b);

        match a_count[0].1.cmp(&b_count[0].1) {
            Ordering::Greater => return Ordering::Greater,
            Ordering::Less => return Ordering::Less,
            Ordering::Equal => {
                if a_count[0].1 == 3 || a_count[0].1 == 2 {
                    match a_count[1].1.cmp(&b_count[1].1) {
                        Ordering::Greater => return Ordering::Greater,
                        Ordering::Less => return Ordering::Less,
                        Ordering::Equal => {}
                    }
                }
            }
        }

        //println!("{:?}", a);

        for (a, b) in a.iter().zip(b.iter()) {
            let a_val = CARDS_TWO.iter().position(|v| v == a).unwrap();
            let b_val = CARDS_TWO.iter().position(|v| v == b).unwrap();

            match a_val.cmp(&b_val) {
                Ordering::Greater => return Ordering::Greater,
                Ordering::Less => return Ordering::Less,
                Ordering::Equal => {}
            }
        }

        panic!("Something went wrong, where a = {:?}, b = {:?}", a, b);
    });

    let mut total_val = 0;

    for (i, line) in lines.into_iter().enumerate() {
        total_val += line.1 as usize * (i + 1);
    }
    println!("final val: {}", total_val);
}

fn sort_cards<'a>(a: &'a [&str]) -> Vec<(&'a str, u8)> {
    let mut a_count: Vec<(&str, u8)> = a.iter().fold(Vec::new(), |mut acc, card| {
        if card == &"J" {
            return acc;
        }

        match acc.iter().position(|count| &count.0 == card) {
            Some(i) => {
                let card_count = acc.get_mut(i).unwrap();

                card_count.1 += 1;
            }
            None => acc.push((card, 1)),
        }
        acc
    });

    a_count.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());

    //now consider the joker
    let mut joker_count = 0;
    for card in a.iter() {
        if card == &"J" {
            joker_count += 1;
        }
    }
    if joker_count == 5 {
        return vec![("J", 5)];
    }

    let largest_count = a_count.get_mut(0).unwrap();
    largest_count.1 += joker_count;

    a_count
}
