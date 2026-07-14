use rand::seq::SliceRandom;
use rand::Rng;
use rand::RngCore;

use crate::faker::Faker;

const MALE_FIRST_NAMES_RAW: &str = include_str!("../data/male_first_names.txt");
const FEMALE_FIRST_NAMES_RAW: &str = include_str!("../data/female_first_names.txt");
const LAST_NAMES_RAW: &str = include_str!("../data/last_names.txt");

#[derive(Debug, Clone)]
pub(crate) struct NameProvider {
    male_first_names: Vec<&'static str>,
    female_first_names: Vec<&'static str>,
    last_names: Vec<&'static str>,
}

impl NameProvider {
    fn parse_names(names: &str) -> Vec<&str> {
        names.lines().filter(|name| *name != "").collect()
    }

    pub(crate) fn new() -> Self {
        NameProvider {
            male_first_names: Self::parse_names(MALE_FIRST_NAMES_RAW),
            female_first_names: Self::parse_names(FEMALE_FIRST_NAMES_RAW),
            last_names: Self::parse_names(LAST_NAMES_RAW),
        }
    }

    fn male_first_name(&self, rng: &mut dyn RngCore) -> String {
        self.male_first_names
            .choose(rng)
            .expect("male_first_names is empty")
            .to_string()
    }

    fn female_first_name(&self, rng: &mut dyn RngCore) -> String {
        self.female_first_names
            .choose(rng)
            .expect("female_first_names is empty")
            .to_string()
    }

    fn first_name(&self, rng: &mut dyn RngCore) -> String {
        if rng.gen_bool(0.5) {
            self.male_first_name(rng)
        } else {
            self.female_first_name(rng)
        }
    }

    fn last_name(&self, rng: &mut dyn RngCore) -> String {
        self.last_names
            .choose(rng)
            .expect("last_names is empty")
            .to_string()
    }

    fn full_name(&self, rng: &mut dyn RngCore) -> String {
        format!("{} {}", self.first_name(rng), self.last_name(rng))
    }
}

/// A NameFaker knows how to generate various kinds of names.
pub trait NameFaker {
    /// Generate a random male first name.
    fn male_first(&mut self) -> String;

    /// Generate a random female first name.
    fn female_first(&mut self) -> String;

    /// Generate a random first name.
    /// The returned name may be either male or female.
    fn first(&mut self) -> String;

    /// Generate a random last name.
    fn last(&mut self) -> String;

    /// Generate a random full name.
    /// Typically consists of a first name followed by a last name.
    fn full(&mut self) -> String;
}

impl NameFaker for Faker {
    fn male_first(&mut self) -> String {
        self.name.male_first_name(&mut self.rng)
    }

    fn female_first(&mut self) -> String {
        self.name.female_first_name(&mut self.rng)
    }

    fn first(&mut self) -> String {
        self.name.first_name(&mut self.rng)
    }

    fn last(&mut self) -> String {
        self.name.last_name(&mut self.rng)
    }

    fn full(&mut self) -> String {
        self.name.full_name(&mut self.rng)
    }
}
