use super::disc::Disc;
use super::tower::{Tower, TowerPushError};
use DropError::*;
use GrabError::*;

#[derive(Debug)]
pub struct Hand(Option<Disc>);

impl Hand {
    pub fn new() -> Self {
        Hand(None)
    }

    pub fn grab_from(&mut self, t: &mut Tower) -> Result<(), GrabError> {
        let od = self.0.as_ref().map_or_else(
            || t.pop().map_or_else(
                || Err(EmptyTower),
                |d| Ok(d)
            ),
            |_| Err(DiscAlreadyInHand)
        );

        od.map(|d| self.0 = Some(d))
    }

    pub fn drop_onto(&mut self, t: &mut Tower) -> Result<(), DropError> {
        self.0.take().map_or_else(
            || Err(NothingToDrop),
            |d| t.push(d).or_else(|(tower_error, d)| {
                self.0 = Some(d);
                Err(CannotDrop(tower_error))
            })
        )
    }

    pub fn empty(&self) -> bool {
        self.0.is_none()
    }
}

impl std::fmt::Display for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match &self.0 {
            Some(d) => format!("Hand is holding disc of size {}", d.0),
            None => "Hand is empty".to_string()
        };

        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum DropError {
    NothingToDrop,
    CannotDrop(TowerPushError)
}

#[derive(Debug)]
pub enum GrabError {
    DiscAlreadyInHand,
    EmptyTower
}
