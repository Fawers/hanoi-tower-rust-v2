pub mod disc;
pub mod tower;
pub mod hand;

use std::io;

use disc::Disc;
use tower::{Tower, TowerPushError};
use hand::{Hand, DropError};

#[derive(Debug)]
pub struct Game {
    towers: Vec<Tower>,
    hand: Hand,
    moves: u32
}

impl Game {
    pub fn new() -> Self {
        let mut g = Game {
            towers: vec![Tower::new(), Tower::new(), Tower::new()],
            hand: Hand::new(),
            moves: 0
        };

        for d in (1..=5).rev() {
            g.towers[0].push(Disc(d)).expect("disc creation failed; tower broke down");
        }

        g
    }

    pub fn play(&mut self) {
        println!("Welcome to Fawers's Hanoi Tower game!");

        while !self.solved() {
            println!("\nTowers:");
            self.print_towers();
            println!("\nStatus: {}", self.hand.to_string());

            println!("What tower are you {}? (`q` to quit)", match self.hand.empty() {
                true => "taking the disc from",
                false => "dropping the disc onto"
            });

            let tower = match self.get_user_input() {
                UserInput::Tower(t) => t,
                UserInput::Quit => break,
                UserInput::OutOfRange(n) => {
                    println!("Tower {} is out of range.", n);
                    continue;
                },
                UserInput::Error(e) => {
                    eprintln!("error: {}", e);
                    continue;
                }
            };

            if self.hand.empty() {
                if self.hand.grab_from(&mut self.towers[tower]) {
                    println!("Took the disc from tower {}.", tower+1);
                }
                else {
                    println!("Couldn't take a disc from tower {}.", tower+1);
                }
            }
            else {
                match self.hand.drop_onto(&mut self.towers[tower]) {
                    Ok(()) => {
                        println!("Dropped disc onto tower {}.", tower+1);
                        self.moves += 1;
                    },
                    Err(DropError::NothingToDrop) => println!("Nothing to drop."),
                    Err(DropError::CannotDrop(TowerPushError::DiscTooLarge)) => {
                        println!("The disc you want to drop is too large.");
                    },
                    Err(DropError::CannotDrop(TowerPushError::TowerIsFull)) => {
                        println!("The tower can't fit any more discs.");
                    }
                };
            }
        }

        if self.solved() {
            self.print_towers();
            println!("You moved all the discs in {} moves! Congratulations!", self.moves);
        }
        else {
            println!("You quit the game after {} moves. See you later, I guess?", self.moves);
        }
    }

    fn print_towers(&self) {
        for i in (0..5).rev() {
            for t in &self.towers {
                if let Some(d) = t.get(i) {
                    print!("{}{:^9}{} ", size_to_ansicolor(d.0), d.to_string(), "\x1b[0m");
                } else {
                    print!("{:^9} ", "|");
                }
            }
            println!("");
        }

        println!("{:^9} {:^9} {:^9}", 1, 2, 3);
    }

    fn get_user_input(&self) -> UserInput {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't read from stdin");
        let input = input.trim();

        if input == "q" {
            return UserInput::Quit;
        }

        match input.parse::<usize>() {
            Ok(n) if (1..=self.towers.len()).contains(&n) => UserInput::Tower(n-1),
            Ok(n) => UserInput::OutOfRange(n),
            Err(e) => UserInput::Error(format!("couldn't parse `{}`: {}", input, e))
        }
    }

    fn solved(&self) -> bool {
        self.towers[2].height() == 5
    }
}

enum UserInput {
    Tower(usize),
    OutOfRange(usize),
    Quit,
    Error(String)
}

fn size_to_ansicolor(size: usize) -> &'static str {
    match size {
        1 => "\x1b[31m", // red
        2 => "\x1b[32m", // blue
        3 => "\x1b[34m", // green
        4 => "\x1b[35m", // magenta
        5 => "\x1b[36m", // cyan
        6 => "\x1b[33m", // yellow
        _ => ""
    }
}
