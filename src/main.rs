// IPXCALC
// input is a string like "192.168.1.0/24"
// output is host range, net addr, bcast addr

use ipaddress::IPAddress;
use text_colorizer::*;
use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // IP/Bits
    prefix: String
}

fn main() {
    let args = Args::parse();
    let ip = IPAddress::parse(args.prefix).unwrap();
    if ip.is_ipv4() || ip.is_ipv6() {
	// eprintln!("{}", "ipxcalc".blue());
        // eprintln!("Decimal Host Address {}", ip.host_address);
        // eprintln!("{} {}", "Prefix".blue(), ip.prefix().to_s().yellow());
        eprintln!("{} {} {} {} {} {}", "Network".blue(), ip.network().to_s().yellow(), "Broadcast".blue(), ip.broadcast().to_s().yellow(), "Mask".blue(), ip.netmask().to_s().yellow());
        eprintln!("{} {} - {}", "Hosts".blue(), ip.first().to_s().green(), ip.last().to_s().green());
    } else {
        eprintln!("Prefix does not seem to be an IPv4 or IPv6 address");
    }

}
