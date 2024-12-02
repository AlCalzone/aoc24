const INPUT: &'static str = include_str!("input.txt");

fn main() {
    let reports = INPUT.lines().map(|l| {
        l.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let incremental = reports.map(|r| r.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i32>>());

    let increasing = incremental
        .clone()
        .filter(|increments| increments.iter().all(|&i| i > 0 && i <= 3));
    let decreasing = incremental.filter(|increments| increments.iter().all(|&i| i < 0 && i >= -3));

    let safe = increasing.chain(decreasing);

    println!("Safe: {}", safe.count());
}
