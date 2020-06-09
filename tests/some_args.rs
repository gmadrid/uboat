use structopt::StructOpt;
use uboat::Uboat;

#[derive(Debug)]
pub struct Error;

#[derive(StructOpt, Uboat)]
#[uboat]
pub enum Basic {
    CommandOne,
    CommandTwo { a_string: String },
}

pub fn command_one() -> Result<(), Error> {
    println!("ONE");
    Ok(())
}

pub fn command_two(a_string: &str) -> Result<(), Error> {
    println!("TWO: {}", a_string);
    Ok(())
}

fn main() -> std::result::Result<(), Error> {
    Basic::dispatch_from_iter(&["binary_name", "command-one"])?;
    Basic::dispatch_from_iter(&["binary_name", "command-two", "a_string_value"])?;
    Ok(())
}
