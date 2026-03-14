// TODO: support more options:
//   -A, --show-all
//          equivalent to -vET
//   -e     equivalent to -vE
//   -E, --show-ends
//          display $ or ^M$ at end of each line
//   -s, --squeeze-blank
//          suppress repeated empty output lines
//   -t     equivalent to -vT
//   -T, --show-tabs
//          display TAB characters as ^I
//   -v, --show-nonprinting
//          use ^ and M- notation, except for LFD and TAB

fn main() {
    if let Err(e) = catr::get_args().and_then(catr::run) {
        eprintln!("{}", e);
    }
}
