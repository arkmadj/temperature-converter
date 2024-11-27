use regex::Regex;
use std::io;

fn main() {
    println!("Enter the temperate in Celsius or Fahrenheit: (e.g.,  32C or -25.3F)");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input = input.trim();

    let re = Regex::new(r"(?P<temp>-?\d+(\.\d+)?)(?P<unit>[CF])").unwrap();

    let Some(caps) = re.captures(input) else {
        println!("Invalid input");
        return;
    };

    let temperature = &caps["temp"];
    let temperature: f64 = match temperature.trim().parse() {
        Ok(temp) => temp,
        Err(_) => {
            println!("Invalid temperature.");
            return;
        }
    };

    let unit = &caps["unit"];

    if unit == "C" {
        println!(
            "The temperature is {:.2}F",
            convert_to_fahrenheit(temperature)
        );
    } else if unit == "F" {
        println!("The temperature is {:.2}C", convert_to_celsius(temperature));
    }
}

fn convert_to_celsius(temp: f64) -> f64 {
    5.0 / 9.0 * (temp - 32.0)
}

fn convert_to_fahrenheit(temp: f64) -> f64 {
    1.8 * temp + 32.0
}
