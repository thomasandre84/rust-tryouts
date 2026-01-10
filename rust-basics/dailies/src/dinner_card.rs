/*
Write a function create_dinner_card that takes guest_name, relationship, age_group, dietary_preference and returns a formatted dinner seating card string.

The function creates personalized seating cards for the family reunion dinner party with appropriate titles and special notes based on guest details.

Logic:

Determine title based on relationship: "Uncle/Aunt" for siblings, "Cousin" for cousins, "Grandparent" for parents, "Guest" for others
Add age group designation: "Senior" (65+), "Adult" (18-64), "Youth" (under 18)
Include dietary notes: "Vegetarian Menu", "Gluten-Free Menu", "Standard Menu", or "Special Diet" for others
Add candle snuffer assignment for adults and seniors
Conditions:

If relationship is "sibling": use "Uncle" or "Aunt" title
If age_group is "senior" or "adult": add candle snuffer duty
If dietary_preference is "vegetarian", "gluten-free", or "standard": use specific menu note
Parameters:

guest_name (String): Name of the family member
relationship (String): Family relationship ("sibling", "cousin", "parent", "other")
age_group (String): Age category ("senior", "adult", "youth")
dietary_preference (String): Dietary needs ("vegetarian", "gluten-free", "standard", "other")
Returns: Formatted seating card string. Format: Welcome [Title] [Name] ([Age Group]) Menu: [Dietary Note] [Candle Duty Assignment if applicable]
*/


pub fn create_dinner_card(guest_name: String, relationship: String, age_group: String, dietary_preference: String) -> String {
    // Write code here
    let title = if relationship == "sibling" {
        "Uncle/Aunt"
    } else if relationship == "cousin" {
        "Cousin"
    } else if relationship == "parent" {
        "Grandparent"
    } else {
        "Guest"
    };

    let age_gr = match age_group.as_str() {
        "senior" => "Senior",
        "adult" => "Adult",
        _ => "Youth",
    };

    let dietry_notes = match dietary_preference.as_str() {
        "vegetarian" => "Vegetarian Menu",
        "gluten-free" => "Gluten-Free Menu",
        "standard" => "Standard Menu",
        _ => "Special Diet",
    };

    let appendix = "\nCandle Snuffer Assignment: Yes";

    let mut ret_str = format!("Welcome {title} {guest_name} ({age_gr})\nMenu: {dietry_notes}"); 
    // to append, if condition met: "[Candle Duty Assignment if applicable]"
    if age_group == "senior" || age_group == "adult" {
        ret_str += &(appendix.to_string());
    }
    return ret_str;
}