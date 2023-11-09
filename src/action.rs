#[allow(dead_code)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Quit,
    Tick,
    Render,
    Update,
    None,
    EnterUrlInsert,
    EnterNormal,
    CompleteInput(String),
    Resize(u16, u16),
}
