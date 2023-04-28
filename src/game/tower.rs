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
        } else if self.0.last().map_or(false, |e| d.larger_than(e)) {
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

    pub fn lines(&self) -> impl Iterator<Item=String> {
        (0..5).rev()
            .map(|i| self.0.get(i))
            .map(|od| od.map_or(    format!("{:^9} ", "|"),
                                |d| format!("{}{:^9}{} ", d.to_ansicolor(), d.to_string(), "\x1b[0m")))
            .collect::<Vec<_>>()
            .into_iter()
    }
}

#[derive(Debug)]
pub enum TowerPushError {
    TowerIsFull,
    DiscTooLarge
}