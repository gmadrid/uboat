use structopt::StructOpt;
use uboat::Uboat;

#[derive(Debug)]
pub struct Error;

pub fn command_one() -> Result<(), Error> {
    println!("ONE");
    Ok(())
}

pub fn command_two() -> Result<(), Error> {
    println!("TWO");
    Ok(())
}

#[derive(StructOpt, Uboat)]
#[uboat]
pub enum Basic {
    CommandOne,
    CommandTwo,
}

fn main() -> std::result::Result<(), Error> {
    Basic::dispatch_from_iter(&["binary_name", "command-one"])?;
    Basic::dispatch_from_iter(&["binary_name", "command-two"])?;
    Ok(())
}
