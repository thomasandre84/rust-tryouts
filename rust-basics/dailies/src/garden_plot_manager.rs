use std::io;

fn main() {
    let mut task_line = String::new();
    io::stdin().read_line(&mut task_line).expect("Failed to read");
    let task: i32 = task_line.trim().parse().expect("Invalid task number");
    
    let mut data_line = String::new();
    io::stdin().read_line(&mut data_line).expect("Failed to read");
    let data = data_line.trim();
    
    match task {
        1 => {
            // Task 1: Count numbers with odd number of divisors (perfect squares)
            let n: i32 = data.parse().expect("Invalid number");
            let count = (1..=n).filter(|&x| has_odd_divisors(x)).count();
            println!("{}", count);
        },
        2 => {
            // Task 2: Determine watering intervention based on moisture
            let moisture: i32 = data.parse().expect("Invalid moisture reading");
            let intervention = if moisture >= 60 {
                "no_action"
            } else if moisture >= 40 {
                "light_watering"
            } else if moisture >= 20 {
                "moderate_watering"
            } else {
                "heavy_watering"
            };
            println!("{}", intervention);
        },
        3 => {
            // Task 3: Calculate alternating sum of Pascal's triangle row
            let n: i32 = data.parse().expect("Invalid row number");
            let result = alternating_sum_pascal(n);
            println!("{}", result);
        },
        _ => {
            println!("Invalid task number");
        }
    }
}

fn has_odd_divisors(n: i32) -> bool {
    // A number has odd number of divisors if and only if it's a perfect square
    let sqrt = (n as f64).sqrt() as i32;
    sqrt * sqrt == n
}

fn alternating_sum_pascal(n: i32) -> i64 {
    // The alternating sum of row n in Pascal's triangle is:
    // C(n,0) - C(n,1) + C(n,2) - C(n,3) + ...
    // This equals 0 for n > 0 and 1 for n = 0
    // This is because (1-1)^n = 0 for n > 0
    if n == 0 {
        1
    } else {
        0
    }
}
