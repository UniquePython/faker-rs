#[derive(Debug)]
pub enum ProviderError {
    NameConflict { fake_name: String },
}
