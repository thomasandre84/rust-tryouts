fn generate_event_code(participant_name: String, beer_type: String, cheese_pairing: String, participant_number: i32) -> String {
    // Calculate custom hash: sum all character ASCII values from name, beer type, and cheese pairing, then take modulo 1000
    let mut hash_sum = 0;
    
    for ch in participant_name.chars() {
        hash_sum += ch as u32;
    }
    
    for ch in beer_type.chars() {
        hash_sum += ch as u32;
    }
    
    for ch in cheese_pairing.chars() {
        hash_sum += ch as u32;
    }
    
    let hash = hash_sum % 1000;
    
    // Calculate flavor compatibility score: multiply lengths of beer_type and cheese_pairing strings, then take modulo 100
    let score = (beer_type.len() * cheese_pairing.len()) % 100;
    
    // Format as "BT2024-HASH{hash}-P{participant_number:02}-FS{score:02}"
    format!("BT2024-HASH{:03}-P{:02}-FS{:02}", hash, participant_number, score)
}
