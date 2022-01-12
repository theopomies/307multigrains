pub enum Commands {
    Program(Arguments),
    Help,
}

pub struct Arguments {
    pub n1: u64,
    pub n2: u64,
    pub n3: u64,
    pub n4: u64,
    pub po: u64,
    pub pw: u64,
    pub pc: u64,
    pub pb: u64,
    pub ps: u64,
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
            .map(|arg| arg.parse::<u64>())
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
