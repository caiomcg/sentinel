mod config;
mod sniffer;

use pnet::datalink;
use structopt::StructOpt;
use config::Opts;
use sniffer::Sniffer;
use pnet::packet::ethernet::EtherTypes;
use sniffer::sniffer_handler::{ SnifferHandler, Handler };
use sniffer::ipv4_handler::Ipv4Handler;

fn main() {
    let config = Opts::from_args();
    let interfaces = datalink::interfaces();
    let interface = interfaces.iter()
        .filter(|iface| iface.name == config.interface)
        .next();

    let mut handlers = SnifferHandler::new();
    handlers.add_handler(Handler::Packet(
            EtherTypes::Ipv4,
            Box::new(Ipv4Handler::new()),
            ));

    if let Some(interface) = interface {
        let sniffer = Sniffer::new(handlers);
        sniffer.sniff(interface);
    } else {
        let interfaces: Vec<&str> = interfaces.iter()
            .map(|iface| &iface.name[..])
            .collect();

        println!("Could not find the requested interface.");
        println!(
            "These are the available adapters on your device: {:?}", 
            interfaces
            );
    }
}
