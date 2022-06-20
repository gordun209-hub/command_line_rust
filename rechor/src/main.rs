use clap::{App, Arg};

fn main() {
    let matches = App::new("echor")
        .version("0.1.0")
        .author("Ken Youens-Clark <kyclark@gmail.com>")
        .about("Rust echo")
        .arg(
            Arg::with_name("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .min_values(1),
        )
        .arg(
            Arg::with_name("omit_newline")
                .short("n")
                .help("Do not print newline")
                .takes_value(false),
        )
        .get_matches();
    // that returns Vec inside Option so unwrapping this gives
    //  vec
    let text = matches.values_of_lossy("text").unwrap();
    // gives bool custom code here
    let omit_newline = match matches.is_present("omit_newline") {
        true => " ",
        false => "\n",
    };
    // program output
    print!("{}{}", text.join(" "), omit_newline);
}
