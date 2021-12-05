use std::fmt::Display;
use std::ops::{Deref, DerefMut};

#[derive(Copy, Clone, Debug)]
pub struct Currency(pub f32);

impl Deref for Currency {
    type Target = f32;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for Currency {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Display for Currency {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<Currency> for f32 {
    fn from(c: Currency) -> Self {
        c.0
    }
}
