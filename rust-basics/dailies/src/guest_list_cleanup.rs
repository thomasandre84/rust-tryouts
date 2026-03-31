use std::io;
use std::collections::HashMap;

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line).expect("Failed to read");
    let n: usize = line.trim().parse().expect("Invalid number");
    
    let mut names: Vec<String> = Vec::new();
    let mut badge_numbers: Vec<String> = Vec::new();
    
    // Read alternating names and badge numbers
    for i in 0..n {
        let mut name_line = String::new();
        io::stdin().read_line(&mut name_line).expect("Failed to read");
        let cleaned_name = name_line.trim_end_matches('\n').trim_end_matches('\r').trim_end();
        names.push(cleaned_name.to_string());
        
        let mut badge_line = String::new();
        io::stdin().read_line(&mut badge_line).expect("Failed to read");
        let badge = badge_line.trim().to_string();
        badge_numbers.push(badge);
    }
    
    // Print cleaned names
    println!("Cleaned Names:");
    for name in &names {
        println!("{}", name);
    }
    
    // Find duplicates
    let mut name_counts: HashMap<String, usize> = HashMap::new();
    for name in &names {
        *name_counts.entry(name.clone()).or_insert(0) += 1;
    }
    
    println!("\nDuplicates:");
    let mut found_duplicate = false;
    for (name, count) in &name_counts {
        if *count > 1 {
            println!("{} (appears {} times)", name, count);
            found_duplicate = true;
        }
    }
    if !found_duplicate {
        println!("None");
    }
    
    // Check badge numbers
    println!("\nBadge Status:");
    for badge in &badge_numbers {
        if let Ok(num) = badge.parse::<i32>() {
            if num >= 100 && num <= 999 {
                println!("{}: Valid", badge);
            } else {
                println!("{}: Invalid", badge);
            }
        } else {
            println!("{}: Invalid", badge);
        }
    }
}
