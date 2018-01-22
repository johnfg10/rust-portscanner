extern crate clap;
extern crate chrono;

use clap::{Arg, App, SubCommand};
use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::fs::File;
use std::path::Path;
use std::fs::remove_file;
use std::fs::OpenOptions;
use std::time::Instant;
use chrono::prelude::*;

struct LogInfo{
    file: File,
    console_logging_enabled: bool
}

fn ping_tcp(lowest_port: u16, highest_port: u16, ipaddress: &str, mut log_info: LogInfo){
    for port in lowest_port .. highest_port {
        let tcp_stream = TcpStream::connect(SocketAddr::new(IpAddr::from_str(ipaddress).unwrap(), port));
        match tcp_stream {
            Ok(stream) => {
                let message = format!("Tcp reciver found on port: {}", port);
                writeln!(log_info.file, "{}", message);

                if log_info.console_logging_enabled {
                    print!("{}", message);
                }
            }
            Err(er) => ()
        }
    }
}

fn main() {

    let matches = App::new("pinger")
        .version("1.0.0")
        .author("John <johnfg2610@gmail.com>")
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("FILE")
                .help("Sets a custom output file if empty will default to output.txt")
        )
        .arg(
            Arg::with_name("transfercontrolprotocol")
                .short("t")
                .long("transfercontrolprotocol")
                .help("Sets whether or not to scan for TCP ports")
        )
        .arg(
            Arg::with_name("verbose")
                .short("v")
                .long("verbose")
                .help("Sets whether or not to print to console")
        )
        .arg(
            Arg::with_name("minport")
                .short("m")
                .long("minport")
                .value_name("INT")
                .help("The minimum port to scan by default is 0")
        )
        .arg(
            Arg::with_name("maxport")
                .short("M")
                .long("maxport")
                .value_name("INT")
                .help("The maximum port to scan by default is 65535")
        )
        .arg(
            Arg::with_name("ipaddress")
                .short("ip")
                .long("ipaddress")
                .value_name("TEXT")
                .help("The ip address this should be targeted at by default localhost")
           )
        .get_matches();

    let minimum_port = matches.value_of("minport").unwrap_or("0").parse().unwrap_or(0);
    let maximum_port = matches.value_of("maxport").unwrap_or("65535").parse().unwrap_or(65535);
    let ipaddress = matches.value_of("ipaddress").unwrap_or("0.0.0.0");
    let file_loc = matches.value_of("output").unwrap_or("output.txt");
    let path = Path::new(file_loc);
    let is_verbose: bool = match matches.occurrences_of("verbose") {
        1 => true,
        _ => false
    };
    let is_tcp: bool = match matches.occurrences_of("transfercontrolprotocol") {
        1 => true,
        _ => false
    };

    if !path.exists() {
        File::create(path).unwrap();
    }

    let mut file = OpenOptions::new().write(true).append(true).open(path).unwrap();

    writeln!(file, "Initilizing port scans: {}", Utc::now().to_string());

    let log_info = LogInfo{file, console_logging_enabled: is_verbose };

    if is_tcp {
        println!("pinging ips");
        ping_tcp(minimum_port, maximum_port, ipaddress, log_info);
    }
}