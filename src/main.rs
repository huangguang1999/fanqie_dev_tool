use clap::{Arg, ArgMatches, Command};
use urlencoding::{decode, encode};
use std::env;
use clipboard::{ClipboardContext, ClipboardProvider};

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
        .subcommand(
            Command::new("lower")
                .about("to lowercase")
                .arg(Arg::new("input").help("to lowercase").required(true)),
        )
        .subcommand(
            Command::new("upper")
                .about("to uppercase")
                .arg(Arg::new("input").help("to uppercase").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("encode", sub_matches)) => url_encode(sub_matches),
        Some(("decode", sub_matches)) => url_decode(sub_matches),
        Some(("lower", sub_matches)) => string_to_lowercase(sub_matches),
        Some(("upper", sub_matches)) => string_to_uppercase(sub_matches),
        _ => unreachable!(),
    }
}

fn copy_to_clipboard(content: &str) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    ctx.set_contents(content.to_owned()).unwrap();
}

fn url_encode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
		let encoded_url = encode(input);
        println!("{}", encoded_url);
        copy_to_clipboard(&encoded_url);
    }
}

fn url_decode(matches: &ArgMatches) {
    if let Some(input) = matches.get_one::<String>("input") {
		let decoded_url = decode(input).unwrap();
        println!("{}", decoded_url);
        copy_to_clipboard(&decoded_url);
    }
}

fn string_to_lowercase(matches: &ArgMatches) {
	if let Some(input) = matches.get_one::<String>("input") {
        let lowercase = input.to_lowercase();
        println!("{}", lowercase);
        copy_to_clipboard(&lowercase);
    }
}

fn string_to_uppercase(matches: &ArgMatches) {
	if let Some(input) = matches.get_one::<String>("input") {
        let uppercase = input.to_uppercase();
        println!("{}", uppercase);
        copy_to_clipboard(&uppercase);
    }
}