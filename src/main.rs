use clap::{Args, Parser, Subcommand, ValueEnum};
mod coc6;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    #[command(about = "Create a character sheet")]
    Char(CharArgs),
}

#[derive(Args)]
struct CharArgs {
    #[arg(short, long, default_value = "coc6")]
    rule: Rule,
}

#[derive(Debug, Clone, ValueEnum)]
enum Rule {
    Coc6,
    Coc7,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Command::Char(args) => run_new_char(args),
    }
}

fn run_new_char(args: CharArgs) {
    match args.rule {
        Rule::Coc6 => run_coc6(),
        Rule::Coc7 => todo!(),
    }
}

fn run_coc6() {
    println!("==== CoC6 ====");
    let sheet = coc6::Sheet::new();
    sheet.print_statuses();
}
