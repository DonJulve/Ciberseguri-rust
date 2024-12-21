use std::env;
use std::net::{TcpStream, ToSocketAddrs};
use std::time::Duration;
use std::process;

fn scan_port(address: &str, port: u16) -> bool {
    let socket_addr = format!("{}:{}", address, port);
    let timeout = Duration::from_secs(1);
    TcpStream::connect_timeout(&socket_addr.to_socket_addrs().unwrap().next().unwrap(), timeout).is_ok()
}

fn usage() {
    println!("Usage: port_scanner <IP_ADDRESS> [START_PORT END_PORT]");
    println!("  <IP_ADDRESS>: The IP address to scan.");
    println!("  [START_PORT END_PORT]: Optional. The range of ports to scan (inclusive). Default is 1-1024.");
    process::exit(1);
}

fn main() {
    // Obtención de los argumentos desde la línea de comandos
    let args: Vec<String> = env::args().collect();

    // Verificación de los argumentos
    if args.len() == 1 || args.len() == 2 || args.len() == 4 {
        if args.len() == 1 {
            usage();
        }

        if args[1] == "--help" {
            usage();
        }

        let target = &args[1];

        let start_port = if args.len() > 2 {
            args[2].parse::<u16>().unwrap_or(1)
        } else {
            1
        };

        let end_port = if args.len() > 3 {
            args[3].parse::<u16>().unwrap_or(1024)
        } else {
            1024
        };

        if start_port > end_port {
            eprintln!("Error: start port cannot be greater than end port.");
            process::exit(1);
        }

        let ports: Vec<u16> = (start_port..=end_port).collect();

        println!("Scanning ports on {} from {} to {}...", target, start_port, end_port);

        for port in ports {
            if scan_port(target, port) {
                println!("Port {} is open", port);
            }
        }
    } else {
        usage();
    }
}

