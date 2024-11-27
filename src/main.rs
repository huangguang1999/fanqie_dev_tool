use clap::{Arg, ArgMatches, Command};
use urlencoding::{decode, encode};
use std::env;

fn main() {
	let args: Vec<String> = env::args().collect();
    let command_name = if args.get(0).map(|s| s.ends_with("fanqie_dev_tool")).unwrap_or(false) {
        "fanqie_dev_tool"
    } else {
        "fq"
    };
    let matches = Command::new(command_name)
        .version("1.0")
        .author("huangguang <1226399454@qq.com>")
        .about("Developer's tool for urlencode!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("encode")
                .about("URL-encode a string")
                .arg(Arg::new("input").help("String to encode").required(true)),
        )
        .subcommand(
            Command::new("decode")
                .about("URL-decode a string")
                .arg(Arg::new("input").help("String to decode").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encode", sub_matches)) => url_encode(sub_matches),
        Some(("decode", sub_matches)) => url_decode(sub_matches),
        _ => unreachable!(),
    }
}

fn url_encode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", encode(input));
    }
}

fn url_decode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
        println!("{}", decode(input).unwrap());
    }
}