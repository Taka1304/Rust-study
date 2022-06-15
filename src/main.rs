use rand::Rng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    loop {
        println!("[Select Gamemode]");
        println!("1: Guessing Game");
        println!("2: Hit and brow");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        let guess: u32 = match guess.trim().parse() { 
            Ok(num) => num,
            Err(_) => continue,
        };
        match guess {
            1 => guessing_game(),
            2 => hit_brow(),
            _ => {
                println!("See you!");
                break;
            }
        }
    }
}
fn guessing_game() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..101);   //乱数の生成、1..=100も同義

    // println!("The secret number is: {}",secret_number);  
    loop {
        println!("Please input your guess. (0 ~ 100)");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");    //.readlineが失敗した時の処理

        let guess: u32 = match guess.trim().parse() {   //trim()で改行文字や空白を消す
            Ok(num) => num,                             //parseの結果
            Err(_) => continue,
        };
        
        println!("You guessed:{}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
fn hit_brow () {
    // Answer
    let mut ans = ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
    let mut rng = thread_rng();
    ans.shuffle(&mut rng);
    let ans = &ans[0..4];
    // println!("{:?}",ans);
    let mut score = 0;
    println!("Hit and brow");
    println!("Please input your guess. (0～9)");
    println!("example: 4 0 9 2");
    
    loop{
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");

        let guess: Vec<&str> = guess.split_whitespace().collect();
        if guess.len() != 4 {
            println!("Incomplete input.");
            println!("Please imitate the example.");
            continue;
        }
        score += 1;
        let mut res: [&str; 4] = [""; 4];
        for i in 0..guess.len() {
            if guess[i] == ans[i] {
                res[i] = "◎ ";
            } else if ans.iter().any(|&a| a == guess[i]) {
                res[i] = "△ ";
            } else {
                res[i] = "✕ ";
            }
        }
        println!("{:?}", &res);
        if res.iter().all(|&r| r == "◎ ") {
            println!("You win!!");
            println!("Your score is {} point\n", score);
            break;
        }
    }
}