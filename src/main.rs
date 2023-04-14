use std::io;
use rand::{self, thread_rng, Rng};
fn main(){
    let rnum: u32 = thread_rng().gen_range(1..=100);

    loop{
    let mut num = String::new();
    println!("eneter a number");

    io::stdin().read_line(&mut num).expect("failed to read input");

    let num:u32 = num.trim().parse().expect("failed to get num");

    if num == rnum{
        println!("good job");
        break;
    }
    else if num > rnum{
        println!("your guess is bigger then the number")
    }
    else if num < rnum{
        println!("your guess is smaller then the number")
    }
    else{
        println!("you guessed incorrectly\n the num was: {rnum}");
    }
}
}

