use std::convert::{TryFrom, TryInto};
use std::fmt::{Formatter, Display};
use std::error::Error;

#[derive(Debug)]
pub struct GameConfiguration {
    pub player_id: char,
    pub field_width: u32,
    pub field_height: u32,
}

impl TryFrom<&str> for GameConfiguration {
    type Error = Box<dyn Error>;

    fn try_from(line: &str) -> Result<Self, Self::Error> {
        let split: Vec<_> = line.split("|").collect();
        let player_id = split.get(0).ok_or("no player id")?.parse::<char>()?;
        let field_width = split.get(1).ok_or("no field width")?.parse::<u32>()?;
        let field_height = split.get(2).ok_or("no field height")?.parse::<u32>()?;

        Ok(GameConfiguration {
            player_id,
            field_width,
            field_height,
        })
    }
}

#[derive(Debug)]
pub enum FieldElement {
    Empty,
    Wall,
    Death,
    PlayerHead(char),
    PlayerTail(char),
}

impl TryFrom<char> for FieldElement {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let element = match value {
            '.' => FieldElement::Empty,
            '#' => FieldElement::Wall,
            '*' => FieldElement::Death,
            head if head.is_ascii_alphabetic() && head.is_ascii_uppercase() => {
                FieldElement::PlayerHead(head)
            }
            tail if tail.is_ascii_alphabetic() && tail.is_ascii_lowercase() => {
                FieldElement::PlayerTail(tail)
            }
            _ => Err("Invalid field element!")?,
        };

        Ok(element)
    }
}

#[derive(Debug)]
pub struct Field {
    data: Vec<FieldElement>,
}

impl TryFrom<&str> for Field {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let mut data = Vec::new();

        for element in value.chars() {
            let parsed_element: FieldElement = element.try_into()?;
            data.push(parsed_element);
        }

        Ok(Field { data })
    }
}

#[derive(Debug)]
pub enum Orientation {
    North,
    East,
    South,
    West,
}

impl Display for Orientation {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Orientation::North => write!(f, "north"),
            Orientation::East => write!(f, "east"),
            Orientation::South => write!(f, "south"),
            Orientation::West => write!(f, "west"),
        }
    }
}
