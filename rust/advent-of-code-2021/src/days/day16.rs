// Advent of Code 2021
// Day 16: Packet Decoder

use crate::input_handler::parse_hexadecimal_transmission;

const FILE: &str = "inputs/day16_input.txt";

#[derive(Clone, Debug)]
struct Packet {
    version: u8,
    type_id: u8,
    value: usize,
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

    println!("Day 16, part 1: {}", sum_versions(&get_packet_info(&hexadecimal_transmission)));
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
    //packet.length = binary_chars.len(); // MOVE

    // The other bits (more info about this packet or subpackets)
    let subpacket_bits = binary_chars[6..].chars().collect::<Vec<_>>();

    // If no packet info, we are done
    if subpacket_bits.len() < 1 {
        return;
    }

    // Literal value
    if packet.type_id == 4 {
        let mut literal_bin = String::new();
        let mut packet_len = 6;

        for chunk in subpacket_bits.chunks(5) {
            for c in chunk[1..].iter() {
                literal_bin.push(*c);
            }

            packet_len += chunk.len();
            
            if chunk[0] == '0' {
                packet.length = packet_len;
                packet.value = usize::from_str_radix(&literal_bin.to_string(), 2).unwrap();
                list_packets.push(packet);
                return;
            }
        }
    }
    // Operators
    else {
        if subpacket_bits[0] == '0' {
            if subpacket_bits.len() < 16 {
                return;
            }

            let mut total_len = usize::from_str_radix(&subpacket_bits[1..16].iter().collect::<String>(), 2).unwrap() + 16;

            if total_len > subpacket_bits.len() {
                total_len = subpacket_bits.len();
            }

            let mut current_len = 16;

            while current_len <= total_len {
                let subpacket = Packet::new();
                
                parse_packet(subpacket, &subpacket_bits[current_len..total_len].iter().map(|c| *c).collect::<String>(), list_packets);
                current_len += list_packets[list_packets.len() - 1].length;
            }

            packet.length = total_len;
        }
        else {
            if subpacket_bits.len() < 12 {
                return;
            }

            let number_subpackets = u16::from_str_radix(&subpacket_bits[1..12].iter().collect::<String>(), 2).unwrap();
            let mut current_len = 12;

            for _ in 0..number_subpackets {
                let subpacket = Packet::new();
                
                parse_packet(subpacket, &subpacket_bits[current_len..].iter().collect::<String>(), list_packets);
                current_len += list_packets[list_packets.len() - 1].length;

                if current_len > subpacket_bits.len() {
                    current_len -= subpacket_bits.len();
                    break;
                }
            }

            packet.length = current_len;
        }

        list_packets.push(packet);
    }
}

fn get_packet_info(hex: &String) -> Vec<Packet> {
    // Hex to binary
    let binary_chars = hex.chars().map(|c| hex_to_bin(c)).collect::<String>();
    println!("{:?}", binary_chars);
    // List to keep track of packets
    let mut list_packets: Vec<Packet> = Vec::new();

    // First packet, the One For All
    let one_for_all = Packet::new();

    // Passing One For All, input bits and the list to the parser
    parse_packet(one_for_all, &binary_chars, &mut list_packets);

    println!("Number packets: {:?}", list_packets.len());
    println!("{:?}", list_packets);
    list_packets
}

fn sum_versions(list_packets: &Vec<Packet>) -> usize {
    list_packets.iter().map(|p| p.version as usize).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const FILE: &str = "inputs/day16_input_test.txt";

    #[test]
    fn day16_part1_test() {
        let hexadecimal_transmission = parse_hexadecimal_transmission(FILE);
        println!("{:?}", sum_versions(&get_packet_info(&String::from("620080001611562C8802118E34"))));

        //assert_eq!(sum_versions(&get_packet_info(&String::from("D2FE28"))), 6);
        //assert_eq!(sum_versions(&get_packet_info(&String::from("38006F45291200"))), 9);
        //assert_eq!(sum_versions(&get_packet_info(&String::from("EE00D40C823060"))), 14);
        //assert_eq!(sum_versions(&get_packet_info(&hexadecimal_transmission)), 16);
        //assert_eq!(sum_versions(&get_packet_info(&String::from("C0015000016115A2E0802F182340"))), 23);
        //assert_eq!(sum_versions(&get_packet_info(&String::from("A0016C880162017C3686B18A3D4780"))), 31);
    }

    #[test]
    fn day16_part2_test() {
        
    }
}