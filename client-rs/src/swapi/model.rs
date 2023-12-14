
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct SwapiPeople {
    pub name: String,
    pub height: String,
    pub mass: String,
    pub hair_color: String,
    pub skin_color: String,
    pub eye_color: String,
    pub birth_year: String,
    pub gender: String,
    pub homeworld: String,
}