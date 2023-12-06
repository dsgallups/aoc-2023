fn main() {
    let input = include_str!("./input.txt");
    pt1(input);
    pt2(input);
}

fn pt1(input: &str) {
    let lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|line| {
            line.split(' ')
                .skip(1)
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let times = &lines[0];
    let dists = &lines[1];

    let races = times
        .iter()
        .zip(dists)
        .map(|(time, distance)| (*time, *distance))
        .collect::<Vec<(i32, i32)>>();

    let mut total = 1;
    for (time, race_distance) in races {
        let mut winning_combs = 0;
        for btn_time in 0..=time {
            let remaining_time = time - btn_time;

            let distance_travelled = btn_time * remaining_time;
            //println!("distance travelled = {}", distance_travelled);

            if distance_travelled > race_distance {
                winning_combs += 1;
            }
        }

        println!("winning comes for {}: {}", time, winning_combs);
        total *= if winning_combs > 1 { winning_combs } else { 1 };
    }
    println!("total: {}", total);
}

fn pt2(input: &str) {
    let lines = input
        .split('\n')
        .map(|l| l.trim().to_string())
        .collect::<Vec<String>>()
        .into_iter()
        .map(|line| {
            line.split(' ')
                .skip(1)
                .filter_map(|v| v.parse::<i32>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let times = &lines[0];
    let dists = &lines[1];

    let mut t_str = String::new();
    let mut t_dist = String::new();
    for time in times {
        t_str.push_str(&time.to_string());
    }
    for dist in dists {
        t_dist.push_str(&dist.to_string());
    }

    let time = t_str.parse::<i128>().unwrap();
    let dist = t_dist.parse::<i128>().unwrap();

    let mut winning_combs = 0;
    for btn_time in 0..=time {
        let remaining_time = time - btn_time;

        let distance_travelled = btn_time * remaining_time;
        //println!("distance travelled = {}", distance_travelled);

        if distance_travelled > dist {
            winning_combs += 1;
        }
    }

    println!("winning combs = {}", winning_combs);
}
