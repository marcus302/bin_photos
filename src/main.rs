use std::io;

fn main() {
    let num_classes = 5;
    let mut classes = vec![""; num_classes];
    let mut class = String::new();
    for iter in 0..num_classes { 
        io::stdin()
            .read_line(&mut class)
            .expect("Failed to read line.");
        classes[iter] = class.trim();
        let mut class = String::new();
    }
    println!("{:?}", classes);
}