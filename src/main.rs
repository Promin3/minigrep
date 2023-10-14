use std::env;
use std::process::exit;
use minigrep::Config;
fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err|{
        // eprintln!(...); 标准输出流 cargo run > output.txt
       println!("Problem parsing arguments: {}", err);
        exit(1);
    });


    if let Err(e) = minigrep::run(config){
        // eprint!(...)
        println!("Application error: {}", e);
        exit(1);
    }
}
