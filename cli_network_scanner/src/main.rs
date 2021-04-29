extern crate pnet;

use pnet::datalink;
use std::env;
use std::io::{self, Write};
use std::net::IpAddr;
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

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

struct PingReturn {
    ip: IpAddr,
    status: bool,
}

impl PingReturn {
    fn new(ip: IpAddr, status: bool) -> Self {
        return PingReturn { ip, status };
    }
}

fn run_ping(tx: Sender<PingReturn>, ip: IpAddr) {
    let output = process::Command::new("ping")
        .arg(ip.to_string())
        .args(&["-n", "2", "-w", "100"])
        .output()
        .expect("Failed to run ping ip");
    let out_str = String::from_utf8_lossy(&output.stdout);
    if out_str.to_string().contains("TTL=") {
        tx.send(PingReturn::new(ip, true)).unwrap();
    } else {
        tx.send(PingReturn::new(ip, false)).unwrap();
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

    // Iterate through the not null prefix interfaces
    for iface in datalink::interfaces() {
        if iface.ips[0].prefix() != 0 {
            println!("INTERFACE : {}", iface.ips[0].ip());
            println!("  DEVICES");
            let (tx, rx) = channel();
            //? Create a thread for each ping
            for ip in iface.ips[0].iter() {
                let tx = tx.clone();
                //? Receiving reply for our arp query might pose prbrm, maybe pinging is all we need ?
                thread::spawn(move || {
                    run_ping(tx, ip);
                });
            }
            let mut ids = Vec::with_capacity(256 as usize);
            for _ in 0..256 {
                ids.push(rx.recv());
            }
            for p in ids {
                match p {
                    Ok(s) => {
                        if s.status {
                            println!("  {}", s.ip);
                            io::stdout().flush().unwrap();
                        }
                    }
                    Err(_) => {}
                }
            }
        };
    }
}
