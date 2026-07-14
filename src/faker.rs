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
        let new_fakes: Vec<(String, String)> = provider
            .supported_fakes()
            .iter()
            .map(|name| ((*name).to_string(), Self::normalize(name)))
            .collect();

        for existing in &self.providers {
            for existing_fake in existing.supported_fakes() {
                let normalized_existing = Self::normalize(existing_fake);

                if let Some((original, _)) = new_fakes
                    .iter()
                    .find(|(_, normalized)| *normalized == normalized_existing)
                {
                    return Err(ProviderError::NameConflict {
                        fake_name: original.clone(),
                    });
                }
            }
        }

        self.providers.push(provider);
        Ok(())
    }

    pub fn generate(&mut self, fake: &str) -> Option<String> {
        let fake = Self::normalize(fake);

        for provider in &self.providers {
            if let Some(value) = provider.generate(&fake, &mut self.rng) {
                return Some(value);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::providers::SimpleNameProvider;
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
                fake_name: " Name ".to_string(),
            })
        );

        assert_eq!(faker.providers.len(), 1);
    }

    #[test]
    fn generate_known_fake_returns_value() {
        let mut faker = Faker::new();
        faker.seed(42);

        faker
            .add_provider(Box::new(SimpleNameProvider::new(vec![
                "Alice", "Bob", "Charlie",
            ])))
            .unwrap();

        let result = faker.generate("name");

        assert!(result.is_some());

        let name = result.unwrap();
        assert!(["Alice", "Bob", "Charlie"].contains(&name.as_str()));
    }

    #[test]
    fn generate_unknown_fake_returns_none() {
        let mut faker = Faker::new();

        faker
            .add_provider(Box::new(SimpleNameProvider::new(vec![
                "Alice", "Bob", "Charlie",
            ])))
            .unwrap();

        assert_eq!(faker.generate("bogus"), None);
    }

    #[test]
    fn same_seed_produces_same_sequence() {
        let mut faker1 = Faker::new();
        faker1.seed(42);
        faker1
            .add_provider(Box::new(SimpleNameProvider::new(vec![
                "Alice", "Bob", "Charlie",
            ])))
            .unwrap();

        let mut faker2 = Faker::new();
        faker2.seed(42);
        faker2
            .add_provider(Box::new(SimpleNameProvider::new(vec![
                "Alice", "Bob", "Charlie",
            ])))
            .unwrap();

        for _ in 0..10 {
            assert_eq!(faker1.generate("name"), faker2.generate("name"));
        }
    }
}
