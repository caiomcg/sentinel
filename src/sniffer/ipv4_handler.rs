use dns_lookup::{ lookup_addr };
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::Packet;

use crate::sniffer::handlers::{ PacketHandler };

use std::net::IpAddr;

pub struct Ipv4Handler;

impl Ipv4Handler {
    pub fn new() -> Ipv4Handler {
        Ipv4Handler{}
    }
}

impl PacketHandler for Ipv4Handler {
    fn handle(&self, interface_name: &str, packet: &EthernetPacket) {
        if let Some(header) = Ipv4Packet::new(packet.payload()) {
            println!(
                "[IPV4]: {} {} {}",
                interface_name,
                lookup_addr(&IpAddr::V4(header.get_source())).unwrap(),
                header.get_next_level_protocol(),
                );
        } else {
            println!("Can't handle packet");
        }

    }
}
