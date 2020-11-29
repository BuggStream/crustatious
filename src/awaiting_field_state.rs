use crate::{BotBehaviour, BotInstruction, BotState, Orientation};
use std::error::Error;

#[derive(Debug)]
pub struct AwaitingFieldState {
    pub player_id: char,
    pub field_width: u32,
    pub field_height: u32,
}

impl BotBehaviour for AwaitingFieldState {
    fn read_line(&mut self, _line: &str) -> Result<BotInstruction, Box<dyn Error>> {
        Ok(BotInstruction::ToDirection(
            self.player_id,
            Orientation::West,
        ))
    }
}

impl From<AwaitingFieldState> for BotState {
    fn from(state: AwaitingFieldState) -> Self {
        BotState::AwaitingFieldState(state)
    }
}
