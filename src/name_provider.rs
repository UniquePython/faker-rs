use crate::Provider;
use rand::seq::SliceRandom;
use rand::RngCore;

const MALE_FIRST_NAMES_RAW: &str = include_str!("../data/male_first_names.txt");
const FEMALE_FIRST_NAMES_RAW: &str = include_str!("../data/female_first_names.txt");
const LAST_NAMES_RAW: &str = include_str!("../data/last_names.txt");

#[derive(Debug)]
pub struct NameProvider {
    male_first_names: Vec<&'static str>,
    female_first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
}

impl NameProvider {
    fn parse_names(names: &str) -> Vec<&str> {
        names.lines().filter(|name| *name != "").collect()
    }

    pub fn new() -> Self {
        NameProvider {
            male_first_names: Self::parse_names(MALE_FIRST_NAMES_RAW),
            female_first_names: Self::parse_names(FEMALE_FIRST_NAMES_RAW),
            last_names: Self::parse_names(LAST_NAMES_RAW),
        }
    }

    pub fn male_first_name(&self, rng: &mut dyn RngCore) -> String {
        self.male_first_names
            .choose(rng)
            .expect("male_first_names is empty")
            .to_string()
    }

    pub fn female_first_name(&self, rng: &mut dyn RngCore) -> String {
        self.female_first_names
            .choose(rng)
            .expect("female_first_names is empty")
            .to_string()
    }

    pub fn first_name(&self, rng: &mut dyn RngCore) -> String {
        if rng.next_u32() % 2 == 0 {
            self.male_first_name(rng)
        } else {
            self.female_first_name(rng)
        }
    }

    pub fn last_name(&self, rng: &mut dyn RngCore) -> String {
        self.last_names
            .choose(rng)
            .expect("last_names is empty")
            .to_string()
    }

    pub fn full_name(&self, rng: &mut dyn RngCore) -> String {
        format!("{} {}", self.first_name(rng), self.last_name(rng))
    }
}

impl Provider for NameProvider {
    fn generate(&self, fake: &str, rng: &mut dyn RngCore) -> Option<String> {
        match fake {
            "name" => Some(self.full_name(rng)),
            _ => None,
        }
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
        let mut parts = name.split_whitespace();

        let first = parts.next().unwrap();
        let last = parts.next().unwrap();

        assert!(parts.next().is_none()); // exactly two parts

        assert!(
            provider.male_first_names.contains(&first)
                || provider.female_first_names.contains(&first)
        );

        assert!(provider.last_names.contains(&last));
    }

    #[test]
    fn returns_none_for_unknown_fake_name() {
        let provider = NameProvider::new();

        let mut rng = StdRng::seed_from_u64(42);

        let result = provider.generate("bogus", &mut rng);

        assert_eq!(result, None);
    }
}
