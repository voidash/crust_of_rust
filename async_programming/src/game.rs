use std::{sync::mpsc, thread, io::Write};
use rand::Rng;

pub fn mpsc_guess_the_number_game() {
    const RECEIVED_CORRECT_GUESS: &str = "guessed";
    const RECEIVED_INCORRECT_GUESS: &str = "wrong_guess";

    let (tx,rx) = mpsc::channel();
    let (tx1,rx2) = mpsc::channel();

    let game_loop = thread::spawn(move || {
        let a: u32 = rand::random::<u32>() % 100;
        let random_number= rand::thread_rng().gen_range(0..100);
        println!("random number is {}", random_number);
        loop {
        match rx.recv(){
            Ok(guess) => {
                if guess == random_number {
                    println!(" found the number {}", guess);
                    tx1.send(RECEIVED_CORRECT_GUESS).unwrap();
                    break;
                }else {
                    println!("wrong guess, guess it again");
                    tx1.send(RECEIVED_INCORRECT_GUESS).unwrap();
                }
            }
            Err(err) => {println!("error {} ", err);}
        }
        }
    });

    let player = thread::spawn(move || {
        loop {
            let mut guess_string = String::new();
            std::io::stdout().flush().unwrap();
            std::io::stdin().read_line(&mut guess_string).unwrap();
            
            let guess = match guess_string.trim().parse::<i32>() {
                Ok(num) => num,
                Err(_) => {
                    println!("invalid input, try again");
                    continue;
                }
            };
            match tx.send(guess) {
                Ok(_) => {println!("sent");},
                Err(e) => {println!("{}",e);}
            }

            if let Ok(recv_string) = rx2.recv() {
                if recv_string == RECEIVED_CORRECT_GUESS {
                    println!("received");
                    break;
                }else {
                    println!("incorrect guess received from the channel");
                }
            }
        }
    });

    game_loop.join().unwrap();
    player.join().unwrap();

}