use std:: io;
use std::cmp::Ordering;
use rand::Rng;
use colored::{self, Colorize};
fn main() {
    println!("Gusss the Number");

    let secreate_number = rand::thread_rng().gen_range(1..= 101);
    println!("secreate number {}", secreate_number);
      loop{
          println!("Please enter the Number you want to guess");

           let mut guess = String::new();
           io::stdin().read_line(&mut guess).expect("failled to read input");

           let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
           };

            println!("your guessed number {}", guess);

            match guess.cmp(&secreate_number) {
           Ordering::Less => println!("{}","Too small.".red()),
           Ordering::Greater => println!("{}","Too big".red()),
            Ordering::Equal => {
                println!("{}","you win".green());
                break;
            },
           }
         }



        }
