use structopt::StructOpt;
use uboat::Uboat;

#[derive(Debug)]
pub struct Error;

#[derive(StructOpt, Uboat)]
#[uboat]
pub enum Basic {
    CommandOne,
    CommandTwo,
}

fn main() -> std::result::Result<(), Error> {
    // Capture the error, because we've provided no arguments.
    Basic::dispatch()?;
    
    Ok(())
}
