fn main() {
    let input = include_str!("./input.txt");
    p1(input);
    p2(input);
}

fn p1(input: &str) {
    let mut lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .peekable();

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
    let mut lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .peekable();

    let og_seeds = lines
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|v| v.parse::<i128>().unwrap())
        .collect::<Vec<i128>>();

    let og_seeds = (0..og_seeds.len() / 2)
        .map(|i| Range::new(og_seeds[i * 2], og_seeds[(i * 2) + 1]))
        .collect::<Vec<Range>>();

    let mut seeds = og_seeds.clone();

    loop {
        let line_clone = &mut lines;

        let mut maps = line_clone
            .skip_while(|line| !line.contains("map"))
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|map_line| {
                let vals = map_line
                    .split(' ')
                    .map(|v| v.parse::<i128>().unwrap())
                    .collect::<Vec<i128>>();
                let destination = vals[0];
                let source = vals[1];
                let len = vals[2];
                (Range::new(source, len), destination)
            })
            .collect::<Vec<(Range, i128)>>();

        maps.sort_by(|a, b| a.0.start.cmp(&b.0.start));

        seeds = seeds.into_iter().fold(Vec::new(), |mut acc, seed| {
            let mut new_ranges = seed.map(&maps);
            acc.append(&mut new_ranges);
            acc
        });

        if line_clone.peek().is_none() {
            break;
        }
    }

    println!("seeds: ");
    seeds.sort_by(|a, b| a.start.cmp(&b.start));

    let mut overlaps = 0;

    seeds.windows(2).for_each(|window| {
        let seed_one = &window[0];
        let seed_two = &window[1];
        if seed_one.start + seed_one.len < seed_two.start {
            println!(
                "({}, {}) => ({})",
                seed_one.start, seed_one.len, seed_two.start
            );
            overlaps += 1;
        }
    });
    println!("overlaps: {}", overlaps);
    let min = seeds.into_iter().fold(i128::MAX, |dec, v| dec.min(v.start));
    println!("min = {}", min);
}

#[derive(Debug, Clone)]
pub struct Range {
    pub start: i128,
    pub len: i128,
}

impl Range {
    pub fn new(start: i128, len: i128) -> Self {
        Self { start, len }
    }

    pub fn map(&self, map_ranges: &[(Range, i128)]) -> Vec<Range> {
        let mut res = Vec::new();

        let mut cursor = self.start;

        for (range, map) in map_ranges {
            if cursor > self.start + self.len - 1 {
                break;
            }

            if range.start + range.len - 1 < cursor {
                continue;
            }

            if range.start > cursor && range.start < self.start + self.len {
                let end = range.start;

                let new_range_len = end - cursor;

                let new_range = Range::new(cursor, new_range_len);
                res.push(new_range);
                cursor += new_range_len;
            }

            if (range.start..range.start + range.len).contains(&cursor) {
                let end = if range.start + range.len > self.start + self.len {
                    self.start + self.len
                } else {
                    range.start + range.len
                };
                let new_range_len = end - cursor;

                let map_diff = *map - range.start;

                let new_range = Range::new(cursor + map_diff, new_range_len);
                res.push(new_range);
                cursor += new_range_len;
            }
        }

        if cursor < self.start + self.len {
            let new_range_len = (self.start + self.len) - cursor;

            let new_range = Range::new(cursor, new_range_len);
            res.push(new_range);
        }

        res
    }
}
