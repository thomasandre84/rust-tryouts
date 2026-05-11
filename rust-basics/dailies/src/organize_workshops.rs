fn organize_workshop(tool_inventory: String) -> String {
    let mut report = String::from("WORKSHOP INVENTORY\n");
    let mut borrowed_count = 0;
    
    let tools: Vec<&str> = tool_inventory.split(',').collect();
    
    for tool in tools {
        let parts: Vec<&str> = tool.split(':').collect();
        if parts.len() != 3 {
            continue;
        }
        
        let name = parts[0];
        let condition = parts[1];
        let tool_type = parts[2];
        
        match condition {
            "good" => {
                report.push_str(&format!("- [{}] {}\n", tool_type.to_uppercase(), name));
            },
            "borrowed" => {
                borrowed_count += 1;
                continue;
            },
            "broken" => {
                break;
            },
            _ => continue,
        }
    }
    
    report.push_str(&format!("Borrowed tools: {}", borrowed_count));
    report
}
