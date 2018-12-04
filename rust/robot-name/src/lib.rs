extern crate rand;

use rand::distributions::Uniform;
use rand::{thread_rng, Rng};

pub struct Robot {
    name: String,
}

impl Robot {
    pub fn new() -> Self {
        Robot {
            name: Self::generate_name(),
        }
    }

    fn generate_name() -> String {
        let letter_part = thread_rng()
            .sample_iter(&Uniform::new(b'A', b'Z'))
            .take(2)
            .map(|c| c as char)
            .collect::<String>();
        letter_part + "100"
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset_name(&mut self) {
        self.name = Self::generate_name();
    }
}
