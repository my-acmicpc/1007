use std::{collections::BTreeSet, io};

fn solution(points: &Vec<(i64, i64)>, selection: BTreeSet<usize>) -> Option<i64> {
    let vector_count = points.len() / 2;

    if selection.len() == vector_count {
        let (x, y) = points
            .iter()
            .enumerate()
            .fold((0, 0), |(acc_x, acc_y), (index, (x, y))| {
                if selection.contains(&index) {
                    (acc_x + x, acc_y + y)
                } else {
                    (acc_x - x, acc_y - y)
                }
            });
        return Some(x * x + y * y);
    }

    let start = if let Some(&i) = selection.last() {
        i + 1
    } else {
        0
    };
    let mut result: Option<i64> = None;
    for i in start..points.len() {
        let mut next_selection = selection.clone();
        next_selection.insert(i);
        result = if let Some(solution) = solution(points, next_selection) {
            if let Some(result) = result {
                Some(result.min(solution))
            } else {
                Some(solution)
            }
        } else {
            result
        }
    }
    result
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).unwrap();
    let test_case_count = line.trim().parse::<usize>().unwrap();
    for _ in 0..test_case_count {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        let point_count = line.trim().parse::<usize>().unwrap();
        let mut points = Vec::with_capacity(point_count);
        for _ in 0..point_count {
            let mut line = String::new();
            io::stdin().read_line(&mut line).unwrap();
            let mut iter = line.trim().split(' ').flat_map(&str::parse::<i64>);
            points.push((iter.next().unwrap(), iter.next().unwrap()))
        }

        println!(
            "{}",
            (solution(&points, BTreeSet::new()).unwrap() as f64).sqrt()
        );
    }
}
