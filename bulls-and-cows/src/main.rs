use rand::Rng;
use std::io;

struct Game {
    secret: String,
    bulls: u32,
    cows: u32,
}

impl Game {
    fn new() -> Game {
        let secret = Self::generate_secret();
        Game {
            secret,
            bulls: 0,
            cows: 0,
        }
    }

    fn generate_secret() -> String {
        let mut rng = rand::thread_rng();
        (0..4).map(|_| rng.gen_range(0, 10).to_string()).collect()
    }

    fn check_guess(&mut self, guess: &str) {
        self.bulls = 0;
        self.cows = 0;
        for (i, ch) in guess.chars().enumerate() {
            if ch == self.secret[i..i + 1].parse().unwrap() {
                self.bulls += 1;
            } else if self.secret.contains(ch) {
                self.cows += 1;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();

    loop {
        println!("Enter your guess (4 digits):");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess).expect("Failed to read line");
        guess = guess.trim().to_string();

        game.check_guess(&guess);

        println!("Bulls: {} Cows: {}", game.bulls, game.cows);

        if game.bulls == 4 {
            println!("You win!");
            break;
        }
    }
}

