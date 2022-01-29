// IPXCALC
// input is a string like "192.168.1.0/24"
// output is host range, net addr, bcast addr

// /mnt/home/steve/AndroidStudioProjects/IPvXCalc/app/src/main/java/com/alpharez/ipvxcalc/ipcalc
// use std::str::FromStr;
use std::env;
use ipaddress::IPAddress;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} IP/BITS", args[0]);
        eprintln!("Example: {} 192.168.1.0/24", args[0]);
        std::process::exit(1);
    }
    let str_prefix = &args[1];
    let ip = IPAddress::parse(str_prefix).unwrap();
    println!("{:?}", ip);
    if ip.is_ipv4() {
        println!("Decimal Host Address {}", ip.host_address);
        println!("Prefix {}", ip.prefix().to_s());
        println!("Netmask {}", ip.netmask().to_s());
        println!("Network {}", ip.network().to_s());
        println!("Broadcast {}", ip.broadcast().to_s());
        println!("Host Range: {} to {}", ip.first().to_s(), ip.last().to_s());
    }
}
