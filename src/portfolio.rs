use std::collections::HashMap;
use std::collections::hash_map::Keys;

pub struct Portfolio {
    data: HashMap<String, f32>,
}

impl<'a> Portfolio {
    pub fn categories_mut (&'a mut self) -> impl Iterator<Item=(&'a String, &'a mut f32)> {
        self.data.iter_mut()
    }

    pub fn update_category(&mut self, category: &str, value: f32) {

    }
}
