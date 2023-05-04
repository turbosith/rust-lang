use std::cmp::Ordering;
use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is: {secret_number}");
    loop {//зацикливание
        println!("Please input your guess.");
        let mut guess = String::new();

        io::stdin()//ввод числа
            .read_line(&mut guess)
            .expect("Failed to read line");

       let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("Your guessed: {guess}");
        match guess.cmp(&secret_number) {//вывод результата
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
