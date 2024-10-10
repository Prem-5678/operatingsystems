// Declare a constant for the freezing point of water in Fahrenheit
const FREEZING_POINT_FAHRENHEIT: f64 = 32.0;

// Function to convert Fahrenheit to Celsius
fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_FAHRENHEIT) * 5.0 / 9.0
}

// Function to convert Celsius to Fahrenheit
fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_FAHRENHEIT
}

fn main() {
    // Declare a mutable variable with a temperature in Fahrenheit
    let mut fahrenheit_temp: f64 = 32.0;

    // Convert the initial temperature to Celsius and print the result
    let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
    println!("{:.2}°F is {:.2}°C", fahrenheit_temp, celsius_temp);

    // Loop to convert and print the next 5 integer temperatures (F to C)
    for _ in 0..5 {
        fahrenheit_temp += 1.0;
        let celsius_temp = fahrenheit_to_celsius(fahrenheit_temp);
        println!("{:.2}°F is {:.2}°C", fahrenheit_temp, celsius_temp);
    }

    // Use the celsius_to_fahrenheit function to convert a temperature back to Fahrenheit
    let celsius_value: f64 = 0.0;
    let fahrenheit_value = celsius_to_fahrenheit(celsius_value);
    println!("{:.2}°C is {:.2}°F", celsius_value, fahrenheit_value);
}
