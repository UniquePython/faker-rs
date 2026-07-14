use rand::rngs::StdRng;
use rand::SeedableRng;

use crate::error::ProviderError;
use crate::Provider;

pub struct Faker {
    providers: Vec<Box<dyn Provider>>,
    rng: StdRng,
}

impl Faker {
    pub fn new() -> Self {
        Faker {
            providers: vec![],
            rng: StdRng::from_entropy(),
        }
    }

    pub fn seed(&mut self, seed: u64) -> () {
        self.rng = StdRng::seed_from_u64(seed);
    }

    fn normalize(name: &str) -> String {
        name.trim().to_lowercase()
    }

    pub fn add_provider(&mut self, provider: Box<dyn Provider>) -> Result<(), ProviderError> {
        let new_fakes: Vec<String> = provider
            .supported_fakes()
            .iter()
            .map(|name| Self::normalize(name))
            .collect();

        for existing in &self.providers {
            for existing_fake in existing.supported_fakes() {
                let existing_fake = Self::normalize(existing_fake);

                if new_fakes.contains(&existing_fake) {
                    return Err(ProviderError::NameConflict {
                        fake_name: existing_fake,
                    });
                }
            }
        }

        self.providers.push(provider);
        Ok(())
    }
}
