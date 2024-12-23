fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 3 {
        println!("\nUSAGE: {} <temperature> [f|c]", args[0]);
        println!("- f -- from Fahrenheit to Celsius\n- c -- from Celsius to Fahrenheit\n");
        return;
    }

    let temperature: f64 = match args[1].trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Not a number.");
            return;
        }
    };

    let conversion_type = &args[2];
    
    if conversion_type == "f" {
        println!("{}", fah_to_cel(temperature));
    } 
    
    else if conversion_type == "c" {
        println!("{}", cel_to_fah(temperature));
    } 
    
    else {
        println!("Bad argument.");
    }

}


fn fah_to_cel(f: f64) -> f64 {
    (f - 32.0) * 5.0 / 9.0
}


fn cel_to_fah(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}