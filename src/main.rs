use std::process::exit;

mod args;
mod rational;

use args::Commands;

use crate::rational::Rational;

const HELP_MESSAGE: &str = "USAGE
\t./307multigrains n1 n2 n3 n4 po pw pc pb ps

DESCRIPTION
\tn1\tnumber of tons of fertilizer F1
\tn2\tnumber of tons of fertilizer F2
\tn3\tnumber of tons of fertilizer F3
\tn4\tnumber of tons of fertilizer F4
\tpo\tprice of one unit of oat
\tpw\tprice of one unit of wheat
\tpc\tprice of one unit of corn
\tpb\tprice of one unit of barley
\tps\tprice of one unit of soy";

fn main() {
    match Commands::try_from_args() {
        Err(e) => {
            eprintln!("{}", HELP_MESSAGE);
            eprintln!("{}", e);
            exit(84)
        }
        Ok(Commands::Help) => {
            println!("{}", HELP_MESSAGE);
            exit(0)
        }
        Ok(Commands::Program(_arguments)) => {}
    }
}
