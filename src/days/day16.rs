// Advent of Code 2021
// Day 16: Packet Decoder

use crate::input_handler::parse_hexadecimal_transmission;

const FILE: &str = "inputs/day16_input.txt";

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: u16,
    length: usize,
}

impl Packet {
    fn new() -> Packet {
        Packet {version: 0, type_id: 0, value: 0, length: 0}
    }

    fn is_literal(&self) -> bool {
        self.value != 0
    }
}

pub fn day16_answer() {
    let hexadecimal_transmission = parse_hexadecimal_transmission(FILE);

    println!("Day 16, part 1: {}", 0);
    println!("Day 16, part 2: {}\n", 0);
}

fn hex_to_bin(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}

fn parse_packet(mut packet: Packet, binary_chars: &String, list_packets: &mut Vec<Packet>) {
    // Set first 6 bits to packet
    if binary_chars.len() < 6 {
        return;
    }
    packet.version = u8::from_str_radix(&binary_chars[0..3].to_string(), 2).unwrap();
    packet.type_id = u8::from_str_radix(&binary_chars[3..6].to_string(), 2).unwrap();
    packet.length = binary_chars.len();

    // The other bits (more info about this packet or subpackets)
    let subpacket_bits = binary_chars[6..].chars().collect::<Vec<_>>();

    if subpacket_bits.len() <= 0 {
        return;
    }

    // If Type ID is 4, it is literal value
    if packet.type_id == 4 {
        let mut literal_bin = String::new();

        for w in subpacket_bits.chunks(5) {
            for c in w[1..].iter() {
                literal_bin.push(*c);
            }

            if w[0] == '0' {
                packet.value = u16::from_str_radix(&literal_bin.to_string(), 2).unwrap();
                list_packets.push(packet);
                return;
            }
        }
    }
    // Operators
    else {
        list_packets.push(packet);
        
        let length_type_id = subpacket_bits[0];

        if length_type_id == '0' {
            // Something here is wrong
            if subpacket_bits.len() < 17 {
                return;
            }

            let total_length = usize::from_str_radix(&subpacket_bits[1..16].iter().collect::<String>(), 2).unwrap();
            let mut current_len = 0;

            while current_len <= total_length {
                let subpacket = Packet::new();

                parse_packet(subpacket, &subpacket_bits[16 + current_len..].iter().map(|c| *c).collect::<String>(), list_packets);
                current_len += list_packets[list_packets.len() - 1].length;

                if current_len > total_length || subpacket_bits.len() < 16 + current_len {
                    return;
                }
            }
        }
        else {
            if subpacket_bits.len() < 12 {
                return;
            }

            let number_subpackets = u16::from_str_radix(&subpacket_bits[1..12].iter().collect::<String>(), 2).unwrap();
            let mut current_len = 0;

            for _ in 0..number_subpackets {
                let subpacket = Packet::new();
                let subpacket_bits = binary_chars[12 + current_len..].chars().collect::<String>();

                parse_packet(subpacket, &subpacket_bits, list_packets);
                current_len += list_packets[list_packets.len() - 1].length;
            }
        }
    }
}

fn get_packet_info(hex: &String) {
    // Hex to binary
    let binary_chars = hex.chars().map(|c| hex_to_bin(c)).collect::<String>();

    // List to keep track of packets
    let mut list_packets: Vec<Packet> = Vec::new();

    // First packet, the One For All
    let one_for_all = Packet::new();

    // Passing One For All, input bits and the list to the parser
    parse_packet(one_for_all, &binary_chars, &mut list_packets);

    println!("{:?}", list_packets);
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day16_input_test.txt";

    #[test]
    fn day16_part1_test() {
        let hexadecimal_transmission = parse_hexadecimal_transmission(FILE);
        get_packet_info(&hexadecimal_transmission);
        get_packet_info(&String::from("38006F45291200"));
        //get_packet_info(&String::from("8A004A801A8002F478"));
        
        //assert_eq!( , 2021);
    }

    #[test]
    fn day16_part2_test() {
        
    }
}