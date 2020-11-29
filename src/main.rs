use crustatious::Bot;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut bot = Bot::new();
    bot.run()?;

    Ok(())
}
