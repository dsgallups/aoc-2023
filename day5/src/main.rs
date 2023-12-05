fn main() {
    let input = include_str!("./input.txt");
    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .peekable();

    let mut lines = lines.into_iter();

    let og_seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|v| v.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    let mut seeds = og_seeds.clone();

    println!("{:?}", seeds);

    //lines.for_each(|v| println!("{}", v));
    loop {
        let line_clone = &mut lines;

        let mut mapped = (0..seeds.len()).map(|_| false).collect::<Vec<bool>>();

        line_clone
            .skip_while(|line| !line.contains("map"))
            .skip(1)
            .take_while(|line| !line.is_empty())
            .for_each(|map_line| {
                let vals = map_line
                    .split(' ')
                    .map(|v| v.parse::<u128>().unwrap())
                    .collect::<Vec<u128>>();
                let destination = vals[0];
                let source = vals[1];
                let len = vals[2];

                seeds.iter_mut().enumerate().for_each(|(i, seed)| {
                    if mapped[i] {
                        return;
                    }

                    if (source..source + len).contains(seed) {
                        *seed = destination + (*seed - source);
                        mapped[i] = true;
                    }
                });
            });

        println!("{:?}", seeds);
        if line_clone.peek().is_none() {
            break;
        }
    }

    let res = seeds.into_iter().fold(u128::MAX, |acc, val| acc.min(val));

    println!("result: {}", res);
}

fn p2(input: &str) {
    let lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .peekable();

    let mut lines = lines.into_iter();

    let og_seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|v| v.parse::<u128>().unwrap())
        .collect::<Vec<u128>>();

    println!("here 1");
    let og_seeds = (0..=og_seeds.len() / 2)
        .map(|i| (og_seeds[i], og_seeds[i + 1]))
        .collect::<Vec<(u128, u128)>>()
        .into_iter()
        .fold(Vec::new(), |mut acc, val| {
            for i in 0..val.1 {
                acc.push(val.0 + i);
            }
            acc
        });

    println!("here 2");
    let mut seeds = og_seeds.clone();

    println!("{:?}", seeds);

    //lines.for_each(|v| println!("{}", v));
    loop {
        let line_clone = &mut lines;

        let mut mapped = (0..seeds.len()).map(|_| false).collect::<Vec<bool>>();

        line_clone
            .skip_while(|line| !line.contains("map"))
            .skip(1)
            .take_while(|line| !line.is_empty())
            .for_each(|map_line| {
                let vals = map_line
                    .split(' ')
                    .map(|v| v.parse::<u128>().unwrap())
                    .collect::<Vec<u128>>();
                let destination = vals[0];
                let source = vals[1];
                let len = vals[2];

                seeds.iter_mut().enumerate().for_each(|(i, seed)| {
                    if mapped[i] {
                        return;
                    }

                    if (source..source + len).contains(seed) {
                        *seed = destination + (*seed - source);
                        mapped[i] = true;
                    }
                });
            });

        println!("{:?}", seeds);
        if line_clone.peek().is_none() {
            break;
        }
    }

    let res = seeds.into_iter().fold(u128::MAX, |acc, val| acc.min(val));

    println!("result: {}", res);
}
