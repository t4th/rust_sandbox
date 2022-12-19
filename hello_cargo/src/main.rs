use std::io;
use rand::Rng;

fn main()
{
    let word = "string"; //utf8
    let num = 44;

    println!("print non mutable: {}, {}", word, num);

    let mut word2 = String::new();

    println!("write string: ");

    let result = io::stdin()
        .read_line(&mut word2); // & is a reference
        
        result.expect("failed to read a line");

    println!("input: {word2}");

    // use range expression with inclusive bounds: start..=end
    let random_number = rand::thread_rng().gen_range(1..=100);

    println!("random number: {random_number}");
}
