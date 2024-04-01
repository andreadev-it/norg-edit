#[derive(Debug, Clone, PartialEq, Default)]
pub struct FileChosenEvent {
    pub filename: String,
    pub content: String,
}
