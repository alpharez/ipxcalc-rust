# IPXCALC

IP Calculator in Rust.  Command line input of IP/bits and it will output info on the IP Subnet and hosts.

IPAddress doc located at

https://docs.rs/ipaddress/0.1.1/ipaddress/struct.IPAddress.html

## Usage Example
<pre>
$ ipxcalc 10.1.0.64/27
IPAddress: 10.1.0.64/27
Decimal Host Address 167837760
Prefix 27
Netmask 255.255.255.224
Network 10.1.0.64
Broadcast 10.1.0.95
Host Range: 10.1.0.65 to 10.1.0.94
</pre>

