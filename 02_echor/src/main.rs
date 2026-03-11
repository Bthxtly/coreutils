use clap::Parser;

#[derive(Parser)]
#[command(name = "echor")]
#[command(version = "0.1.0")]
#[command(author = "Bthxtly <bthxtly@gmail.com>")]
#[command(about = "Rust echo")]
struct Cli {
    #[arg(value_name = "TEXT")]
    text: Option<Vec<String>>,
    #[arg(short = 'n')]
    #[arg(action = clap::ArgAction::SetTrue)]
    #[arg(help = "Do not print newline")]
    omit_newline: bool,
}

fn main() {
    let cli = Cli::parse();
    let ending = if cli.omit_newline { "" } else { "\n" };

    if let Some(text) = cli.text.as_deref() {
        print!("{}{}", text.join(" "), ending);
    }
}
