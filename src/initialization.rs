use crate::awaiting_field_state::AwaitingFieldState;
use crate::{BotBehaviour, BotInstruction, BotState};
use std::error::Error;

#[derive(Debug)]
pub struct Initialization;

impl BotBehaviour for Initialization {
    fn read_line(&mut self, line: &str) -> Result<BotInstruction, Box<dyn Error>> {
        let setup_data: Vec<_> = line.split("|").collect();
        // println!("{:?}", setup_data);
        let player_id = setup_data.get(0).ok_or("no player id")?.parse::<char>()?;
        let field_width = setup_data.get(1).ok_or("no field width")?.parse::<u32>()?;
        let field_height = setup_data.get(2).ok_or("no field height")?.parse::<u32>()?;

        let next_state = AwaitingFieldState {
            player_id,
            field_width,
            field_height,
        };

        Ok(BotInstruction::ToState(next_state.into()))
    }
}

impl From<Initialization> for BotState {
    fn from(state: Initialization) -> Self {
        BotState::Initialization(state)
    }
}
