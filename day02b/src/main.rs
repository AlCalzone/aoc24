const INPUT: &'static str = include_str!("input.txt");

fn is_safe(report: &[i32]) -> bool {
    let incremental = report.windows(2).map(|w| w[1] - w[0]);
    if incremental.clone().all(|i| i > 0 && i <= 3) {
        return true;
    }
    if incremental.clone().all(|i| i < 0 && i >= -3) {
        return true;
    }
    false
}

fn main() {
    let reports = INPUT.lines().map(|l| {
        l.split_whitespace()
            .map(|n| n.parse::<i32>().unwrap())
            .collect::<Vec<i32>>()
    });

    let problematic = reports.clone().filter(|r| !is_safe(r));
    let really_safe_count = reports.clone().count() - problematic.clone().count();

    // Not efficient, but this is Rust, not Python :D
    let fixable_count = problematic
        .filter(|increments| {
            for i in 0..increments.len() {
                let mut with_removed = increments.clone();
                with_removed.remove(i);

                if is_safe(&with_removed) {
                    return true;
                }
            }
            false
        })
        .count();

    println!("Really safe: {}", really_safe_count);
    println!("Fixable: {}", fixable_count);
    println!("Total: {}", really_safe_count + fixable_count);
}
