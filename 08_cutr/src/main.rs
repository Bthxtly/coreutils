// NOTE: this implementation of `cut` is different from GNU or BSD cut, that it respect escape
// characters

// TODO: implement -s/--only-delimiter option
// support N-M and -M positions
// support output delimiter, which default to the input delimiter(not supported by GNU `cut`)
// support -n option that will prevent the splitting of multibyte characters(not supported either)
// support --complement option from GNU `cut`

use clap::Parser;
use cutr::Args;

fn main() {
    if let Err(e) = cutr::run(Args::parse()) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
