use crate::components::planet::Planet;

pub type GDResult<T> = Result<T, GameDataError>;

#[derive(thiserror::Error, Debug)]
pub enum GameDataError {
    #[error("Planet ({planet:?}) is in more than 1 ({num_systems}) systems")]
    PlanetInMoreThanOneSystems { planet: Planet, num_systems: usize },
}
