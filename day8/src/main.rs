use std::collections::HashMap;

fn main() {
    pt2();
}

fn pt1() {
    let input = include_str!("./example.txt");
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    let mut lines = input.split('\n');

    let og_path = lines.next().unwrap().trim().chars();

    lines.next();

    for line in lines {
        let (k, v) = line.trim().split_once(" = ").unwrap();

        let v = v.trim().replace(['(', ')'], "");

        let (v1, v2) = v.split_once(", ").unwrap();

        map.insert(k.to_string(), (v1.to_string(), v2.to_string()));
    }

    println!("{:?}", og_path);
    println!("{:#?}", map);
    let mut loc = "AAA".to_string();
    let mut path = og_path.clone();
    let mut count = 0;
    loop {
        if loc == "ZZZ" {
            break;
        }

        let direction = match path.next() {
            Some(p) => p,
            None => {
                path = og_path.clone();
                path.next().unwrap()
            }
        };

        let val = map.get(&loc).unwrap();

        match direction {
            'L' => {
                loc = val.0.clone();
            }
            'R' => {
                loc = val.1.clone();
            }

            _ => panic!("invalid direction"),
        }
        count += 1;
    }

    println!("count = {}", count);
}

fn pt2() {
    let input = include_str!("./input.txt");
    let mut lines = input.split('\n');

    let og_path = lines.next().unwrap().trim().chars();

    lines.next();
    let mut locs = Vec::new();
    let mut map: HashMap<String, (String, String)> = HashMap::new();

    for line in lines {
        let (k, v) = line.trim().split_once(" = ").unwrap();

        let v = v.trim().replace(['(', ')'], "");

        let (v1, v2) = v.split_once(", ").unwrap();

        if k.ends_with('A') {
            locs.push(k.to_string());
        }

        map.insert(k.to_string(), (v1.to_string(), v2.to_string()));
    }

    //println!("{:?}", map);

    let mut count = 0;
    let mut path = og_path.clone();
    loop {
        let mut cont = false;
        for loc in locs.iter() {
            if !loc.ends_with('Z') {
                cont = true;
            }
        }
        if !cont {
            break;
        }

        let direction = match path.next() {
            Some(p) => p,
            None => {
                path = og_path.clone();
                path.next().unwrap()
            }
        };

        for loc in locs.iter_mut() {
            let val = map.get(loc).unwrap();
            match direction {
                'L' => {
                    *loc = val.0.clone();
                }
                'R' => {
                    *loc = val.1.clone();
                }
                _ => panic!("invalid direction"),
            }
        }
        //println!("{:?}", locs);
        count += 1;
    }

    println!("count = {}", count);
}
