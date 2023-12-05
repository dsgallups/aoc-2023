use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");

    let lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>();

    let mut lines = lines.into_iter();

    let og_seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|v| v.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    let mut seeds = og_seeds.clone();

    println!("{:?}", seeds);

    //lines.for_each(|v| println!("{}", v));

    loop {
        println!("in loop");
        let line_clone = &mut lines;

        let mut map: HashMap<u32, u32> = HashMap::new();

        line_clone
            .skip_while(|line| !line.contains("map"))
            .skip(1)
            .take_while(|line| !line.is_empty())
            .for_each(|map_line| {
                let vals = map_line
                    .split(' ')
                    .map(|v| v.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>();
                let destination = vals[0];
                let source = vals[1];
                let len = vals[2];

                for i in 0..len {
                    map.insert(source + i, destination + i);
                }
            });
        println!("here 1");

        seeds.iter_mut().for_each(|seed| {
            if let Some(val) = map.get(seed) {
                *seed = *val;
            }
        });
        println!("{:?}", seeds);
        if map.is_empty() {
            break;
        }
    }

    let res = seeds.into_iter().fold(u32::MAX, |acc, val| acc.min(val));

    println!("result: {}", res);
}
