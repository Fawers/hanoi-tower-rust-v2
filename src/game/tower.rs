use super::disc::Disc;
use TowerPushError::*;

#[derive(Debug)]
pub struct Tower(Vec<Disc>);

impl Tower {
    pub fn new() -> Self {
        Tower(vec![])
    }

    pub fn push(&mut self, d: Disc) -> Result<(), (TowerPushError, Disc)> {
        if self.0.len() >= 5 {
            Err((TowerIsFull, d))
        } else if self.0.last().is_some() && d.larger_than(self.0.last().unwrap()) {
            Err((DiscTooLarge, d))
        } else {
            self.0.push(d);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Option<Disc> {
        self.0.pop()
    }

    pub fn get(&self, i: usize) -> Option<&Disc> {
        self.0.get(i)
    }

    pub fn height(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug)]
pub enum TowerPushError {
    TowerIsFull,
    DiscTooLarge
}