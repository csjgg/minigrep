use minigrep::Config;
use std::env;
use std::process;
fn main() {
    let arg: Vec<String> = env::args().collect();
    let args = Config::build(&arg).unwrap_or_else(|err| {
        println!("Exit with '{err}'");
        process::exit(-1);
    });
    minigrep::search(args);
}
