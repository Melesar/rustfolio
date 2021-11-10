use super::currency::Currency;
use std::collections::HashMap;

pub struct Portfolio {
    data: HashMap<String, Currency>,
}

impl<'a> Portfolio {
    pub fn new() -> Self {
        Portfolio { data: HashMap::new() }
    }

    pub fn data_mut (&'a mut self) -> impl Iterator<Item=(&'a String, &'a mut Currency)> {
        self.data.iter_mut()
    }

    pub fn categories(&'a self) -> impl Iterator<Item=&'a String> {
        self.data.keys()
    }

    pub fn values(&self) -> impl Iterator<Item=&Currency> {
        self.data.values()
    }

    pub fn add_category(&mut self, category: String, initial_value: f32) {
        self.data.insert(category, Currency(initial_value));
    }
}

impl std::fmt::Display for Portfolio {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        for (category, value) in self.data.iter() {
            write!(f, "{} - {}\n", category, value)?;
        }

        Ok(())
    }
}
