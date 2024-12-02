use std::fs;

fn main() {
    println!("The answer for day one is...{0}", day_one());
}

fn day_one() -> i32 {
    let path = "inputs/input_1.txt";
    println!("Attempting to read file...");

    let contents = fs::read_to_string(path).expect("Could not read file");
    let lines: Vec<&str> = contents.split("\n").collect();
    let mut left = Vec::new();
    let mut right = Vec::new();
    lines.iter().for_each(|line| {
        let parts: Vec<&str> = line.split("   ").collect();
        if parts.len() == 2 {
            if let (Ok(n1), Ok(n2)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
                left.push(n1);
                right.push(n2);
            }
        }
    });

    left.sort();
    right.sort();

    return left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();
}
