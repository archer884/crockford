fn main() {
    match Command::from_args() {
        Some(Command::Encode(n)) => println!("{}", crockford::encode(n)),
        Some(Command::Decode(n)) => {
            let value = crockford::decode(n).expect("Not a valid Crockford value.");
            println!("{}", value);
        }

        _ => {
            println!("usage: croc encode <positive integer> or croc decode <encoded string>");
            std::process::exit(1);
        }
    }
}

pub enum Command {
    Encode(u64),
    Decode(String),
}

impl Command {
    fn from_args() -> Option<Self> {
        use std::env;

        let mut args = env::args().skip(1);
        match args.next()?.as_ref() {
            "encode" => Some(Command::Encode(args.next()?.parse().ok()?)),
            "decode" => Some(Command::Decode(args.next()?.parse().ok()?)),

            // If the user does not select a subcommand, assume they mean to encode something.
            // If not... Meh.
            other => Some(Command::Encode(other.parse().ok()?)),
        }
    }
}
