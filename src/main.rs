extern crate clap;
#[macro_use]
extern crate log;
extern crate bs58;
extern crate env_logger;

use clap::{App, Arg};

pub mod engines;

fn main() {
    let m = App::new("codest")
        .version("1.0")
        .about("Easily test a string with many decoding tools")
        .author("Taz <g0latour@gmail.com>")
        .arg(
            Arg::with_name("INPUT")
                .help("The string you want to test")
                .required_unless("supported_list"),
        )
        .arg(
            Arg::with_name("supported_list")
                .short("l")
                .long("list")
                .takes_value(false)
                .help("Display a list of the supported decoder"),
        )
        .arg(
            Arg::with_name("verbosity")
                .short("v")
                .multiple(true)
                .takes_value(false)
                .help(
                    "Sets the level of verbosity (Error[default] > Warning > Info > Debug > Trace)",
                ),
        )
        .arg(
            Arg::with_name("silent")
                .short("s")
                .long("silent")
                .takes_value(false)
                .help("Prevent any log message to be output"),
        )
        .get_matches();

    // fetch the engines inside the scope
    let engines = engines::subscribed();

    // display list and stop if asked
    if m.is_present("supported_list") {
        println!("Supported decoders :");
        for e in &engines {
            println!("{}", e.name());
        }
        return;
    }

    // setup verbosity
    if !m.is_present("silent") {
        let log_level = match m.occurrences_of("verbosity") {
            0 => log::LevelFilter::Error,
            1 => log::LevelFilter::Warn,
            2 => log::LevelFilter::Info,
            3 => log::LevelFilter::Debug,
            4 | _ => log::LevelFilter::Trace,
        };

        env_logger::Builder::new()
            .default_format()
            .filter_level(log_level)
            .init();

        info!("Executing with {} log level", &log_level);
    }

    let input = m.value_of("INPUT").unwrap();

    // TODO : import input from STDIN
    //
    // let input = match m.value_of("INPUT") {
    //     Some(value) => value.to_string(),
    //     None => io::stdin()
    //         .lock()
    //         .fill_buf()
    //         .unwrap()
    //         .iter()
    //         .map(|i| *i as char)
    //         .collect::<String>(),
    // };

    // manage

    let mut output: Vec<String> = Vec::new();
    for e in engines::subscribed() {
        match e.decode(&input) {
            Some(value) => output.push(format!("{:<10} : {}", e.name(), value)),
            None => {}
        }
    }
    println!("Results :");
    for o in output {
        println!("{}", o);
    }
}
