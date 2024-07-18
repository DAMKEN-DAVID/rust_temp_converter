const FREEZING_POINT: f64 = 32.0;

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * (9.0/5.0) + FREEZING_POINT
}

fn main() {
    println!("Freezing Point: {}", FREEZING_POINT);
    let temperatures: [i32; 5] = [33, 34, 35, 36, 37];

    for &temp in temperatures.iter() {
        let temp_fahrenheit = celsius_to_fahrenheit(temp as f64);
        println!("{}°C = {}°F", temp, temp_fahrenheit);
    }
}