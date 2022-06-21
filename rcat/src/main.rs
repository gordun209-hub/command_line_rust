fn main() {
if let Err(e) = rcat::run(){
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
