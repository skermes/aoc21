use std::ops::{ShlAssign, AddAssign};
use crate::aoc_error::AocError;

pub const NAME: &str = "Packet Decoder";

#[derive(Debug)]
struct Bits {
    chars: Vec<char>,
    char_index: usize,
    current_bits: Option<u32>,
    current_bit_shift: usize
}

impl Bits {
    fn new(s: &str) -> Self {
        Bits {
            chars: s.chars().collect(),
            char_index: 0,
            current_bits: None,
            current_bit_shift: 0
        }
    }
}

impl Iterator for Bits {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_bits.is_none() {
            if self.char_index >= self.chars.len() {
                return None
            } else {
                self.current_bits = Some(self.chars[self.char_index].to_digit(16).unwrap());
                self.char_index += 1;
                self.current_bit_shift = 3;
            }
        }

        let mask = 1 << self.current_bit_shift;
        let n = self.current_bits.unwrap();
        let bit = if n & mask > 0 { 1 } else { 0 };

        if self.current_bit_shift == 0 {
            self.current_bits = None;
        } else {
            self.current_bit_shift -= 1;
        }

        Some(bit)
    }
}

fn read_to_int<T>(bits: &mut Bits, bit_count: usize) -> T
    where T: Default + ShlAssign + AddAssign + From<u8>
{
    let mut n = T::default();
    for b in bits.take(bit_count) {
        n <<= 1.into();
        n += b.into();
    }

    n
}

#[derive(Debug)]
struct PacketList(Vec<Packet>);

impl PacketList {
    fn version_sum(&self) -> usize {
        self.0.iter().map(|p| p.version_sum()).sum()
    }

    fn sum(&self) -> usize {
        self.0.iter().map(|p| p.value()).sum()
    }

    fn product(&self) -> usize {
        self.0.iter().map(|p| p.value()).product()
    }

    fn min(&self) -> usize {
        self.0.iter().map(|p| p.value()).min().unwrap()
    }

    fn max(&self) -> usize {
        self.0.iter().map(|p| p.value()).max().unwrap()
    }

    fn greater_than(&self) -> usize {
        if self.0[0].value() > self.0[1].value() { 1 } else { 0 }
    }

    fn less_than(&self) -> usize {
        if self.0[0].value() < self.0[1].value() { 1 } else { 0 }
    }

    fn equal_to(&self) -> usize {
        if self.0[0].value() == self.0[1].value() { 1 } else { 0 }
    }
}

#[derive(Debug)]
enum Packet {
    Literal {
        version: u8,
        value: usize
    },
    Operator {
        version: u8,
        type_id: u8,
        packets: PacketList
    }
}

impl Packet {
    fn read_literal(bits: &mut Bits, version: u8) -> (Self, usize) {
        let mut bits_read = 0;
        let mut value = 0;
        loop {
            let last = read_to_int::<u8>(bits, 1) == 0;
            value <<= 4;
            value += read_to_int::<usize>(bits, 4);
            bits_read += 5;

            if last { break; }
        }

        (Packet::Literal { version, value }, bits_read)
    }

    fn read_op_with_bit_len(bits: &mut Bits, version: u8, type_id: u8) -> (Self, usize) {
        let packet_length: usize = read_to_int(bits, 15);
        let mut bits_read = 15;

        let mut packets = Vec::new();
        let mut packet_bits_read = 0;
        loop {
            let (packet, read) = Packet::read_from_bits(bits);
            packets.push(packet);
            packet_bits_read += read;

            if packet_bits_read >= packet_length {
                break;
            }
        }

        bits_read += packet_bits_read;

        let packets = PacketList(packets);
        (Packet::Operator { version, type_id, packets }, bits_read)
    }

    fn read_op_with_packet_count(bits: &mut Bits, version: u8, type_id: u8) -> (Self, usize) {
        let packet_count: usize = read_to_int(bits, 11);
        let mut bits_read = 11;

        let mut packets = Vec::new();
        for _ in 0..packet_count {
            let (packet, read) = Packet::read_from_bits(bits);
            packets.push(packet);
            bits_read += read;
        }

        let packets = PacketList(packets);
        (Packet::Operator { version, type_id, packets }, bits_read)
    }

    fn read_from_bits(bits: &mut Bits) -> (Self, usize) {
        let mut bits_read = 0;

        let version = read_to_int(bits, 3);
        let type_id = read_to_int(bits, 3);
        bits_read += 6;

        if type_id == 4 {
            let (packet, bits) = Packet::read_literal(bits, version);
            (packet, bits_read + bits)
        } else {
            let length_type: u8 = read_to_int(bits, 1);
            bits_read += 1;

            if length_type == 0 {
                let (packet, bits) = Packet::read_op_with_bit_len(bits, version, type_id);
                (packet, bits_read + bits)
            } else {
                let (packet, bits) = Packet::read_op_with_packet_count(bits, version, type_id);
                (packet, bits_read + bits)
            }
        }
    }

    fn version_sum(&self) -> usize {
        match self {
            Packet::Literal { version, .. } => *version as usize,
            Packet::Operator { version, packets, .. } => {
                *version as usize + packets.version_sum()
            }
        }
    }

    fn value(&self) -> usize {
        match self {
            Packet::Literal { value, .. } => *value,
            Packet::Operator { type_id, packets, .. } => {
                match type_id {
                    0 => packets.sum(),
                    1 => packets.product(),
                    2 => packets.min(),
                    3 => packets.max(),
                    5 => packets.greater_than(),
                    6 => packets.less_than(),
                    7 => packets.equal_to(),
                    _ => 0
                }
            }
        }
    }
}

pub fn part_one(input: &str) -> Result<String, AocError> {
    let mut bits = Bits::new(input);
    let (packet, _) = Packet::read_from_bits(&mut bits);

    Ok(packet.version_sum().to_string())
}

pub fn part_two(input: &str) -> Result<String, AocError> {
    let mut bits = Bits::new(input);
    let (packet, _) = Packet::read_from_bits(&mut bits);

    Ok(packet.value().to_string())
}