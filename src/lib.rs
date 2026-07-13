use rand::RngCore;

/// A Provider knows how to generate values for one or more "fakes"
/// (e.g. "first_name", "city", "email").
pub trait Provider {
    /// Attempt to generate a value for the given fake name.
    /// Returns `None` if this provider doesn't recognize the name,
    /// so the caller can try other providers.
    fn generate(&self, fake_name: &str, rng: &mut dyn RngCore) -> Option<String>;
}
