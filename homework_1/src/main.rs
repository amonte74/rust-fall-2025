
const FREEZING_POINT_F: f64 = 32.0;

fn fahrenheit_to_celsius(f: f64) -> f64 {
    (f - FREEZING_POINT_F) * 5.0 / 9.0
}

fn celsius_to_fahrenheit(c: f64) -> f64 {
    (c * 9.0 / 5.0) + FREEZING_POINT_F
}


fn main() {
    let mut temp_f: f64 = 32.0; // starting temperature in Fahrenheit
    println!("{:.1}°F = {:.1}°C", temp_f, fahrenheit_to_celsius(temp_f));

    // Convert and print next 5 integer temperatures
    for _ in 0..5 {
        temp_f += 1.0;
        println!("{:.1}°F = {:.1}°C", temp_f, fahrenheit_to_celsius(temp_f));
    }
}
