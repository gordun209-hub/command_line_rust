fn main() {
    if let Err(e) = rcat::get_args().and_then(rcat::run) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
