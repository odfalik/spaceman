use std::fs::File;
use std::io::Read;
use clap::Clap;

mod request;
use request::Req;

mod requester;
use requester::run;

#[derive(Clap)]
#[clap(version = "0.1", author = "Oded Falik <mail@odedfalik.com>")]
struct Opts {
    input: String,
    // #[clap(subcommand)]
    // subcmd: SubCommand,
}

fn main() {
    let opts: Opts = Opts::parse();
    println!("🚀 {}", opts.input);

    let mut file = File::open(opts.input).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let _json = serde_json::from_str(&contents);
    if _json.is_err() {
        _json.unwrap();
        // eprintln!("Error parsing input file: {:?}", _json.unwrap_err());
        return;
    }
    let req: Vec<Req> = _json.unwrap();
    run(req);
}
