#![allow(dead_code)]

use std::io::Read;
use std::net::TcpStream;

use std::io;
use std::string::FromUtf8Error;

use quartz_nbt::NbtCompound;

// Structs for each of the types used in the packets sent by an MC server

#[derive(Debug, Clone)]
pub struct Boolean(pub bool);
#[derive(Debug, Clone)]
pub struct Byte(pub i8);
#[derive(Debug, Clone)]
pub struct UByte(pub u8);
#[derive(Debug, Clone)]
pub struct Short(pub i16);
#[derive(Debug, Clone)]
pub struct UShort(pub u16);
#[derive(Debug, Clone)]
pub struct Int(pub i32);
#[derive(Debug, Clone)]
pub struct Long(pub i64);
#[derive(Debug, Clone)]
pub struct Float(pub f32);
#[derive(Debug, Clone)]
pub struct Double(pub f64);
#[derive(Debug, Clone)]
pub struct MCString(pub String);
pub type Chat = MCString;
pub type Identifier = MCString;
#[derive(Debug, Clone)]
pub struct VarInt(pub i32);
#[derive(Debug, Clone)]
pub struct VarLong(pub i64);
#[derive(Debug, Clone)]
pub struct EntityMetadata(); // TODO
#[derive(Debug, Clone)]
pub struct Slot(Boolean, Option<VarInt>, Option<Byte>, Option<NBTTag>); // TODO
#[derive(Debug, Clone)]
pub struct NBTTag(pub NbtCompound); // TODO
#[derive(Debug, Clone)]
pub struct Position(pub i32, pub i32, pub i32); // TODO
pub type Angle = UByte;
#[derive(Debug, Clone)]
pub struct UUID(pub [u64; 2]);

// Each of these types implements to_bytes and from_bytes for easy conversion
// I should probably pull it out into a trait, that might make some things more manageable
// and expandable in the future, but for now I can't be bothered

impl Boolean {
    pub fn to_bytes(&self) -> Vec<u8> {
        match self.0 {
            true => vec![0x01],
            false => vec![0x00],
        }
    }

    pub fn from_bytes(val: &[u8]) -> Option<Boolean> {
        match val.get(0) {
            Some(0x00) => Some(Boolean(false)),
            Some(0x01) => Some(Boolean(true)),
            _ => None,
        }
    }
}

impl Byte {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.0 as u8]
    }

    pub fn from_bytes(val: &[u8; 1]) -> Byte {
        Byte(val[0] as i8)
    }
}

impl UByte {
    pub fn to_bytes(&self) -> Vec<u8> {
        vec![self.0]
    }

    pub fn from_bytes(val: &[u8; 1]) -> UByte {
        UByte(val[0])
    }
}

impl Short {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 2]) -> Short {
        Short(i16::from_be_bytes(*val))
    }
}

impl UShort {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 2]) -> UShort {
        UShort(u16::from_be_bytes(*val))
    }
}

impl Int {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 4]) -> Int {
        Int(i32::from_be_bytes(*val))
    }
}

impl Long {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 8]) -> Long {
        Long(i64::from_be_bytes(*val))
    }
}

impl Float {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 4]) -> Float {
        Float(f32::from_be_bytes(*val))
    }
}

impl Double {
    pub fn to_bytes(&self) -> Vec<u8> {
        self.0.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 8]) -> Double {
        Double(f64::from_be_bytes(*val))
    }
}

impl MCString {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::new();
        let bytes = self.0.as_bytes();
        out.append(&mut VarInt(bytes.len() as i32).to_bytes());
        out.extend(bytes);
        out
    }

    pub fn from_bytes(val: &[u8]) -> Result<MCString, FromUtf8Error> {
        let len = VarInt::from_bytes(val).expect("MCString was not prefixed with valid length");
        let str =
            String::from_utf8(val[len.num_bytes()..(len.0 as usize) + len.num_bytes()].to_vec())?;
        Ok(MCString(str))
    }
}

impl VarInt {
    pub fn from_bytes(buf: &[u8]) -> Option<VarInt> {
        const PART: u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;

        let mut index = 0usize;
        let mut byte = buf[index];

        loop {
            val |= (byte as u32 & PART) << (size * 7);
            size += 1;
            if size > 5 {
                return None;
            }
            if (byte & 0x80) == 0 {
                break;
            }
            index += 1;
            byte = buf[index];
        }
        Some(VarInt(val as i32))
    }

