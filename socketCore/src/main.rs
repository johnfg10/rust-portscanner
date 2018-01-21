use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream, SocketAddr};
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use std::fs::File;
use std::path::Path;
use std::fs::remove_file;

fn ping_ips(lowest_port: u16, highest_port: u16, mut log_file: File){
    for x in lowest_port .. highest_port {

        let tcp_stream = TcpStream::connect(SocketAddr::new(IpAddr::from([0, 0, 0, 0]), x));

        match tcp_stream {
            Ok(stream) => {
                let mut formmated_string = format!("Tcp server found on port: {} \n", x);
                log_file.write_all(formmated_string.as_bytes());
            },
            Err(er) => eprintln!("{}: {}", x,  er)
        }
    }
}

fn main() {
    let path = Path::new("./test.txt");
    println!("test");
    if path.exists() {
        let remove_result = remove_file(path);

        match remove_result {
            Ok(k) => (),
            Err(er) =>  eprintln!("{}", er)
        }
    }

    let file = File::create(path);
    match file {
        Ok(file) => ping_ips(0, 65535, file),
        Err(er) => eprintln!("{}", er)
    }

    //ping_ips(0, 65535);
}