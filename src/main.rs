use std::process::exit;

mod args;
mod rational;
mod tableau;

use args::{Arguments, Commands};
use tableau::TableauBuilder;

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

const CONSTRAINTS: [[u64; 5]; 4] = [
    [1, 0, 1, 0, 2],
    [1, 2, 0, 1, 0],
    [2, 1, 0, 1, 0],
    [0, 0, 3, 1, 2],
];

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
        Ok(Commands::Program(arguments)) => {
            if let Err(e) = program(arguments) {
                eprintln!("{}", e);
                exit(84)
            }
        }
    }
}

fn program(arguments: Arguments) -> Result<(), String> {
    let tableau = TableauBuilder::new(&[
        arguments.po,
        arguments.pw,
        arguments.pc,
        arguments.pb,
        arguments.ps,
    ])
    .add_constraint(&CONSTRAINTS[0], arguments.n1)?
    .add_constraint(&CONSTRAINTS[1], arguments.n2)?
    .add_constraint(&CONSTRAINTS[2], arguments.n3)?
    .add_constraint(&CONSTRAINTS[3], arguments.n4)?
    .get_tableau();
    Ok(())
}
