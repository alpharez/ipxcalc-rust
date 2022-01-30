// IPXCALC
// input is a string like "192.168.1.0/24"
// output is host range, net addr, bcast addr

use std::env;
use ipaddress::IPAddress;
use text_colorizer::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} IP/BITS", args[0]);
        eprintln!("Example: {} 192.168.1.0/24", args[0]);
        std::process::exit(1);
    }
    let str_prefix = &args[1];
    let ip = IPAddress::parse(str_prefix).unwrap();
    // println!("{:?}", ip);
    if ip.is_ipv4() {
	// eprintln!("{}", "ipxcalc".blue());
        // eprintln!("Decimal Host Address {}", ip.host_address);
        // eprintln!("{} {}", "Prefix".blue(), ip.prefix().to_s().yellow());
        eprintln!("{} {} {} {} {} {}", "Network".blue(), ip.network().to_s().yellow(), "Broadcast".blue(), ip.broadcast().to_s().yellow(), "Mask".blue(), ip.netmask().to_s().yellow());
        eprintln!("{} {} - {}", "Hosts".blue(), ip.first().to_s().green(), ip.last().to_s().green());
    }
}
