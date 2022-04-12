use std::process::exit;

use rand::Rng;


fn main() {
  let random_number = rand::thread_rng().gen_range(1..11);
  println!("Guess the number, psst, it's {}", random_number);

  let mut lost = true;
  for _ in 1..11 {
    println!("Please input your guess");

    let mut guess = String::new();

    std::io::stdin()
      .read_line(&mut guess)
      .expect("Failed to read_line");

    let guess = guess.trim();

    if guess == "quit" { exit(0); }

    let guess: u32 = match guess.parse() {
      Ok(num) => num,
      Err(msg) => {
        println!("{}", msg.to_string());
        continue;
      },
    };

    if guess == random_number {
      println!("Correct!");
      lost = false;
      break;
    } else {
      print!("Wrong, ")
    }
  }

  if lost {
    println!("You lose");
  } else {
    println!("You win!");
  }
}
