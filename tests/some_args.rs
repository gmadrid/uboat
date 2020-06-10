use structopt::StructOpt;
use uboat::Uboat;

#[derive(Debug)]
pub struct Error;

#[derive(StructOpt, Uboat)]
#[uboat]
pub enum Basic {
    CommandOne { a_string: String },
    CommandTwo { an_int: u32, a_string: String },
}

pub fn command_one(a_string: &str) -> Result<(), Error> {
    println!("ONE: {}", a_string);
    Ok(())
}

pub fn command_two(an_int: &u32, a_string: &str) -> Result<(), Error> {
    println!("TWO: an_int: {}, a_string: {}", an_int, a_string);
    Ok(())
}

fn main() -> std::result::Result<(), Error> {
    Basic::dispatch_from_iter(&["binary_name", "command-one", "string_one"])?;
    Basic::dispatch_from_iter(&["binary_name", "command-two", "543", "string_two"])?;
    Ok(())
}
