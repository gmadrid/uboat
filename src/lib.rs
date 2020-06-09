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
    fields: Vec<&'a Ident>,
}

fn extract_commands(input: &DeriveInput) -> Commands {
    if let Data::Enum(data_enum) = &input.data {
        let commands = data_enum
            .variants
            .iter()
            .map(|variant| {
                //dbg!(variant);
                let fields = variant.fields.iter().map(|f| {
                    // This only works with named struct variants.
                    f.ident.as_ref().unwrap()
                }).collect();
                CommandDesc {
                    name: &variant.ident,
                    fields,
                }
            })
            .collect();
        Commands {
            elem_name: &input.ident,
            commands,
        }
    } else {
        panic!("Uboat only allowed on enums.");
    }
}

fn make_match_arms(commands: Commands) -> TokenStream2 {
    use heck::SnakeCase;

    let arms = commands.commands.iter().map(|cmd| {
        let name = cmd.name;
        let fields = &cmd.fields;
        let func_name = Ident::new(&name.to_string().to_snake_case(), name.span());
        quote! { Self::#name{ #(#fields),* } => crate::#func_name(#(#fields),*), }
    });

    quote! {
    #(
        #arms
    )*
    }
}

fn make_dispatch_func(commands: Commands) -> TokenStream2 {
    let match_arms = make_match_arms(commands);
    quote! {
        fn dispatch_self(&self) -> std::result::Result<(), Error> {
            match self {
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
    trait UboatCaptain {
        fn dispatch_from_iter<I>(iter: I) -> std::result::Result<(), Error>
          where Self: Sized + structopt::StructOpt, I:IntoIterator, I::Item: Into<std::ffi::OsString> + Clone {
            Self::dispatch_self(&Self::from_iter(iter))
        }
        fn dispatch() -> std::result::Result<(), Error>
          where Self: Sized + structopt::StructOpt {
            Self::dispatch_self(&Self::from_args())
        }
        fn dispatch_self(&self) -> std::result::Result<(), Error>;
    }

    impl UboatCaptain for #elem_name {
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
    t.pass("tests/*.rs");
}
