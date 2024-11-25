use clap::{Arg, ArgMatches, Command};
use chrono::{Local, TimeZone};
use urlencoding::{decode, encode};

fn main() {
    let matches = Command::new("fanqie_dev_tool")
        .version("1.0")
        .author("huangguang <1226399454@qq.com>")
        .about("Developer's tool for urlencode and time format!")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new("urlencode")
                .about("URL-encode a string")
                .arg(Arg::new("input").help("String to encode").required(true)),
        )
        .subcommand(
            Command::new("urldecode")
                .about("URL-decode a string")
                .arg(Arg::new("input").help("String to decode").required(true)),
        )
        .subcommand(
            Command::new("timestamp")
                .about("Convert a UNIX timestamp to local datetime")
                .arg(Arg::new("timestamp").help("UNIX timestamp").required(true)),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("urlencode", sub_matches)) => url_encode(sub_matches),
        Some(("urldecode", sub_matches)) => url_decode(sub_matches),
        Some(("timestamp", sub_matches)) => convert_timestamp(sub_matches),
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

fn convert_timestamp(matches: &ArgMatches) {
    if let Some(timestamp_str) = matches.get_one::<String>("timestamp") {
        let timestamp = timestamp_str.parse::<i64>().unwrap();
        let datetime = Local.timestamp_opt(timestamp, 0).unwrap();
        println!("{}", datetime.to_rfc3339());
    }
}