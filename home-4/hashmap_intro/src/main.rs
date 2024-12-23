fn main() {
    let numbers: Vec<i32> = vec![2, 2, 8, 1, 3, 3, 7, 2, 8, 2, 1, 5, 9];

    let median = med(&numbers);
    let mode = mode(&numbers);

    println!("Median: {}", median);
    println!("Mode: {:?}", mode);
}

fn med(numbers: &[i32]) -> f64 {
    let mut sorted = numbers.to_vec();

    sorted.sort();

    let len = sorted.len();

    if len % 2 == 0 {
        (sorted[len / 2 - 1] as f64 + sorted[len / 2] as f64) / 2.0
    } else {
        sorted[len / 2] as f64
    }
}

fn mode(numbers: &[i32]) -> Vec<i32> {
    let mut occurs = std::collections::HashMap::new();

    for &number in numbers {
        *occurs.entry(number).or_insert(0) += 1;
    }

    let max_count = occurs.values().cloned().max().unwrap_or(0);
    occurs.into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(number, _)| number)
        .collect()
}
