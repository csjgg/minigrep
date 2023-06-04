use std::fs;
use std::io;
use std::process;
pub struct Config {
    query: String,
    filename: Option<String>,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("args are not enough");
        }
        let config = Config {
            query: args[1].clone(),
            filename: match args.len() {
                2 => None,
                _ => Some(args[2].clone()),
            },
        };
        return Ok(config);
    }
}

pub fn search(args: Config) -> () {
    println!("Search for '{}'", args.query);
    let contents = match args.filename {
        Some(t) => fs::read_to_string(&t).expect("Something went wrong reading the file"),
        None => read_from_commend().unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(-1);
        }),
    };
    let mut flag = true;
    for line in contents.lines() {
        if line.contains(&args.query) {
            println!("Found : {line}");
            flag = false;
        }
    }
    if flag {
        println!("Not Found");
    }
}

fn read_from_commend() -> Result<String, &'static str> {
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => Ok(input),
        _ => Err("Input wrong"),
    }
}
