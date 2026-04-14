fn format_safe_speeds(speeds: Vec<i32>, speed_limit: i32) -> String {
    let safe_speeds: Vec<i32> = speeds
        .into_iter()
        .filter(|&speed| speed < speed_limit)
        .take(3)
        .collect();
    
    let formatted_speeds: Vec<String> = safe_speeds
        .iter()
        .map(|speed| format!("{}kph", speed))
        .collect();
    
    format!("Safe speeds: {}", formatted_speeds.join(", "))
}
