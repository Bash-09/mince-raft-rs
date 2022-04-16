#![allow(dead_code)]

use std::error::Error;
use std::fmt::Display;
use std::io::{Read, Write, Cursor};

use quartz_nbt;

use quartz_nbt::NbtCompound;

// Structs for each of the types used in the packets sent by an MC server

pub type Chat = String;
pub type Identifier = String;
#[derive(Debug, Clone)]
pub struct VarInt(pub i32);
#[derive(Debug, Clone)]
pub struct VarLong(pub i64);
#[derive(Debug, Clone)]
pub struct Metadata(); // TODO
#[derive(Debug, Clone)]
pub struct Slot(bool, Option<VarInt>, Option<i8>, Option<NBTTag>); // TODO
#[derive(Debug, Clone)]
pub struct NBTTag(pub NbtCompound); // TODO
#[derive(Debug, Clone)]
pub struct Position(pub i32, pub i32, pub i32); // TODO
pub type Angle = u8;
#[derive(Debug, Clone)]
pub struct UUID(pub [u64; 2]);

#[derive(Debug)]
enum ParseErrorReason {
    VarIntTooBig,
    VarLongTooBig,
}

impl Display for ParseErrorReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl Error for ParseErrorReason {}

#[derive(Debug)]
struct ParseError(ParseErrorReason);

impl Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self)
    }
}

impl std::error::Error for ParseError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        Some(&self.0)
    }
}



pub trait PacketType {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>>;
}



// **********************************************************

// *** Actual Packet Type Implementations *******************

// **********************************************************


impl PacketType for bool {
    fn read<R: Read>(r: &mut R) -> Result<bool, Box<dyn std::error::Error>> {
        let mut byte = [0];
        r.read_exact(&mut byte)?;
        return match byte[0] {
            0 => Ok(false),
            _ => Ok(true),
        }
    }
    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        return match &self {
            true => {
                w.write(&[1])?;
                Ok(())
            },
            false => {
                w.write(&[0])?;
                Ok(())
            },
        }
    }
}

impl PacketType for i8 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut byte = [0];
        r.read_exact(&mut byte)?;
        Ok(byte[0] as i8)
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&[*self as u8])?;
        Ok(())
    }
}

impl PacketType for u8 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut byte = [0];
        r.read_exact(&mut byte)?;
        Ok(byte[0])
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&[*self])?;
        Ok(())
    }
}

impl PacketType for i16 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 2];
        r.read_exact(&mut bytes)?;
        Ok(i16::from_be_bytes(bytes))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for u16 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 2];
        r.read_exact(&mut bytes)?;
        Ok(u16::from_be_bytes(bytes))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.to_be_bytes())?;
        Ok(())
    }
}


impl PacketType for i32 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 4];
        r.read_exact(&mut bytes)?;
        Ok(i32::from_be_bytes(bytes))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for i64 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes)?;
        Ok(i64::from_be_bytes(bytes))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for f32 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 4];
        r.read_exact(&mut bytes)?;
        Ok(f32::from_be_bytes(bytes))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for f64 {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes)?;
        Ok(f64::from_be_bytes(bytes))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for String {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let VarInt(len) = VarInt::read(r)?;
        let mut bytes = vec![0u8; len as usize];
        r.read_exact(&mut bytes)?;
        Ok(String::from_utf8(bytes)?)
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.as_bytes();
        VarInt(bytes.len() as i32).write(w)?;
        w.write(&bytes)?;
        Ok(())
    }
}

impl PacketType for VarInt {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        const PART: u32 = 0x7F;
        let mut size = 0;
        let mut val = 0u32;
        let mut byte: [u8; 1] = [0];

        r.read_exact(&mut byte)?;

        loop {
            val |= (byte[0] as u32 & PART) << (size * 7);
            size += 1;
            if size > 5 {
                return Err(Box::new(ParseError(ParseErrorReason::VarIntTooBig)))
            }
            if (byte[0] & 0x80) == 0 {
                break;
            }
            r.read_exact(&mut byte)?;
        }
        Ok(VarInt(val as i32))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: Vec<u8> = Vec::new();

        const PART: u32 = 0x7F;
        let mut val = self.0 as u32;
        loop {
            if (val & !PART) == 0 {
                buf.push(val as u8);
                break;
            }
            buf.push(val as u8 | !0x7F);
            val >>= 7;
        }
        w.write(&buf)?;
        Ok(())
    }
}

impl PacketType for VarLong {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        const PART: u64 = 0x7F;
        let mut size = 0;
        let mut val = 0u64;
        let mut byte: [u8; 1] = [0];

        r.read_exact(&mut byte)?;

        loop {
            val |= (byte[0] as u64 & PART) << (size * 7);
            size += 1;
            if size > 10 {
                return Err(Box::new(ParseError(ParseErrorReason::VarLongTooBig)))
            }
            if (byte[0] & 0x80) == 0 {
                break;
            }
            r.read_exact(&mut byte)?;
        }
        Ok(VarLong(val as i64))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: Vec<u8> = Vec::new();

        const PART: u64 = 0x7F;
        let mut val = self.0 as u64;
        loop {
            if (val & !PART) == 0 {
                buf.push(val as u8);
                break;
            }
            buf.push(val as u8 | !0x7F);
            val >>= 7;
        }
        w.write(&buf)?;
        Ok(())
    }
}

impl PacketType for Metadata {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl PacketType for Slot {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        todo!()
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}

impl PacketType for NBTTag {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let (out, _) = quartz_nbt::io::read_nbt(r, quartz_nbt::io::Flavor::Uncompressed)?;
        Ok(NBTTag(out))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let mut buf: Vec<u8> = Vec::new();
        quartz_nbt::io::write_nbt(&mut buf, None, &self.0, quartz_nbt::io::Flavor::Uncompressed)?;
        Ok(())
    }
}


impl PacketType for Position {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes)?;

        let big = u64::from_be_bytes(bytes);

        let x = (big >> 38) as i32;
        let y = (big & 0xfff) as i32;
        let z = (big << 26 >> 38) as i32;

        // This may be a source of error

        Ok(Position(x, y, z))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let big = (((self.0 as u64) & 0x3FFFFFF) << 38)
            | (((self.2 as u64) & 0x3FFFFFF) << 12)
            | ((self.1 as u64) & 0xFFF);

        w.write(&big.to_be_bytes())?;
        Ok(())
    }
}


impl PacketType for UUID {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut b1 = [0u8; 8];
        let mut b2 = [0u8; 8];

        r.read_exact(&mut b1)?;
        r.read_exact(&mut b2)?;

        Ok(UUID([u64::from_be_bytes(b1), u64::from_be_bytes(b2)]))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0[0].to_be_bytes())?;
        w.write(&self.0[1].to_be_bytes())?;
        Ok(())
    }
}

impl<T: PacketType> PacketType for Vec<T> {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let len = VarInt::read(r)?;
        let mut vec = Vec::new();


        for _ in 0..(len.0) {
            let t = T::read(r)?;
            vec.push(t);
        }

        Ok(vec)
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        VarInt(self.len() as i32).write(w)?;
        for t in self {
            t.write(w)?;
        }

        Ok(())
    }
}