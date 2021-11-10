use std::path::PathBuf;
use std::collections::HashMap;

pub struct Portfolio {
    path: PathBuf,
    data: HashMap<String, f32>,
}

impl<'a> Portfolio {
    pub fn new(path: PathBuf) -> Self {
        Portfolio { path, data: HashMap::new() }
    }

    pub fn read_from_file(path: PathBuf) -> Self {
        Portfolio::new(path)
    }

    pub fn save(&self) {
        
    }

    pub fn categories_mut (&'a mut self) -> impl Iterator<Item=(&'a String, &'a mut f32)> {
        self.data.iter_mut()
    }

    pub fn add_category(&mut self, category: String, initial_value: f32) {
        self.data.insert(category, initial_value);
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
