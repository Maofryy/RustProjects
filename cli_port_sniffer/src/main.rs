use std::env;
use std::io::{self, Write};
use std::net::{IpAddr, TcpStream};
use std::process;
use std::str::FromStr;
use std::sync::mpsc::{channel, Sender};
use std::thread;

mod csv_utils;

const MAX_THREADS: u16 = 65535;

struct Arguments {
    flag: String,
    ipaddr: IpAddr,
    threads: u16,
}

impl Arguments {
    pub fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Invalid number of arguments.");
        } else if args.len() > 4 {
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

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                print!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX_THREADS - port) <= num_threads {
            break;
        }
        port += num_threads;
    }
}

fn print_usage() {
    println!("Usage: 
    \r    ./cli_port_sniffer -h                           : Print this help message
    \r    ./cli_port_sniffer <ip_addr>                    : Sniff all ports of target <ip_addr>
    \r    ./cli_port_sniffer -j <thread_number> <ip_addr> : Sniff all ports of <ip_addr> using <thread_number> threads. 
    ");
}

fn main() {
    println!("Welcome to your CLI Port sniffer.");
    // Handle args

    let tcp_dict = csv_utils::read_csv("./data/tcp.csv".to_string()).unwrap();
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(1);
        }
    });
    // divide into threadd, scan ip
    let num_threads = arguments.threads;
    let addr = arguments.ipaddr;
    // Other flag options ideas ?
    let (tx, rx) = channel();
    for i in 0..num_threads {
        let tx = tx.clone();

        thread::spawn(move || {
            scan(tx, i, addr, num_threads);
        });
    }

    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }

    println!("\nOpened ports on {}:\nPORT     STATE SERVICE", addr);
    out.sort();
    for v in out {
        let mut nb = v.to_string();
        nb.push_str("/tcp");
        println!(
            "{:<5} open  {:<}",
            nb,
            tcp_dict.get(&v.to_string()).unwrap()
        );
    }
    //TODO Implement tcp list server to add description
    //TODO add other flags behaviors ?
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_args() {
        assert!(Arguments::new(&["./test".to_string(), "-h".to_string()]).is_err());
        assert!(Arguments::new(&[
            "./test".to_string(),
            "-j".to_string(),
            "15".to_string(),
            "192.168.1.52".to_string()
        ])
        .is_ok());
        assert!(Arguments::new(&["./test".to_string(), "192.168.1.52".to_string()]).is_ok());
        assert!(Arguments::new(&[
            "-j".to_string(),
            "nb".to_string(),
            "192.168.1.52".to_string()
        ])
        .is_err());
        assert!(Arguments::new(&["./test".to_string(), "1192.168.1.52".to_string()]).is_err());
    }

    #[test]
    fn test_scan() {}
}
