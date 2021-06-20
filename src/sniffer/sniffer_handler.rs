use super::handlers::{ PacketHandler, ProtocolHandler };

use pnet::packet::ethernet::EtherType;
use pnet::packet::ip::IpNextHeaderProtocol;

use std::collections::HashMap;

pub struct SnifferHandler {
    packet_handlers: HashMap<EtherType, Box<dyn PacketHandler>>,
    protocol_handlers: HashMap<IpNextHeaderProtocol, Box<dyn ProtocolHandler>>,
}

pub enum Handler {
    Packet(EtherType, Box<dyn PacketHandler>),
    Protocol(IpNextHeaderProtocol, Box<dyn ProtocolHandler>),
}

impl SnifferHandler {
    pub fn new() -> SnifferHandler {
        SnifferHandler {
            packet_handlers: HashMap::new(),
            protocol_handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, handler: Handler) {
        match handler {
            Handler::Packet(ether_types, packet_handler) => {
                SnifferHandler::add_packet(
                    &mut self.packet_handlers, 
                    ether_types,
                    packet_handler
                    );
            },
            Handler::Protocol(protocol, protocol_handler) => {
                SnifferHandler::add_packet(
                    &mut self.protocol_handlers,
                    protocol,
                    protocol_handler
                    );
            }
        }
    }

    pub fn handle_packet(&self, packet_type: EtherType) -> Option<&Box<dyn PacketHandler>> {
        self.packet_handlers.get(&packet_type)
    }

    pub fn handle_protocol(&self, packet_type: EtherType) -> Option<&Box<dyn PacketHandler>> {
        self.packet_handlers.get(&packet_type)
    }

    fn add_packet<K, H>(map: &mut HashMap<K,H>, k: K, handler: H) 
        where K: std::cmp::Eq + std::hash::Hash
        {
            if let Some(map_handler) = map.get_mut(&k) {
                *map_handler = handler;
            } else {
                map.insert(k, handler);
            }
        }
}

