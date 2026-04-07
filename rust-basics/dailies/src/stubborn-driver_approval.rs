fn approve_driver(driver_name: String, behavior_code: String) -> String {
    // Reverse the driver's name completely
    let reversed_name: String = driver_name.chars().rev().collect();
    
    // Check if the reversed name contains "safe" (case-insensitive)
    let contains_safe = reversed_name.to_lowercase().contains("safe");
    
    // Check if behavior code starts with "good" (case-sensitive)
    let starts_with_good = behavior_code.starts_with("good");
    
    // Return "APPROVED" only if both conditions are met, otherwise "DENIED"
    if contains_safe && starts_with_good {
        "APPROVED".to_string()
    } else {
        "DENIED".to_string()
    }
}
