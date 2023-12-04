use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    //pt_one(input);
    pt_two(input)
}
fn pt_one(input: &str) {
    let mut total = 0;
    for line in input.split('\n') {
        let line = line.trim();
        let (card, draw) = line.split_once(" | ").unwrap();
        let card_nos = card
            .split(' ')
            .skip(2)
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let draw_nos = draw
            .split(' ')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>();

        println!("{:?}, {:?}", card_nos, draw_nos);
        let mut amt = 0;

        for card_no in card_nos {
            if draw_nos.contains(&card_no) {
                if amt == 0 {
                    amt = 1
                } else {
                    amt *= 2
                }
            }
        }
        total += amt;
        println!("{}", amt);
    }
    println!("total: {}", total);
}

fn pt_two(input: &str) {
    let mut card_results: Vec<u32> = Vec::new();
    let mut copies: HashMap<usize, u32> = HashMap::new();

    for (card_number, line) in input.split('\n').enumerate() {
        let line = line.trim();
        let (card, draw) = line.split_once(" | ").unwrap();
        let card_nos = card
            .split(' ')
            .skip(2)
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let draw_nos = draw
            .split(' ')
            .filter_map(|s| s.trim().parse::<u32>().ok())
            .collect::<Vec<u32>>();

        let mut amt = 0;

        for card_no in card_nos {
            if draw_nos.contains(&card_no) {
                amt += 1;
            }
        }
        card_results.push(amt);
        copies.insert(card_number, 0);
    }

    let mut card_count = 0;

    for (card_number, result) in card_results.into_iter().enumerate() {
        let mut count = *copies.get(&card_number).unwrap() + 1;

        card_count += count;

        while count != 0 {
            let mut res_copy = result;
            while res_copy != 0 {
                let v = copies.get_mut(&(card_number + res_copy as usize)).unwrap();
                *v += 1;
                res_copy -= 1;
            }
            count -= 1;
        }
    }

    println!("card_count: {}", card_count);
}
