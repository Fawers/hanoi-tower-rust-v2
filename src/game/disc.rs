#[derive(Debug)]
pub struct Disc(pub usize);

impl Disc {
    pub fn larger_than(&self, d: &Disc) -> bool {
        self.0 > d.0
    }

    pub fn to_ansicolor(&self) -> &'static str {
        ["\x1b[31m", "\x1b[32m",
         "\x1b[33m", "\x1b[34m",
         "\x1b[35m", "\x1b[36m"][self.0 % 6]
    }
}

impl std::fmt::Display for Disc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let disc_length = self.0 * 2 - 1;
        write!(f, "{}", "—".repeat(disc_length))
    }
}
