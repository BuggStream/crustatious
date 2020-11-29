use crate::awaiting_field_state::AwaitingFieldState;
use crate::initialization::Initialization;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::{stdin, stdout, Stdin, Stdout, Write};

mod awaiting_field_state;
mod initialization;

#[derive(Debug)]
pub enum BotInstruction {
    ToState(BotState),
    ToDirection(char, Orientation),
}

pub trait BotBehaviour {
    fn read_line(&mut self, line: &str) -> Result<BotInstruction, Box<dyn Error>>;
}

#[derive(Debug)]
pub enum BotState {
    Initialization(Initialization),
    AwaitingFieldState(AwaitingFieldState),
}

impl BotState {
    fn read_line(&mut self, line: &str) -> Result<BotInstruction, Box<dyn Error>> {
        match self {
            BotState::Initialization(state) => state.read_line(line),
            BotState::AwaitingFieldState(state) => state.read_line(line),
        }
    }
}

#[derive(Debug)]
pub struct Bot {
    state: BotState,
    stdin: Stdin,
    stdout: Stdout,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            state: BotState::Initialization(Initialization),
            stdin: stdin(),
            stdout: stdout(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            let mut line = String::new();
            stdin().read_line(&mut line)?;

            let line = line.trim();

            if line == "STOP" {
                break
            }

            match self.state.read_line(line)? {
                BotInstruction::ToState(state) => {
                    // println!("Previous state: {:?}", self.state);
                    self.state = state;
                    // println!("Next state: {:?}", self.state);
                }
                BotInstruction::ToDirection(player_id, orientation) => {
                    self.stdout
                        .write_all(format!("{}:{}\n", player_id, orientation).as_bytes())?;
                }
            };
        }

        Ok(())
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
