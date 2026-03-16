// TODO: implement handling numeric values with suffixes and negative values (-c=1K, -n=-3)
fn main() {
    if let Err(e) = headr::get_args().and_then(headr::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
