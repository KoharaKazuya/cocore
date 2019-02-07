use clap::{App, Arg};
use cocore::{convert, Representation};
use std::io;
use std::io::prelude::*;
use std::process::exit;

fn main() {
    let matches = App::new("cocore")
        .version("0.1.0")
        .about("converts color representation such as HSL colors and RGB colors")
        .author("KoharaKazuya")
        .arg(
            Arg::with_name("to")
                .long("to")
                .value_name("representation")
                .help("color representation cocore converts into")
                .possible_values(&["hex", "rgb", "hsl"])
                .default_value("hex")
                .takes_value(true),
        )
        .arg(Arg::with_name("expression").multiple(true))
        .get_matches();

    let to = matches.value_of("to").unwrap();
    let representation = match to {
        "rgb" => Representation::RGB,
        "hsl" => Representation::HSL,
        _ => Representation::Hex,
    };

    let mut exit_status = 0;

    macro_rules! convert_and_print {
        ($e:expr) => {
            match convert($e, representation) {
                Ok(converted) => {
                    println!("{}", converted);
                }
                Err(err) => {
                    eprintln!("{}", err);
                    exit_status = 1;
                }
            }
        };
    }

    let is_tty = unsafe { libc::isatty(0) == 1 };
    if !is_tty {
        let stdin = io::stdin();
        for ret in stdin.lock().lines() {
            match ret {
                Ok(line) => convert_and_print!(&line),
                Err(err) => {
                    eprintln!("{}", err);
                    exit_status = 1;
                }
            }
        }
    }

    let arg_expression = matches
        .values_of("expression")
        .unwrap_or_default()
        .collect::<Vec<_>>()
        .join(" ");
    if arg_expression != "" {
        convert_and_print!(&arg_expression)
    }

    exit(exit_status);
}
