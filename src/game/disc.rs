#[derive(Debug)]
pub struct Disc(pub usize);

impl Disc {
    pub fn larger_than(&self, d: &Disc) -> bool {
        self.0 > d.0
    }
}

impl std::fmt::Display for Disc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disc_length = self.0 * 2 - 1;
        write!(f, "{}", "—".repeat(disc_length))
    }
}
