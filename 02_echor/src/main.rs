// TODO: support interpretation of backslash escapes:
// - \ backslash
// - \a alert (BEL)
// - \b backspace
// - \c produce no further output
// - \e escape
// - \f form feed
// - \n new line
// - \r carriage return
// - \t horizontal tab
// - \v vertical tab
// - \0NNN byte with octal value NNN (1 to 3 digits)
// - \xHH byte with hexadecimal value HH (1 to 2 digits)

fn main() {
    if let Err(e) = echor::run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
