use explore_rust::process_number;

fn main() {
    println!("Hello, world!");
    
    let number = 9;
    let sequence = process_number(number);
    
    print!("{}", number);
    for value in sequence {
        print!(" --> {}", value);
    }
    println!();
    println!("The end!");
}
