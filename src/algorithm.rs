use crate::tron::{Field, GameConfiguration, Orientation};
use std::error::Error;

pub fn calculate_direction(_config: &GameConfiguration, history: &Vec<Field>) -> Result<Orientation, Box<dyn Error>> {
    let current_field = history.last().ok_or("Error, no current field state available!")?;

    println!("{:?}", current_field);

    Ok(Orientation::West)
}
