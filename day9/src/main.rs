fn main() {
    let input = include_str!("./input.txt");
    pt1(input);
    pt2(input);
}

fn pt2(input: &str) {
    let values: Vec<Vec<i128>> = input
        .split('\n')
        .map(|l| {
            l.trim()
                .split(' ')
                .filter_map(|v| v.parse::<i128>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut total = 0;
    for number_set in values {
        let mut all_differences: Vec<Vec<i128>> = vec![number_set.clone()];

        let mut differences = number_set.clone();
        loop {
            let mut cont = false;
            for diff in differences.iter() {
                if diff != &0 {
                    cont = true;
                }
            }
            if !cont {
                break;
            }
            differences = differences
                .windows(2)
                .map(|numbers| {
                    let n1 = numbers[0];
                    let n2 = numbers[1];

                    n2 - n1
                })
                .collect::<Vec<_>>();
            all_differences.push(differences.clone());
        }

        total += all_differences
            .into_iter()
            .rev()
            .fold(0, |acc, diff_line| *diff_line.first().unwrap() - acc);
    }
    println!("total = {}", total);
}

fn pt1(input: &str) {
    let values: Vec<Vec<i128>> = input
        .split('\n')
        .map(|l| {
            l.trim()
                .split(' ')
                .filter_map(|v| v.parse::<i128>().ok())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<_>>>();

    let mut total = 0;
    for number_set in values {
        let mut all_differences: Vec<Vec<i128>> = vec![number_set.clone()];

        let mut differences = number_set.clone();
        loop {
            let mut cont = false;
            for diff in differences.iter() {
                if diff != &0 {
                    cont = true;
                }
            }
            if !cont {
                break;
            }
            differences = differences
                .windows(2)
                .map(|numbers| {
                    let n1 = numbers[0];
                    let n2 = numbers[1];

                    n2 - n1
                })
                .collect::<Vec<_>>();
            all_differences.push(differences.clone());
        }

        total += all_differences
            .into_iter()
            .rev()
            .fold(0, |acc, diff_line| acc + *diff_line.last().unwrap());
    }
    println!("total = {}", total);
}
