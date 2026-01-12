/*
Write a function generate_and_validate_item_codes that takes item_name, price, code1, code2 and returns a string containing the generated item code and validation result.

The function generates a unique shop item code using hash-based encoding and validates if two existing codes represent equivalent products.

Logic:

Calculate a simple hash of the item name (sum of ASCII values)
Check if the price is a perfect square (special number property)
Generate item code: format as "ITEM_{hash}_{price}_{S}" where S is "SPECIAL" if price is perfect square, "REGULAR" otherwise
Validate equivalence: two codes are equivalent if they have the same hash and price components (ignore the special/regular suffix)
Parameters:

item_name (String): Name of the shop item
price (i32): Price of the item in cents
code1 (String): First item code to compare
code2 (String): Second item code to compare
Returns: Generated item code and validation result. Format: ITEM_{hash}_{price}_{type}|{equivalent} where equivalent is "EQUIVALENT" or "DIFFERENT"
 */

fn is_perfect_square(n: i32) -> bool {
    if n < 2 {
        return true;
    }

    let mut left = 1;
    let mut right = n / 2;

    while left <= right {
        let mid = (left + right) / 2;
        let sq = mid * mid;

        if sq == n {
            return true;
        } else if sq < n {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    false
}

fn are_equivalent(a: String, b: String) -> bool {
    let v1: Vec<&str> = a.split('_').collect();
    let v2: Vec<&str> = b.split('_').collect();

    if v1[1] == v2[1] && v1[2] == v2[2] {
        return true;
    }
    return false;
}
fn generate_and_validate_item_codes(item_name: String, price: i32, code1: String, code2: String) -> String {
    // Write code here
    let hash: u32 = item_name.bytes().map(|b| b as u32).sum();
    let price_type = if is_perfect_square(price) {
        "SPECIAL"
    } else {
        "REGULAR"
    };
    let equivalent = if are_equivalent(code1, code2){
        "EQUIVALENT"
    } else {
        "DIFFERENT"
    };
    format!("ITEM_{hash}_{price}_{price_type}|{equivalent}")
}
