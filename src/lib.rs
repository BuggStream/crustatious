use crate::algorithm::calculate_direction;
use crate::tron::{Field, GameConfiguration};
use std::convert::TryInto;
use std::error::Error;
use std::io::{stdin, stdout, Write};

mod algorithm;
mod tron;

#[derive(Debug)]
pub struct Bot {
    history: Vec<Field>,
}

impl Bot {
    pub fn new() -> Self {
        Bot {
            history: Vec::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        let stdin = stdin();
        let mut stdout = stdout();

        let mut line = String::new();
        stdin.read_line(&mut line)?;
        let config: GameConfiguration = line.trim().try_into()?;

        loop {
            let mut line = String::new();
            stdin.read_line(&mut line)?;
            let line = line.trim();

            if line == "STOP" {
                break;
            }

            let field: Field = line.try_into()?;
            self.history.push(field);
            let orientation = calculate_direction(&config, &self.history)?;

            stdout.write_all(format!("{}:{}\n", config.player_id, orientation).as_bytes())?;
        }

        Ok(())
    }
}
