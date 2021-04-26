use std::env;
use std::net::IpAddr;
use std::str::FromStr;
use std::process;

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
            return Ok(Arguments{flag: String::from(""), ipaddr, threads: 4});
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                print_usage();
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments.");
            } else if flag.contains("-j") && args.len() == 4 {
                let ipaddr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Incorrect ip address format."),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Incorrect number format."),
                };
                return Ok(Arguments{flag, ipaddr, threads});
            } else {
                return Err("Invalid syntax.");
            }
        }
        
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
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(
        |err| {
            if err.contains("help") {
                process::exit(0);
            } else {
                eprintln!("{} problem parsing arguments: {}", program, err);
                process::exit(1);
            }
        }
    );
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn test_args() {
        assert!(Arguments::new(&["./test".to_string(), "-h".to_string()]).is_err());
        assert!(Arguments::new(&["./test".to_string(),"-j".to_string(), "15".to_string(), "192.168.1.52".to_string()]).is_ok());
        assert!(Arguments::new(&["./test".to_string(),"192.168.1.52".to_string()]).is_ok());
        assert!(Arguments::new(&["-j".to_string(), "nb".to_string(), "192.168.1.52".to_string()]).is_err());
        assert!(Arguments::new(&["./test".to_string(), "1192.168.1.52".to_string()]).is_err());
    }

    #[test]
    fn test_scan() {

    }
}