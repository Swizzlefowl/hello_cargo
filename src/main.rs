use std::io;

pub fn fun(x:u32){
  if x == 10 {
    println!("your guess was correct");
}
else{
 println!("your guess was not correct")
}
}
 fn main() {
   println!("Enter a num:");
   let mut guess = String::new();

   io::stdin().read_line(&mut guess).expect("failed to readline");
   let guess:u32 = guess.trim().parse().expect("enter a number");
  
  fun(guess);
}


