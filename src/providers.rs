use crate::Provider;
use rand::seq::SliceRandom;
use rand::RngCore;

pub struct SimpleNameProvider {
    names: Vec<&'static str>,
}

impl SimpleNameProvider {
    pub fn new(names: Vec<&'static str>) -> Self {
        SimpleNameProvider { names }
    }
}

impl Provider for SimpleNameProvider {
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
        let provider = SimpleNameProvider::new(vec!["Alice", "Bob", "Charlie"]);

        let mut rng = StdRng::seed_from_u64(42);

        let result = provider.generate("name", &mut rng);

        assert!(result.is_some());

        let name = result.unwrap();
        assert!(provider.names.contains(&name.as_str()));
    }

    #[test]
    fn returns_none_for_unknown_fake_name() {
        let provider = SimpleNameProvider::new(vec!["Alice", "Bob", "Charlie"]);

        let mut rng = StdRng::seed_from_u64(42);

        let result = provider.generate("bogus", &mut rng);

        assert_eq!(result, None);
    }
}
