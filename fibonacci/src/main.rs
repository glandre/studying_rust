use std::io;

fn main() {
    println!("Generating the n-th fibonacci number");
    
    loop {
        let number: u128 = loop {
            println!("Enter a number positive number");
            let mut input = String::new();
            io::stdin()
                .read_line(&mut input)
                .expect("Failed to read input");

            match input.trim().parse() {
                Ok(num) => {
                    if num > 0 {
                        break num
                    } else {
                        continue
                    }
                },
                Err(_) => continue
            }
        };

        println!("The {}-th fibonacci number is: {}", number, fibonacci(number));
    }
}

fn fibonacci(nth: u128) -> u128 {
    if nth <= 2 {
        return nth - 1;
    }

    let mut n_2 = 0;
    let mut n_1 = 1;
    let mut n = 1;

    for _ in 2..nth {
        n = n_1 + n_2;
        n_2 = n_1;
        n_1 = n;
    }

    n
}
