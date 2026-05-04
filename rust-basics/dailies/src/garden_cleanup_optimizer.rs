use std::collections::HashMap;

fn optimize_garden_cleanup(volunteers: Vec<String>, sections: Vec<String>, tools: Vec<String>) -> String {
    // Parse volunteers
    let mut volunteer_list: Vec<(String, i32)> = volunteers
        .iter()
        .map(|v| {
            let parts: Vec<&str> = v.split(':').collect();
            (parts[0].to_string(), parts[1].parse::<i32>().unwrap())
        })
        .collect();
    
    // Sort volunteers by skill level (highest first)
    volunteer_list.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Parse sections
    let mut section_list: Vec<(String, i32, String)> = sections
        .iter()
        .map(|s| {
            let parts: Vec<&str> = s.split(':').collect();
            (parts[0].to_string(), parts[1].parse::<i32>().unwrap(), parts[2].to_string())
        })
        .collect();
    
    // Sort sections by difficulty (highest first)
    section_list.sort_by(|a, b| b.1.cmp(&a.1));
    
    // Parse tools into a HashMap
    let mut tool_inventory: HashMap<String, i32> = HashMap::new();
    for tool in tools {
        let parts: Vec<&str> = tool.split(':').collect();
        tool_inventory.insert(parts[0].to_string(), parts[1].parse::<i32>().unwrap());
    }
    
    let mut assignments: Vec<(String, i32, String, i32)> = Vec::new();
    let mut used_volunteers: Vec<bool> = vec![false; volunteer_list.len()];
    
    // Assign volunteers to sections
    for (section_name, difficulty, tool_needed) in section_list {
        // Check if tool is available
        if let Some(&available) = tool_inventory.get(&tool_needed) {
            if available > 0 {
                // Find a suitable volunteer
                for (i, (volunteer_name, skill)) in volunteer_list.iter().enumerate() {
                    if !used_volunteers[i] && *skill >= difficulty {
                        // Assign this volunteer
                        assignments.push((section_name.clone(), difficulty, volunteer_name.clone(), *skill));
                        used_volunteers[i] = true;
                        // Consume the tool
                        tool_inventory.insert(tool_needed.clone(), available - 1);
                        break;
                    }
                }
            }
        }
    }
    
    // Format the output
    let mut result = String::from("GARDEN CLEANUP ASSIGNMENTS\n");
    
    for (section_name, difficulty, volunteer_name, skill) in assignments {
        result.push_str(&format!("Section: {} (Difficulty: {})\n", section_name, difficulty));
        result.push_str(&format!("- {} (Skill: {})\n\n", volunteer_name, skill));
    }
    
    // Remove the last newline
    if result.ends_with('\n') {
        result.pop();
    }
    
    result
}
