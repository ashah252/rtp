
use std::env;
use std::io;
use std::io::prelude::*;
use std::fs::File;

mod rtp;

fn usage(){
    println!("Usage: $PATH/rtp-send <ip> <port> <filepath>");
    std::process::exit(0);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        4 => {},
        _ => usage()
    }

    let ip = args[1].clone();
    let port: u16 = args[2].parse::<u16>().unwrap();
    let filepath = args[3].clone();
    

    println!("sending {} to {}:{}", filepath, ip, port);
    let mut tcp_sender: rtp::TcpSession;

    let fd: File;
    match File::open(&filepath) {
        Ok(x) => {
            fd = x;
        },
        Err(_) => {
            println!("Error Opening file {}", filepath);
            std::process::exit(0);
        }
    }
    
}

