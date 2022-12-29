use std::{io, cmp::Ordering};
use rand::Rng;

fn main()
{
    println!("guess a num: ");
    let random_number = rand::thread_rng().gen_range(1..=100);

    loop
    {
        let mut input = String::new();
        let result = io::stdin()
            .read_line(&mut input); // & is a reference
        
        result.expect("failed to read a line");

        // remove whitespaces, parse to number
        let number:i32 = input.trim().parse().expect("value is not a number!");

        
        match random_number.cmp(&number)
        {
            Ordering::Greater =>{ println!("greater");},
            Ordering::Less => { println!("smaller");},
            Ordering::Equal => {println!("equal"); break},
        }
    }
}
