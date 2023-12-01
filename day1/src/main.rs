const NUMBERS: [char; 10] = ['1', '2', '3', '4', '5', '6', '7', '8', '9', '0'];
const NUMBERS_WORDS: [&str; 10] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "zero",
];

fn main() {
    let input = include_str!("./input.txt");

    let res_one = pt_one(input);
    println!("\n\npt_one res: {}", res_one);
    let res_two = pt_two(input);

    println!("\n\npt_two res: {}", res_two);
}

fn pt_one(input: &str) -> i32 {
    let result = input.split('\n').fold(0, |acc, x| {
        let x_clone = x.trim().to_string();

        let mut chars = x
            .trim()
            .chars()
            .filter(|x| NUMBERS.contains(x))
            .collect::<Vec<char>>();

        let num = if let Some((first, els)) = chars.split_first_mut() {
            let mut number = String::from(*first);

            if let Some(last) = els.last_mut() {
                number.push(*last);
            } else {
                number.push(*first);
            }

            number.parse::<i32>().unwrap()
        } else {
            0
        };

        println!("c: {}, num: {}, acc: {}", x_clone, num, acc);
        acc + num
    });

    result
}

fn pt_two(input: &str) -> i32 {
    let mut acc = 0;
    for line in input.split('\n') {
        let mut un = get_unordered_numbers(line.trim());
        un.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
        let mut sorted = un.into_iter().map(|v| v.1).collect::<Vec<_>>();

        let num = if let Some((first, els)) = sorted.split_first_mut() {
            let mut number = String::from(*first);

            if let Some(last) = els.last_mut() {
                number.push(*last);
            } else {
                number.push(*first);
            }

            number.parse::<i32>().unwrap()
        } else {
            0
        };
        acc += num;
    }

    acc
}

fn get_unordered_numbers(line: &str) -> Vec<(usize, char)> {
    let mut numbers: Vec<(usize, char)> = Vec::new();

    NUMBERS_WORDS.iter().for_each(|word| {
        let mut locs = line
            .match_indices(word)
            .map(|(index, word)| {
                let digit = word_to_digit(word).unwrap();
                (index, digit)
            })
            .collect::<Vec<(usize, char)>>();
        numbers.append(&mut locs);
    });

    let mut number_chars = line
        .chars()
        .enumerate()
        .filter_map(|(i, x)| {
            if NUMBERS.contains(&x) {
                Some((i, x))
            } else {
                None
            }
        })
        .collect::<Vec<(usize, char)>>();
    numbers.append(&mut number_chars);

    numbers
}

fn word_to_digit(word: &str) -> Option<char> {
    match word {
        "one" => Some('1'),
        "two" => Some('2'),
        "three" => Some('3'),
        "four" => Some('4'),
        "five" => Some('5'),
        "six" => Some('6'),
        "seven" => Some('7'),
        "eight" => Some('8'),
        "nine" => Some('9'),
        "zero" => Some('0'),
        _ => None,
    }
}
