use clap::{Parser, ValueEnum};
mod coc6;

#[derive(Parser)]
struct Cli {
    #[arg(short, long, default_value = "coc6")]
    rule: Rule,
}
#[derive(Debug, Clone, ValueEnum)]
enum Rule {
    Coc6,
    Coc7,
}

fn main() {
    let args = Cli::parse();

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
