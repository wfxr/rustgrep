/*******************************************************************************
*    Author: Wenxuan                                                           *
*     Email: wenxuangm@gmail.com                                               *
*   Created: 2018-05-01 21:40                                                  *
*******************************************************************************/
extern crate minigrep;

use std::env;
use std::process;

use minigrep::Config;
use minigrep::run;

fn main() {
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
        process::exit(1);
    });

    if let Err(err) = run(config) {
        eprintln!("Application error: {}", err);
        process::exit(1);
    }
}
