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
        let mut term: u32 = 2;
        let mut fibonacci_term2: u32 = 2;
        let mut fibonacci_term1: u32 = 1;
        while term < desiredterm {
            let fibonacci_oldterm1 = fibonacci_term1;
            fibonacci_term1 = fibonacci_term2;
            fibonacci_term2 = fibonacci_oldterm1 + fibonacci_term2;
            term = term +1;
        }

        println!("The {} term in the Fibonacci sequence is {}", desiredterm, fibonacci_term2);
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