    pub fn from_vec(buf: &Vec<u8>, start: usize) -> (VarInt, usize) {
        const PART: u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;

        let mut index = 0usize;
        let mut byte = buf[index + start];

        loop {
            val |= (byte as u32 & PART) << (size * 7);
            size += 1;
            if size > 5 {
                return (VarInt(0), 0);
            }
            if (byte & 0x80) == 0 {
                break;
            }
            index += 1;
            match buf.get(index + start) {
                Some(i) => byte = *i,
                None => return (VarInt(0), 0),
            }
        }
        (VarInt(val as i32), index + 1)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        const PART: u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                buf.push(val as u8);
                return buf;
            }
            buf.push(val as u8 | !0x7F);
            val >>= 7;
        }
    }

    /// Reads a varint from a Read-able, consuming the bytes
    ///
    /// # Arguments
    ///
    /// * `stream` - The Read-able from which to read
    ///
    /// # Returns
    ///
    /// * `Ok(Some(VarInt))` if everything goes well
    /// * `Err(e)` if there is an error reading it
    /// * `Ok(None)` I don't think it actually returns this, I should probably clean this up later
    ///
    pub fn from_stream<S: Read>(stream: &mut S) -> Result<Option<(VarInt, usize)>, io::Error> {
        const PART: u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;

        let mut byte: [u8; 1] = [0];

        match stream.read_exact(&mut byte) {
            Ok(_) => {}
            Err(e) => return Err(e),
        }

        loop {
            val |= (byte[0] as u32 & PART) << (size * 7);
            size += 1;
            if size > 5 {
                panic!("VarInt too big!");
            }
            if (byte[0] & 0x80) == 0 {
                break;
            }
            match stream.read_exact(&mut byte) {
                Ok(_) => {}
                Err(e) => return Err(e),
            }
        }
        Ok(Some((VarInt(val as i32), size)))
    }

    pub fn num_bytes(&self) -> usize {
        let mut bytes: usize = 1;
        const PART: u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                return bytes;
            }
            val >>= 7;
            bytes += 1;
        }
    }
}

impl VarLong {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::new();

        const PART: u64 = 0x7F;
        let mut val = self.0 as u64;
        loop {
            if (val & !PART) == 0 {
                buf.push(val as u8);
                return buf;
            }
            buf.push(val as u8 | !0x7F);
            val >>= 7;
        }
    }

    pub fn from_bytes(buf: &[u8]) -> VarLong {
        const PART: u64 = 0x7F;
        let mut size = 0;
        let mut val = 0u64;

        let mut index = 0usize;
        let mut byte = buf[index];

        loop {
            val |= (byte as u64 & PART) << (size * 7);
            size += 1;
            if size > 10 {
                panic!("Invalid VarLong!");
            }
            if (byte & 0x80) == 0 {
                break;
            }
            index += 1;
            byte = buf[index];
        }
        VarLong(val as i64)
    }

    pub fn from_vec(buf: &Vec<u8>, start: usize) -> (VarLong, usize) {
        const PART: u64 = 0x7F;
        let mut size = 0;
        let mut val = 0u64;

        let mut index = 0usize;
        let mut byte = buf[index + start];

        loop {
            val |= (byte as u64 & PART) << (size * 7);
            size += 1;
            if size > 10 {
                return (VarLong(0), 0);
            }
            if (byte & 0x80) == 0 {
                break;
            }
            index += 1;
            match buf.get(index + start) {
                Some(i) => byte = *i,
                None => return (VarLong(0), 0),
            }
        }
        (VarLong(val as i64), index + 1)
    }
}

impl EntityMetadata {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(val: &[u8]) -> Option<Box<EntityMetadata>> {
        todo!()
    }
}

impl Slot {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(val: &[u8]) -> Option<Box<Slot>> {
        todo!()
    }
}

impl NBTTag {
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }

    pub fn from_bytes(val: &[u8]) -> Option<Box<NBTTag>> {
        todo!()
    }
}

impl Position {
    // I have a feeling this will not work as intended, but we can hope.
    pub fn to_bytes(&self) -> Vec<u8> {
        let big = (((self.0 as u64) & 0x3FFFFFF) << 38)
            | (((self.2 as u64) & 0x3FFFFFF) << 12)
            | ((self.1 as u64) & 0xFFF);

        big.to_be_bytes().to_vec()
    }

    pub fn from_bytes(val: &[u8; 8]) -> Position {
        let big = u64::from_be_bytes(*val);

        let x = (big >> 38) as i32;
        let y = (big & 0xfff) as i32;
        let z = (big << 26 >> 38) as i32;

        // This may be a source of error

        Position(x, y, z)
    }
}

impl UUID {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.0[0].to_be_bytes().to_vec();
        bytes.append(&mut self.0[1].to_be_bytes().to_vec());
        bytes
    }

    pub fn from_bytes(val: &[u8; 16]) -> UUID {
        let mut b1 = [0u8; 8];
        let mut b2 = [0u8; 8];
        for i in 0..16 {
            if i < 8 {
                b1[i] = val[i]
            }
            if i >= 8 {
                b2[i - 8] = val[i]
            }
        }
        UUID([u64::from_be_bytes(b1), u64::from_be_bytes(b2)])
    }
}

#[cfg(test)]
mod tests {
    use crate::network::types::{VarInt, VarLong};
    use rand::Rng;

    #[test]
    fn varint_works() {
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let val = VarInt(rng.gen());

            let bytes = val.to_bytes();
            let out = VarInt::from_bytes(bytes.as_slice()).unwrap();

            assert_eq!(val.0, out.0);
        }
    }

    #[test]
    fn varlong_works() {
        let mut rng = rand::thread_rng();

        for _ in 0..10000 {
            let val = VarLong(rng.gen());

            let bytes = val.to_bytes();
            let out = VarLong::from_bytes(bytes.as_slice());

            assert_eq!(val.0, out.0);
        }
    }
}
