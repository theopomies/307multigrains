pub enum Commands {
    Program(Arguments),
    Help,
}

pub struct Arguments {
    n1: u32,
    n2: u32,
    n3: u32,
    n4: u32,
    po: u32,
    pw: u32,
    pc: u32,
    pb: u32,
    ps: u32,
}

const TOTAL_ARGS: usize = 9;

impl Commands {
    pub fn try_from_args() -> Result<Commands, String> {
        let mut args = std::env::args().skip(1);
        if args.any(|arg| arg == "-h" || arg == "--help") {
            return Ok(Commands::Help);
        }
        let args = std::env::args()
            .skip(1)
            .map(|arg| arg.parse::<u32>())
            .collect::<Result<Vec<_>, _>>();
        match args {
            Err(e) => Err(e.to_string()),
            Ok(args) if args.len() == TOTAL_ARGS => Ok(Commands::Program(Arguments {
                n1: args[0],
                n2: args[1],
                n3: args[2],
                n4: args[3],
                po: args[4],
                pw: args[5],
                pc: args[6],
                pb: args[7],
                ps: args[8],
            })),
            Ok(args) => Err(format!(
                "Invalid arguments count {}, expected {}.",
                args.len(),
                TOTAL_ARGS
            )),
        }
    }
}
