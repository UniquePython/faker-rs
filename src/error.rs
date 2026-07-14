#[derive(Debug, PartialEq)]
pub enum ProviderError {
    NameConflict { fake_name: String },
}
