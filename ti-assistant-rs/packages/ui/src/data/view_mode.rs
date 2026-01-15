use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ViewMode {
    Game,
    Score,
    Techs,
    Planets,
    Laws,
    Map,
}

impl Display for ViewMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                ViewMode::Game => "Game",
                ViewMode::Score => "Score",
                ViewMode::Techs => "Techs",
                ViewMode::Planets => "Planets",
                ViewMode::Laws => "Laws",
                ViewMode::Map => "Map",
            }
        )
    }
}
