fn update_boutique_inventory(products: Vec<String>, mut stock_counts: Vec<i32>, product_name: String, change_amount: i32) -> String {
    // Find the product and update its stock count
    for (i, product) in products.iter().enumerate() {
        if *product == product_name {
            stock_counts[i] += change_amount;
            break;
        }
    }
    
    // Create the formatted inventory string
    let mut result = Vec::new();
    for (i, product) in products.iter().enumerate() {
        result.push(format!("{}. {}: {}", i + 1, product, stock_counts[i]));
    }
    
    result.join(", ")
}
