use crate::Provider;
use rand::seq::SliceRandom;
use rand::RngCore;

struct SimpleNameProvider {
    names: Vec<&'static str>,
}

impl Provider for SimpleNameProvider {
    fn generate(&self, fake_name: &str, rng: &mut dyn RngCore) -> Option<String> {
        if fake_name != "name" {
            return None;
        }

        self.names.choose(rng).map(|name: &&str| name.to_string())
    }
}
