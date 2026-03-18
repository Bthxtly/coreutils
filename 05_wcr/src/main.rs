// TODO: Write a version that mimics the output from the GNU wc instead of the BSD version:
// * handle --files0-from and --max-line-length
// * deal with word segmentation for non-ascii sentences
fn main() {
    if let Err(e) = wcr::get_args().and_then(wcr::run) {
        eprintln!("{e}");
        std::process::exit(1);
    }
}
