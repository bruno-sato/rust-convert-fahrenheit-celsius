use std::io;

fn main() {
    println!("Convert Fahrenheit to Celsius");
    println!("Please tipe the Fahrenheit temperature: ");

    let mut fah_temp = String::new();

    // Usado para capturar input
    io::stdin()
        .read_line(&mut fah_temp)
        .expect("Failed to read line");

    let fah_temp: f64 = fah_temp.trim().parse().unwrap();

    println!("temp: {}", fah_temp);

    let celsius: f64 = convert_to_celsius(fah_temp);

    println!("The temperature in Celcius is: {:.4}", celsius);
}

fn convert_to_celsius(temp: f64) -> f64 {
    return (temp - 32.0) / 1.8;
}
