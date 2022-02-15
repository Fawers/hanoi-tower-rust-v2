#[derive(Debug)]
pub struct Disc(pub usize);

impl Disc {
    pub fn larger_than(&self, d: &Disc) -> bool {
        self.0 > d.0
    }
}

impl ToString for Disc {
    fn to_string(&self) -> String {
        let disc_length = self.0 * 2 - 1;
        format!("{}", "—".repeat(disc_length))
    }
}
