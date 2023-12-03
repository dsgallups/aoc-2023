const MAX_RED: u32 = 12;
const MAX_GREEN: u32 = 13;
const MAX_BLUE: u32 = 14;

fn main() {
    let input = include_str!("./input.txt");
    let count_p1 = input.split('\n').fold(0, |acc, line| {
        let sep = line.trim().split_once(": ").unwrap();

        let game_no = sep
            .0
            .chars()
            .skip(5)
            .take_while(|v| v.ne(&':'))
            .collect::<String>()
            .parse::<u32>()
            .unwrap();

        if sep.1.split("; ").fold(true, |acc, round| {
            if !acc {
                return acc;
            }

            valid_round(round)
        }) {
            return acc + game_no;
        }

        acc
    });

    //r,g,b
    let count_p2 = input.split('\n').fold(0, |acc, line| {
        let game_acc = line
            .trim()
            .split_once(": ")
            .unwrap()
            .1
            .split("; ")
            .fold((0, 0, 0), |acc, round| max_tuple(acc, count_rgb(round)));

        acc + (game_acc.0 * game_acc.1 * game_acc.2)
    });

    println!("{}", count_p1);

    println!("{}", count_p2);
}

fn valid_round(round: &str) -> bool {
    let parsed_round = round
        .split(", ")
        .map(|round_amount| {
            let round_amount: Vec<&str> = round_amount.split(' ').collect();

            (round_amount[0].parse::<u32>().unwrap(), round_amount[1])
        })
        .collect::<Vec<_>>();

    for reveal in parsed_round {
        match reveal.1 {
            "blue" => {
                if reveal.0 > MAX_BLUE {
                    return false;
                }
            }
            "red" => {
                if reveal.0 > MAX_RED {
                    return false;
                }
            }
            "green" => {
                if reveal.0 > MAX_GREEN {
                    return false;
                }
            }
            _ => panic!("invalid color provided"),
        }
    }

    true
}

fn count_rgb(round: &str) -> (u32, u32, u32) {
    let mut res = (0, 0, 0);

    let parsed_round = round
        .split(", ")
        .map(|round_amount| {
            let round_amount: Vec<&str> = round_amount.split(' ').collect();

            (round_amount[0].parse::<u32>().unwrap(), round_amount[1])
        })
        .collect::<Vec<_>>();

    for reveal in parsed_round {
        match reveal.1 {
            "blue" => {
                res.2 = reveal.0.max(res.2);
            }
            "red" => {
                res.0 = reveal.0.max(res.0);
            }
            "green" => {
                res.1 = reveal.0.max(res.1);
            }
            _ => panic!("invalid color provided"),
        }
    }

    res
}

fn max_tuple(t1: (u32, u32, u32), t2: (u32, u32, u32)) -> (u32, u32, u32) {
    (t1.0.max(t2.0), t1.1.max(t2.1), t1.2.max(t2.2))
}
