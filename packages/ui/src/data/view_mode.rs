use std::fmt::Display;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ViewMode {
    Game,
    Score,
    Techs,
    Planets,
    Relics,
    Laws,
    Map,
    ThundersEdge,
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
                ViewMode::Relics => "Relics",
                ViewMode::Laws => "Laws",
                ViewMode::Map => "Map",
                ViewMode::ThundersEdge => "Thunder's Edge",
            }
        )
    }
}
