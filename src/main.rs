use std::io;

fn main() {
    loop{
        let mut temperature: String = String::new();
        
        println!("Type temperature in celsius please: ");

        io::stdin().read_line(&mut temperature)
            .expect("Failed to read line!");

        let temperature: f64 = match temperature.trim()
            .parse(){
                Ok(num) => num,
                Err(_) => {
                    "Please type a number, not string!";
                    continue
                }
            };
        
        let fahrenheit: f64 = celsius_to_fahrenheit(temperature);

        println!("Temperature in fahrenheit degrees is: {fahrenheit}");
    }
}

fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    temperature * 9.0/5.0 + 32.0
}