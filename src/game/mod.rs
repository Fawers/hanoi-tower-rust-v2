pub mod disc;
pub mod tower;
pub mod hand;

use std::io;

use disc::Disc;
use tower::{Tower, TowerPushError::*};
use hand::{Hand, DropError, GrabError};


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

    pub fn default_read_input(&self) -> impl FnMut() -> UserInput {
        read_input(self.towers.len())
    }

    pub fn play<F>(&mut self, mut read_input: F)
    where F: FnMut() -> UserInput
    {
        println!("Welcome to Fawers's Hanoi Tower game!");

        while !self.solved() {
            println!("\nTowers:");
            self.print_towers();
            println!("\nStatus: {}", self.hand);

            println!("What tower are you {}? (`q` to quit)",
                     if self.hand.empty() { "taking the disc from" }
                     else                 { "dropping the disc onto" });

            let tower = match read_input() {
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

            match self.hand.grab_from(&mut self.towers[tower]) {
                Ok(_) => println!("Took disc from tower {}", tower+1),
                Err(GrabError::EmptyTower) => println!("Can't take discs from empty tower."),
                Err(GrabError::DiscAlreadyInHand) => match self.hand.drop_onto(&mut self.towers[tower]) {
                    Ok(_) => {
                        println!("Dropped disc onto tower {}.", tower+1);
                        self.moves += 1;
                    },
                    Err(DropError::NothingToDrop) => println!("Nothing to drop."),
                    Err(DropError::CannotDrop(DiscTooLarge)) => {
                        println!("The disc you want to drop is too large.");
                    },
                    Err(DropError::CannotDrop(TowerIsFull)) => {
                        println!("The tower can't fit any more discs.");
                    }
                }
            };
        }

        if self.solved() {
            self.print_towers();
            println!("\nYou moved all the discs in {} moves! Congratulations!", self.moves);
        }
        else {
            println!("You quit the game after {} moves. See you later, I guess?", self.moves);
        }
    }

    fn print_towers(&self) {
        let mut lines_vec = self.towers.iter().map(Tower::lines).collect::<Vec<_>>();

        for _ in 0..5 {
            for lines in &mut lines_vec {
                print!("{}", lines.next().unwrap());
            }
            println!("");
        }

        println!("{:^9} {:^9} {:^9}", 1, 2, 3);
    }

    fn solved(&self) -> bool {
        self.towers[2].height() == 5
    }
}

    
fn read_input(num_towers: usize) -> impl FnMut() -> UserInput {
    move || {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Can't read from stdin");
        let input = input.trim();
    
        if input == "q" {
            return UserInput::Quit;
        }
    
        match input.parse::<usize>() {
            Ok(n) if (1..=num_towers).contains(&n) => UserInput::Tower(n-1),
            Ok(n) => UserInput::OutOfRange(n),
            Err(e) => UserInput::Error(format!("couldn't parse `{}`: {}", input, e))
        }
    }
}

pub enum UserInput {
    Tower(usize),
    OutOfRange(usize),
    Quit,
    Error(String)
}
