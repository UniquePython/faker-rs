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

    pub fn seed(&mut self, seed: u64) {
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::RngCore;

    struct TestProvider {
        fakes: Vec<&'static str>,
    }

    impl Provider for TestProvider {
        fn generate(&self, _fake_name: &str, _rng: &mut dyn RngCore) -> Option<String> {
            None
        }

        fn supported_fakes(&self) -> Vec<&'static str> {
            self.fakes.clone()
        }
    }

    #[test]
    fn add_single_provider_succeeds() {
        let mut faker = Faker::new();

        let provider = Box::new(TestProvider {
            fakes: vec!["name"],
        });

        assert!(faker.add_provider(provider).is_ok());
        assert_eq!(faker.providers.len(), 1);
    }

    #[test]
    fn add_two_non_conflicting_providers_succeeds() {
        let mut faker = Faker::new();

        let provider1 = Box::new(TestProvider {
            fakes: vec!["name"],
        });

        let provider2 = Box::new(TestProvider {
            fakes: vec!["city"],
        });

        assert!(faker.add_provider(provider1).is_ok());
        assert!(faker.add_provider(provider2).is_ok());

        assert_eq!(faker.providers.len(), 2);
    }

    #[test]
    fn add_conflicting_provider_returns_error_and_does_not_add() {
        let mut faker = Faker::new();

        let provider1 = Box::new(TestProvider {
            fakes: vec!["name"],
        });

        let provider2 = Box::new(TestProvider {
            fakes: vec!["name"],
        });

        assert!(faker.add_provider(provider1).is_ok());

        let result = faker.add_provider(provider2);

        assert_eq!(
            result,
            Err(ProviderError::NameConflict {
                fake_name: "name".to_string(),
            })
        );

        assert_eq!(faker.providers.len(), 1);
    }

    #[test]
    fn add_normalized_conflicting_provider_returns_error_and_does_not_add() {
        let mut faker = Faker::new();

        let provider1 = Box::new(TestProvider {
            fakes: vec!["name"],
        });

        let provider2 = Box::new(TestProvider {
            fakes: vec![" Name "],
        });

        assert!(faker.add_provider(provider1).is_ok());

        let result = faker.add_provider(provider2);

        assert_eq!(
            result,
            Err(ProviderError::NameConflict {
                fake_name: "name".to_string(),
            })
        );

        assert_eq!(faker.providers.len(), 1);
    }
}
