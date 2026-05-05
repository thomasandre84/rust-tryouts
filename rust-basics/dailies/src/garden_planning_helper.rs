use std::io;

fn main() {
    // Read the RLE string
    let mut rle_line = String::new();
    io::stdin().read_line(&mut rle_line).expect("Failed to read");
    let rle_string = rle_line.trim();
    
    // Decode the RLE string
    let decoded = decode_rle(rle_string);
    println!("{}", decoded);
    
    // Read the costs and budget
    let mut budget_line = String::new();
    io::stdin().read_line(&mut budget_line).expect("Failed to read");
    let parts: Vec<i32> = budget_line
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse"))
        .collect();
    
    let a = parts[0]; // cost per concrete block
    let b = parts[1]; // cost per plank
    let c = parts[2]; // total budget
    
    // Find all solutions where a*x + b*y = c
    let mut solutions = Vec::new();
    
    // x can range from 0 to c/a (if a > 0)
    if a == 0 && b == 0 {
        // Special case: both costs are 0
        if c == 0 {
            // Infinite solutions, but we'll just print (0, 0)
            solutions.push((0, 0));
        }
        // else no solutions
    } else if a == 0 {
        // Only y matters: b*y = c
        if c % b == 0 {
            let y = c / b;
            if y >= 0 {
                solutions.push((0, y));
            }
        }
    } else if b == 0 {
        // Only x matters: a*x = c
        if c % a == 0 {
            let x = c / a;
            if x >= 0 {
                solutions.push((x, 0));
            }
        }
    } else {
        // General case: both a and b are non-zero
        let max_x = c / a;
        for x in 0..=max_x {
            let remaining = c - a * x;
            if remaining >= 0 && remaining % b == 0 {
                let y = remaining / b;
                if y >= 0 {
                    solutions.push((x, y));
                }
            }
        }
    }
    
    // Print solutions
    if solutions.is_empty() {
        println!("No solutions");
    } else {
        for (x, y) in solutions {
            println!("{} {}", x, y);
        }
    }
}

fn decode_rle(s: &str) -> String {
    let mut result = String::new();
    let chars: Vec<char> = s.chars().collect();
    let mut i = 0;
    
    while i < chars.len() {
        // Read the number
        let mut num_str = String::new();
        while i < chars.len() && chars[i].is_ascii_digit() {
            num_str.push(chars[i]);
            i += 1;
        }
        
        // Read the character
        if i < chars.len() {
            let ch = chars[i];
            let count: usize = num_str.parse().unwrap_or(1);
            for _ in 0..count {
                result.push(ch);
            }
            i += 1;
        }
    }
    
    result
}
