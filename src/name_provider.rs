use crate::Provider;
use rand::seq::SliceRandom;
use rand::RngCore;

const FIRST_NAMES_RAW: &str = include_str!("../data/first_names.txt");

#[derive(Debug)]
pub struct NameProvider {
    names: Vec<&'static str>,
}

impl NameProvider {
    pub fn new() -> Self {
        NameProvider {
            names: FIRST_NAMES_RAW.lines().filter(|name| *name != "").collect(),
        }
    }
}

impl Provider for NameProvider {
    fn generate(&self, fake: &str, rng: &mut dyn RngCore) -> Option<String> {
        if fake != "name" {
            return None;
        }

        self.names.choose(rng).map(|name| name.to_string())
    }

    fn supported_fakes(&self) -> Vec<&'static str> {
        vec!["name"]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    #[test]
    fn generates_a_known_name() {
        let provider = NameProvider::new();

        let mut rng = StdRng::seed_from_u64(42);

        let result = provider.generate("name", &mut rng);

        assert!(result.is_some());

        let name = result.unwrap();
        assert!(provider.names.contains(&name.as_str()));
    }

    #[test]
    fn returns_none_for_unknown_fake_name() {
        let provider = NameProvider::new();

        let mut rng = StdRng::seed_from_u64(42);

        let result = provider.generate("bogus", &mut rng);

        assert_eq!(result, None);
    }
}
