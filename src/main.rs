use clap::{Arg, App, SubCommand};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use vpn::start_tun;

mod vpn;

extern crate tun;

#[tokio::main]
async fn main() {
  // println!("Hello, world!");

  let matches = App::new("TorBox Tunnel")
    .version(env!("CARGO_PKG_VERSION"))
    .author("Luca")
    .about(env!("CARGO_PKG_DESCRIPTION"))
    .subcommand(
			SubCommand::with_name("start")
				.about("")
				.author("Luca")
        .arg(Arg::with_name("ipaddr")
          .short("i")
          .long("ipaddr")
          .value_name("IPADDR")
          .default_value("192.168.42.1")
          .help("Set your Tor address")
          .takes_value(true),
        )
        .arg(Arg::with_name("port")
          .short("p")
          .long("port")
          .value_name("PORT")
          .default_value("9050")
          .help("Set your Tor port")
          .takes_value(true)))
    .get_matches();
  
  let socket = SocketAddr::new(IpAddr::V4(
    matches.value_of("ipaddr").unwrap().parse::<Ipv4Addr>().unwrap()),
    matches.value_of("port").unwrap().parse::<u16>().unwrap());

  if let Some(matches) = matches.subcommand_matches("start") {
    start_tun(socket);
  };
}