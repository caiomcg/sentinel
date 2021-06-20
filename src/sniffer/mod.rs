pub mod handlers;
pub mod sniffer_handler;
pub mod ipv4_handler;

use pnet::datalink::{self, NetworkInterface};
use pnet::packet::ethernet::{EtherType, EthernetPacket };
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;
use pnet::datalink::Channel::Ethernet;

use sniffer_handler::SnifferHandler;

pub struct Sniffer {
    handlers: SnifferHandler,
}

impl Sniffer {
    pub fn new(handlers: SnifferHandler) -> Sniffer {
        Sniffer {
            handlers
        }
    }

    pub fn sniff(&self, interface: &NetworkInterface) {
        println!("One day I will sniff: {}", interface.name);

        let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
            Ok(Ethernet(tx, rx)) => (tx, rx),
            Ok(_) => panic!("sniffer: unhandled channel type {}"),
            Err(e) => panic!("sniffer: unable to create channel: {}", e)
        };

        loop {
            match rx.next() {
                Ok(packet) => {
                    let packet = EthernetPacket::new(packet).unwrap();
                    let ethertype = packet.get_ethertype();
                    if let Some(ref mut h) = self.handlers.handle_packet(ethertype) {
                        h.handle(&interface.name, &packet);
                    } else {
                        println!("No handler");
                    }
                },
                Err(e) => panic!("sniffer: unable to get ethernet packet: {}", e)
            }

        }
    }
}

