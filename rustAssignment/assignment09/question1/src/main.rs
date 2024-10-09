// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(fahrenheit: f64) -> f64 {
    (fahrenheit - 32.0) * 5.0 / 9.0
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut fahrenheit: i64 = 32;

    // Convert the initial Fahrenheit temperature to Celsius
    let celsius = fahrenheit_to_celsius(fahrenheit as f64);
    println!("{}°F is {:.2}°C", fahrenheit, celsius);

    // Use a loop to convert and print the next 5 integer temperatures
    for _ in 1..=5 {
        fahrenheit += 1;
        let celsius = fahrenheit_to_celsius(fahrenheit as f64);
        println!("{}°F is {:.2}°C", fahrenheit, celsius);
    }
}

