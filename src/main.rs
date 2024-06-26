#[cfg(test)]
mod test;

mod randomizer;

use clap::Parser;
use crate::randomizer::{create_simple_password, Sampleable, PasswordDist};


#[derive(Parser)]
#[clap(version = "0.2.0")]
struct Cli {
    #[clap(long)]
    alphanumeric: bool,

    #[clap(short, long, default_value_t=30)]
    length: usize,

    #[clap(short, long)]
    clean: bool
}

fn main() {
    let args = Cli::parse();
    let password: String;
    let prefix: &str = if args.clean {""} else {"Your password: "};

    if args.alphanumeric {
        password = create_simple_password(args.length);
    } else {
        password = PasswordDist.sample(args.length);
    }
    println!("{}{}", prefix, password);
}
