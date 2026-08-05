#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum CurrentView {
    #[default]
    Processes,
    Charts,
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub enum Order {
    #[default]
    Asc,
    Desc,
}
