#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CurrentView {
    #[default]
    Processes,
    Charts,
}
