use std::io;

fn main() {
    loop{
    let mut buffer = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("please select an op");
    io::stdin().read_line(&mut buffer).expect("failed to get op");
    let buffer:char = buffer.trim().parse().expect("failed to parse");

   let op:Operation =  match buffer{
    '+' => Operation::Addition,
    '-' => Operation::Subtraction,
    '*' => Operation::Multiplication,
    '/' => Operation::Division,
    _=> continue
    };

    println!("please enter a number");
    io::stdin().read_line(&mut num1).expect("failed to get number");
    let num1:i32 = num1.trim().parse().expect("failed to parse");
    
    println!("enter a second number");
    io::stdin().read_line(&mut num2).expect("failed to get message");
    let num2:i32 = num2.trim().parse().expect("failed to parse");

    match  op {
        Operation::Addition => println!("{}",add(num1, num2)),
        Operation::Subtraction => println!("{}",sub(num1, num2)),
        Operation::Multiplication => println!("{}",mul(num1, num2)),
        Operation::Division => println!("{}",div(num1, num2))
    };

    println!("do you want to continue?, press 1 to continue and any other key to exit");

    let mut con = String::new();
    io::stdin().read_line(&mut con).expect("failed to get input");

    let con:i32 = con.trim().parse().expect("failed to parse");

    if con == 1 {
        continue;
    }
    else {
        break;
    };
}
}

pub enum Operation{
    Addition,
    Subtraction,
    Multiplication,
    Division
}

fn add(x:i32, y:i32) ->i32{
    x+y
}

fn sub(x:i32, y:i32) ->i32{
    x-y
}

fn mul(x:i32, y:i32) ->i32{
    x*y
}

fn div(x:i32, y:i32) ->i32{
    x/y
}
 

