#[allow(dead_code)]
pub enum Action {
    Quit,
    Tick,
    Render,
    Update,
    None,
    EnterUrlInsert,
    EnterNormal,
    CompleteInput(String),
}
