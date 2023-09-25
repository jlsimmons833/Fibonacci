use std::io;
// use std::cmp::Ordering;
// use rand::Rng;

fn main() {
    
    let mut desiredterm = String::new();
    //let secret_number = rand::thread_rng().gen_range(1..=10);

    //println!("The secret number is: {}", secret_number);

    loop {
        println!("Please enter the Fibonacci term you desire");
        io::stdin()
            .read_line(&mut desiredterm)
            .expect("Failed to read line");
        
        let desiredterm: u32 = match desiredterm.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You entered: {desiredterm}");

        let term: u32 = 0;

        
        break;

        // match guess.cmp(&secret_number) {
        //     Ordering::Less => println!("Too small!"),
        //     Ordering::Greater => println!("Too big!"),
        //     Ordering::Equal => {
        //         println!("You win!");
        //         break;
        //     }
        // }
    }
}
