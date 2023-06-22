use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        println!("Invalid input...");
        print_help();
        return;
    }

    let source_temeperature = &args[1].trim();
    let temp_unit = source_temeperature
        .chars()
        .last()
        .unwrap()
        .to_ascii_uppercase();

    let (value, unit) = match temp_unit {
        'C' => {
            // with the explicit unit: C -> convert C to F
            let source_temeperature = &source_temeperature[0..source_temeperature.len() - 1];
            let source_temeperature: i32 = parse_or_panic(source_temeperature);
            (source_temeperature, 'C')
        },
        'F' => {
            // with the explicit unit: F -> convert F to C
            let source_temeperature = &source_temeperature[0..source_temeperature.len() - 1];
            let source_temeperature: i32 = parse_or_panic(source_temeperature);
            (source_temeperature, 'F')
        },
        _ => {
            // a number -> default to unit to F
            let source_temeperature: i32 = parse_or_panic(source_temeperature);

            (source_temeperature, 'F')
        }
    };

    match unit {
        'C' => println!("{}F", celsius_to_fahrenheit(value)),
        'F' => println!("{}C", fahrenheit_to_celsius(value)),
        _ => print_help()
    }


}

fn celsius_to_fahrenheit(value_in_celsius: i32) -> i32 {
    (value_in_celsius * 9 / 5) + 32
}

fn fahrenheit_to_celsius(value_in_fahrenheit: i32) -> i32 {
    (value_in_fahrenheit - 32) * 5 / 9
}

fn parse_or_panic(string: &str) -> i32 {
    string
        .trim()
        .parse()
        .expect("Invalid temperature. Expected a number")
}

fn print_help() {
    println!("Usage: ftocelsius <source_temeperature>");
    println!("Examples:");
    println!("# Convert Fahrenheit to Celsius:");
    println!("> ftocelsius 32");
    println!("Output: 0C");
    println!("> ftocelsius 32F");
    println!("Output: 0C");
    println!("# Convert Fahrenheit to Celsius:");
    println!("> ftocelsius 0C");
    println!("Output: 32F");
}
