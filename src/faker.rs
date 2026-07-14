use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::name_provider::NameProvider;

pub struct Faker {
    pub(crate) rng: StdRng,
    pub(crate) name: NameProvider,
}

impl Faker {
    pub fn new() -> Self {
        Faker {
            rng: StdRng::from_entropy(),
            name: NameProvider::new(),
        }
    }

    pub fn seed(&mut self, seed: u64) {
        self.rng = StdRng::seed_from_u64(seed);
    }
}
