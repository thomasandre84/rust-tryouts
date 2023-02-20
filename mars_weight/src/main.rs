use std::io;

fn main() {
    println!("Please enter your weight in kg:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    // println!("Your input was: {}", input);
    let weight: f32 = input.trim().parse().unwrap();

    let mars_weight = calc_mars_weight(weight);
    println!("Weight on Mars: {} in kg", mars_weight);
}

fn calc_mars_weight(weight: f32) -> f32 {
    weight / 9.81 * 3.711 
}
