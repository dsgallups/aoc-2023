fn main() {
    let input = include_str!("./input.txt");

    let p1_res = get_parts(input);

    let p1_res = p1_res
        .into_iter()
        .fold(0, |acc, val| if val.1 { acc + val.0 } else { acc });

    println!("{:#?}", p1_res);

    let p2_res = get_gear_ratio_sum(input);
    println!("{:?}", p2_res);
}

#[test]
fn run_p1_tests() {
    let tests = vec![
        ("./day3/src/tests_p1/1.txt", 4361),
        ("./day3/src/tests_p1/2.txt", 0),
        ("./day3/src/tests_p1/3.txt", 0),
        ("./day3/src/tests_p1/4.txt", 1599),
        ("./day3/src/tests_p1/5.txt", 4361),
        ("./day3/src/tests_p1/6.txt", 0),
    ];

    for test in tests {
        println!("=====================new test=================");
        let input = std::fs::read_to_string(test.0).unwrap();

        let res = get_parts(&input);
        res.iter()
            .for_each(|val| println!("({}, {})", val.0, val.1));

        let res = res
            .into_iter()
            .fold(0, |acc, val| if val.1 { acc + val.0 } else { acc });
        println!("test: {}\n res: {}", test.0, res);
        assert_eq!(res, test.1);
    }
}

#[test]
fn run_p2_tests() {
    let tests = vec![
        ("./day3/src/tests_p2/1.txt", 467835),
        ("./day3/src/tests_p2/2.txt", 0),
        ("./day3/src/tests_p2/3.txt", 467835),
        ("./day3/src/tests_p2/4.txt", 467835),
        ("./day3/src/tests_p2/5.txt", 1313544),
        ("./day3/src/tests_p2/6.txt", 32),
        ("./day3/src/tests_p2/7.txt", 22 * 22 * 25),
    ];
    for test in tests {
        println!("=====================new test=================");
        let input = std::fs::read_to_string(test.0).unwrap();

        let res = get_gear_ratio_sum(&input);
        println!("test: {}\n res: {}", test.0, res);
        assert_eq!(res, test.1);
    }
}

fn get_parts(input: &str) -> Vec<(u32, bool)> {
    let mut blobs = Vec::new();

    let map = input
        .split('\n')
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    for (line_no, line) in map.iter().enumerate() {
        let mut numstr_start = (line_no, 0);
        let mut numstr = String::new();

        for (char_i, char) in line.iter().enumerate() {
            if !char.is_ascii_digit() {
                if !numstr.is_empty() {
                    let blob = is_part(numstr.as_str(), numstr_start, &map);
                    blobs.push(blob);

                    numstr = String::new();
                    numstr_start = (line_no, 0);
                }

                continue;
            }

            if numstr.is_empty() {
                numstr_start.1 = char_i;
            }

            numstr.push(*char);

            if (char_i == line.len() - 1) && !numstr.is_empty() {
                let blob = is_part(numstr.as_str(), numstr_start, &map);
                blobs.push(blob);
            }
        }
    }

    blobs
}

fn is_part(num: &str, start: (usize, usize), map: &[Vec<char>]) -> (u32, bool) {
    let line_start = if start.0 == 0 { 0 } else { start.0 - 1 };
    let line_end = if start.0 == map.len() - 1 {
        start.0
    } else {
        start.0 + 1
    };

    for line in map.iter().take(line_end + 1).skip(line_start) {
        let char_start = if start.1 == 0 { 0 } else { start.1 - 1 };

        let char_end = if start.1 + num.len() == line.len() - 1 {
            start.1 + num.len()
        } else {
            start.1 + num.len() + 1
        };

        for char in line.iter().take(char_end).skip(char_start) {
            if !char.is_ascii_digit() && char.ne(&'.') {
                return (num.parse().unwrap(), true);
            }
        }
    }

    (num.parse().unwrap(), false)
}

fn get_gear_ratio_sum(input: &str) -> u64 {
    let map = input
        .split('\n')
        .map(|line| line.trim().chars().collect::<Vec<char>>())
        .collect::<Vec<_>>();

    let valid_gear_ratios = new_get_valid_gears(&map);

    valid_gear_ratios.into_iter().sum()
}

fn new_get_valid_gears(map: &[Vec<char>]) -> Vec<u64> {
    let mut number_locs = Vec::new();
    let mut gear_locs = Vec::new();

    map.iter().enumerate().for_each(|(line_no, line)| {
        let mut numstr_start = (line_no, 0);
        let mut numstr = String::new();

        line.iter().enumerate().for_each(|(char_i, char)| {
            if char.eq(&'*') {
                gear_locs.push((line_no, char_i));
            }
            if !char.is_ascii_digit() {
                if !numstr.is_empty() {
                    number_locs.push(NumberLocation {
                        value: numstr.parse().unwrap(),
                        loc: numstr_start,
                        len: numstr.len(),
                    });

                    numstr = String::new();
                    numstr_start = (line_no, 0);
                }

                return;
            }

            if numstr.is_empty() {
                numstr_start.1 = char_i;
            }

            numstr.push(*char);

            if (char_i == line.len() - 1) && !numstr.is_empty() {
                number_locs.push(NumberLocation {
                    value: numstr.parse().unwrap(),
                    loc: numstr_start,
                    len: numstr.len(),
                });
            }
        });
    });

    gear_locs
        .into_iter()
        .filter_map(|gear_loc| {
            let adj_nos: Vec<NumberLocation> = number_locs
                .iter()
                .filter_map(|number_loc| {
                    let num_char_start = number_loc.loc.1.max(1) - 1;
                    let gear_loc_start = gear_loc.0.max(1) - 1;

                    if (gear_loc_start..=gear_loc.0 + 1).contains(&number_loc.loc.0)
                        && (num_char_start..=number_loc.loc.1 + number_loc.len)
                            .contains(&gear_loc.1)
                    {
                        return Some(number_loc.clone());
                    }
                    None
                })
                .collect();
            if adj_nos.len() != 2 {
                return None;
            }

            Some(
                adj_nos
                    .into_iter()
                    .fold(1, |acc, val| acc * val.value as u64),
            )
        })
        .collect()
}

#[derive(Debug, Clone)]
struct NumberLocation {
    pub value: u32,
    pub loc: (usize, usize),
    pub len: usize,
}
