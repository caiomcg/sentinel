use pnet::packet::ethernet::EthernetPacket;
use std::net::IpAddr;

pub trait PacketHandler {
    fn handle(&self, interface_name: &str, packet: &EthernetPacket);
}

pub trait ProtocolHandler {
    fn handle(&self, interface_name: &str, source: IpAddr, destination: IpAddr, packet: &[u8]);
}
