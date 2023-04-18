use std::io;
fn main() {
    loop{
    let mut buffer = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("please select an op");
     match io::stdin().read_line(&mut buffer){
        Ok(_) => {},
        Err(_) => {
            println!("failed to read line");
            continue;
        }
    }
    let buffer:char = match buffer.trim().parse(){
        Ok(char) => char,
        Err(_) => {
            println!("invalid input");
            continue;
        }
    };

   let op:Operation =  match buffer{
    '+' => Operation::Addition,
    '-' => Operation::Subtraction,
    '*' => Operation::Multiplication,
    '/' => Operation::Division,
    _=> continue
    };

    println!("please enter a number");
    match io::stdin().read_line(&mut num1){
        Ok(_) =>{},
        Err(_) =>{
            println!("failed to read input");
            continue;
        }
    };
    let num1:i32 = match num1.trim().parse(){
        Ok(input) => input,
        Err(_) =>{
            println!("failed to read input");
            continue;
        }
    };
    
    println!("enter a second number");
    match io::stdin().read_line(&mut num2){
        Ok(_) =>{},
        Err(_) =>{
            println!("failed to read input");
            continue;
        }
    };
    let num2:i32 = match num2.trim().parse(){
        Ok(input) => input,
        Err(_) =>{
            println!("failed to read input");
            continue;
        }
    };

    match  op {
        Operation::Addition => println!("{}",add(num1, num2)),
        Operation::Subtraction => println!("{}",sub(num1, num2)),
        Operation::Multiplication => println!("{}",mul(num1, num2)),
        Operation::Division => println!("{}",div(num1, num2))
    };

    println!("do you want to continue?\n press 1 to continue and any other key to exit");

    let mut con = String::new();
    match io::stdin().read_line(&mut con){
        Ok(_) =>{},
        Err(_) =>{
            println!("failed to read input");
            continue;
        }
    };

    let con:i32 = match con.trim().parse(){
        Ok(input) => input,
        Err(_) =>break //we want to break if the input is not one anyways
    };

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

 

