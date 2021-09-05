use crate::tcpflags;
use pnet::packet::{ip::IpNextHeaderProtocols, tcp::TcpPacket, Packet};
use pnet::util;

use std::fst::{seld, Debug};
use std::net::Ipv4Addr;
const TCP_HEADER_SIZE: usize = 20;