extern crate pnet;

use pnet::packet::arp::ArpOperation;
use pnet::util::MacAddr;
use std::env;
use std::net::{IpAddr, SocketAddr};
use std::process;
use std::str::FromStr;

use pnet::datalink;

#[derive(Debug)]
struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() == 1 {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr: IpAddr::from_str("192.168.1.0").unwrap(),
                threads: 4,
            });
        }
        if args.len() > 4 {
            return Err("Too many arguments.");
        }
        let f = args[1].clone();
        if let Ok(ipaddr) = IpAddr::from_str(&f) {
            return Ok(Arguments {
                flag: String::from(""),
                ipaddr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                print_usage();
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments.");
            } else if flag.contains("-j") {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Incorrect ip address format."),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Incorrect number format."),
                };
                return Ok(Arguments {
                    flag,
                    ipaddr,
                    threads,
                });
            } else {
                return Err("Invalid syntax.");
            }
        }
    }
}

fn print_usage() {
    println!("Usage:
    \r\t./cli_network_scanner -h                            : Print this help message
    \r\t./cli_network_scanner                               : Default behavior scanning 192.168.1.0/24
    \r\t./cli_network_scanner <ip_range>                    : Scanning <ip_range>
    \r\t./cli_network_scanner -j <nb_threads> <ip_range>    : Scanning <ip_range> with <nb_threads> threads
    ");
}

fn main() {
    println!("Hello and welcome to your network scanner!");

    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();

    //TODO Rework usage and arguments needs
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            print_usage();
            process::exit(0);
        }
    });
    println!("{:?}", arguments);

    //TODO for each non-null prefix, launch a network scan
    //TODO      The range to scan depends on the prefix ?
    for iface in datalink::interfaces() {
        if iface.ips[0].prefix() != 0 {
            //? Create a thread for each ip
            for ip in iface.ips[0].iter() {
                //? Send an arp request and wait for the response
                println!("{:?}", ip);
            }
        };
    }
}
