use std::io;

fn main(){
    loop{
    let mut buffer = String::new();
    let mut num1 = String::new();
    let mut num2 = String::new();

    println!("please select an op");
    io::stdin().read_line(&mut buffer).expect("failed to get op");

    println!("please enter a number");
    io::stdin().read_line(&mut num1).expect("failed to get number");
    let num1:i32 = num1.trim().parse().expect("failed to parse");
    
    println!("enter a second number");
    io::stdin().read_line(&mut num2).expect("failed to get message");
    let num2:i32 = num2.trim().parse().expect("failed to parse");

    let op:char = buffer.trim().parse().expect("failed to parse");

    if op == '+'{
        println!("{}", add(num1,num2));
    }
    else if op == '-'{
        println!("{}",sub(num1,num2));
    }
    else if op == '*'{
        println!("{}",mul(num1,num2));
    }
    else if op == '/'{
        println!("{}",div(num1,num2));
    }
    else{
        println!("invalid op");
        break;
    }
}
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
 

