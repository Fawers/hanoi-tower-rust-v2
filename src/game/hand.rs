use super::disc::Disc;
use super::tower::{Tower, TowerPushError};
use DropError::*;

#[derive(Debug)]
pub struct Hand(Option<Disc>);

impl Hand {
    pub fn new() -> Self {
        Hand(None)
    }

    pub fn grab_from(&mut self, t: &mut Tower) -> bool {
        if let Some(d) = t.pop() {
            self.0 = Some(d);
            return true;
        }

        false
    }

    pub fn drop_onto(&mut self, t: &mut Tower) -> Result<(), DropError> {
        if let Some(d) = self.0.take() {
            match t.push(d) {
                Ok(()) => Ok(()),
                Err((tower_error, d)) => {
                    self.0 = Some(d);
                    Err(CannotDrop(tower_error))
                }
            }
        } else {
            Err(NothingToDrop)
        }
    }

    pub fn empty(&self) -> bool {
        self.0.is_none()
    }
}

impl ToString for Hand {
    fn to_string(&self) -> String {
        let s = match &self.0 {
            Some(d) => format!("Hand is holding disc of size {}", d.0),
            None => "Hand is empty".to_string()
        };

        format!("{}", s)
    }
}

#[derive(Debug)]
pub enum DropError {
    NothingToDrop,
    CannotDrop(TowerPushError)
}
