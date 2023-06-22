fn main() {
    let x = five();

    another_function(x);
    print_labeled_measurement(x, 'h');

    let x = plus_one(x);
    another_function(x);
}

fn another_function(x: i32) {
    println!("The value of x is {x}");
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {value}{unit_label}");
}

fn five() -> i32 {
    5
}

/* header
*/
fn plus_one(x: i32) -> i32 {
    x + 1
}