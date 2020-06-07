extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Ident};

use proc_macro2::TokenStream as TokenStream2;

#[derive(Debug)]
struct Commands<'a> {
    elem_name: &'a Ident,
    commands: Vec<CommandDesc<'a>>,
}    

#[derive(Debug)]
struct CommandDesc<'a> {
    name: &'a Ident,
}

fn extract_commands(input: &DeriveInput) -> Commands {
//    dbg!(input);
    if let Data::Enum(data_enum) = &input.data {
//	dbg!(data_enum);
	let commands = data_enum.variants.iter().map(|variant| {
	    CommandDesc { name: &variant.ident }
	}).collect();
	Commands { elem_name: &input.ident, commands }
    } else {
	panic!("Uboat only allowed on enums.");
    }
}

fn make_match_arms(commands: Commands) -> TokenStream2 {
    let elem_name = commands.elem_name;
    let arms = commands.commands.iter().map(|cmd| {
	let name = cmd.name;
	quote! { #elem_name::#name{} => { Ok(())  } }
    });
    quote! {
	#(
	    #arms
	)*
    }
}

fn make_dispatch_func(commands: Commands) -> TokenStream2 {
    let elem_name = commands.elem_name;
    let match_arms = make_match_arms(commands);
    quote! {
	fn dispatch() -> std::result::Result<(),Error> {
	    //	    let sc = Foobar::from_args();
	    let sc = #elem_name::from_args();
	    match sc {
		#match_arms
	    }
	}
    }
}

#[proc_macro_derive(Uboat, attributes(uboat))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let commands = extract_commands(&input);

    let elem_name = commands.elem_name;
    let dispatch_func = make_dispatch_func(commands);

    let q = quote! {
	impl #elem_name {
	    #dispatch_func
	}
    };
    q.into()
}

/*

  Usage:

  mod structopt_module {
    #[derive(Debug,StructOpt,Subcommands)]
    #[structopt(name = "name", about = "Something about 'name'")]
    enum Subcommands {
      Command1 {
        pub arg1: bool,
        pub arg2: bool,
      },
      #[uboat(action=command2_action)]
      Command2 {
      },
      // ... more commands.
    }
  }

  mod actions {
    pub fn command1(args: structopt_module::Subcommands::Command1) // ...
    pub fn command2_action(args: structopt_module::Subcommands::Command2) // ...
  }

  Then you can call:

  Subcommands::dispatch();

  dispatch will look something like this:

  fn dispatch() -> Result<()> {
    let sc = Subcommands::from_args();
    match sc {
      arg @ Command1 => { command1(arg)?; }
      arg @ Command2 => { command2_action(arg)?; }
    }
  }
*/


#[test]
fn trybuild() {
    let t = trybuild::TestCases::new();
    t.pass("tests/basic.rs");
}
