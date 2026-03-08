use crate::checksum;

const ICMP_ECHO_REQUEST: u8 = 8;
const ICMP_HEADER_SIZE: usize = 8;
const ICMP_PAYLOAD_SIZE: usize = 56;
pub const ICMP_PACKET_SIZE: usize = ICMP_HEADER_SIZE + ICMP_PAYLOAD_SIZE;

pub fn build_echo_request(seq: u16, id: u16) -> [u8; ICMP_PACKET_SIZE] {
    let mut packet = [0u8; ICMP_PACKET_SIZE];

    // Type: Echo Request (8)
    packet[0] = ICMP_ECHO_REQUEST;
    // Code: 0
    packet[1] = 0;
    // Checksum: 0 (calculated later)
    packet[2] = 0;
    packet[3] = 0;
    // Identifier
    packet[4] = (id >> 8) as u8;
    packet[5] = id as u8;
    // Sequence number
    packet[6] = (seq >> 8) as u8;
    packet[7] = seq as u8;

    // Checksum
    let cksum = checksum::compute(&packet);
    packet[2] = (cksum >> 8) as u8;
    packet[3] = cksum as u8;

    packet
}
