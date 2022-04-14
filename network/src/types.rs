#![allow(dead_code)]

use std::error::Error;
use std::fmt::Display;
use std::io::{Read, Write, Cursor};

use quartz_nbt;

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
pub struct Metadata(); // TODO
#[derive(Debug, Clone)]
pub struct Slot(Boolean, Option<VarInt>, Option<Byte>, Option<NBTTag>); // TODO
#[derive(Debug, Clone)]
pub struct NBTTag(pub NbtCompound); // TODO
#[derive(Debug, Clone)]
pub struct Position(pub i32, pub i32, pub i32); // TODO
pub type Angle = UByte;
#[derive(Debug, Clone)]
pub struct UUID(pub [u64; 2]);
#[derive(Debug, Clone)]
pub struct Array<T: PacketType>(pub Vec<T>);

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


impl PacketType for Boolean {
    fn read<R: Read>(r: &mut R) -> Result<Boolean, Box<dyn std::error::Error>> {
        let mut byte = [0];
        r.read_exact(&mut byte)?;
        return match byte[0] {
            0 => Ok(Boolean(true)),
            _ => Ok(Boolean(true)),
        }
    }
    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        return match &self {
            Boolean(true) => {
                w.write(&[1])?;
                Ok(())
            },
            Boolean(false) => {
                w.write(&[0])?;
                Ok(())
            },
        }
    }
}

impl PacketType for Byte {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut byte = [0];
        r.read_exact(&mut byte)?;
        Ok(Byte(byte[0] as i8))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&[self.0 as u8])?;
        Ok(())
    }
}

impl PacketType for UByte {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut byte = [0];
        r.read_exact(&mut byte)?;
        Ok(UByte(byte[0]))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&[self.0])?;
        Ok(())
    }
}

impl From<UByte> for u8 {
    fn from(b: UByte) -> Self {
        b.0
    }
}

impl PacketType for Short {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 2];
        r.read_exact(&mut bytes)?;
        Ok(Short(i16::from_be_bytes(bytes)))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for UShort {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 2];
        r.read_exact(&mut bytes)?;
        Ok(UShort(u16::from_be_bytes(bytes)))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}


impl PacketType for Int {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 4];
        r.read_exact(&mut bytes)?;
        Ok(Int(i32::from_be_bytes(bytes)))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for Long {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes)?;
        Ok(Long(i64::from_be_bytes(bytes)))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for Float {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 4];
        r.read_exact(&mut bytes)?;
        Ok(Float(f32::from_be_bytes(bytes)))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for Double {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let mut bytes = [0u8; 8];
        r.read_exact(&mut bytes)?;
        Ok(Double(f64::from_be_bytes(bytes)))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        w.write(&self.0.to_be_bytes())?;
        Ok(())
    }
}

impl PacketType for MCString {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let VarInt(len) = VarInt::read(r)?;
        let mut bytes = vec![0u8; len as usize];
        r.read_exact(&mut bytes)?;
        let str = String::from_utf8(bytes)?;
        Ok(MCString(str))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = self.0.as_bytes();
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

impl<T: PacketType> PacketType for Array<T> {
    fn read<R: Read>(r: &mut R) -> Result<Self, Box<dyn std::error::Error>> where Self: Sized {
        let len = VarInt::read(r)?;
        let mut vec = Vec::new();


        for _ in 0..(len.0) {
            let t = T::read(r)?;
            vec.push(t);
        }

        Ok(Array(vec))
    }

    fn write<W: Write>(&self, w: &mut W) -> Result<(), Box<dyn std::error::Error>> {
        VarInt(self.0.len() as i32).write(w)?;
        for t in &self.0 {
            t.write(w)?;
        }

        Ok(())
    }
}

impl Into<Vec<u8>> for &Array<UByte> {
    fn into(self) -> Vec<u8> {
        self.0.iter().map(|b| b.0).collect()
    }
}

// impl<T: PacketType> Into<&Vec<T>> for &Array<T> {
//     fn into(&self) -> &Vec<T> {
//         self.0
//     }
// }