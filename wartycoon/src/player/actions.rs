use strum_macros::EnumIter;

/// Represents player's actions
#[derive(Debug, EnumIter)]
pub enum Action {
    Build,
    Harvest,
    Train,
    Conquer,
    Quit,
}
