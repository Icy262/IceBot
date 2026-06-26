use std::io::{Read, Write};
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use ucs2;
#[derive(Clone)]
pub(crate) struct MCByte {
	pub(crate) value: i8,
}

impl MCByte {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_i8().expect("Should be able to read i8 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_i8(data.value).expect("Should be able to write i8 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCUByte {
	pub(crate) value: u8,
}

impl MCUByte {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_u8().expect("Should be able to read u8 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_u8(data.value).expect("Should be able to write u8 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCShort {
	pub(crate) value: i16,
}

impl MCShort {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_i16::<BigEndian>().expect("Should be able to read i16 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_i16::<BigEndian>(data.value).expect("Should be able to write i16 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCInt {
	pub(crate) value: i32,
}

impl MCInt {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_i32::<BigEndian>().expect("Should be able to read i32 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_i32::<BigEndian>(data.value).expect("Should be able to write i32 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCLong {
	pub(crate) value: i64,
}

impl MCLong {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_i64::<BigEndian>().expect("Should be able to read i64 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_i64::<BigEndian>(data.value).expect("Should be able to write i64 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCFloat {
	pub(crate) value: f32,
}

impl MCFloat {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_f32::<BigEndian>().expect("Should be able to read f32 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_f32::<BigEndian>(data.value).expect("Should be able to write f32 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCDouble {
	pub(crate) value: f64,
}

impl MCDouble {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_f64::<BigEndian>().expect("Should be able to read f64 from stream");
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_f64::<BigEndian>(data.value).expect("Should be able to write f64 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCString8 {
	pub(crate) length: MCShort,
	pub(crate) text: String,
}

impl MCString8 {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let length = MCShort::read(stream);
let mut buffer = vec![0u8; length.value as usize];
stream.read_exact(&mut buffer).expect("Should be able to read length u8 from stream");
let text = String::from_utf8(buffer).expect("Should be able to convert array of utf8 bytes to string");
		Self {
			length: length,
			text: text,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
MCShort::write(stream, data.length);
stream.write_all(data.text.as_bytes()).expect("Should be able to write all bytes of utf8 to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCString16 {
	pub(crate) length: MCShort,
	pub(crate) text: String,
}

impl MCString16 {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let length = MCShort::read(stream);
let mut buffer = vec![0u8; 2 * length.value as usize];
stream.read_exact(&mut buffer).expect("Should be able to read exact number of bytes");
let text = String::from_utf16(&buffer.chunks_exact(2).map(|character_bytes| u16::from_be_bytes([character_bytes[0], character_bytes[1]])).collect::<Vec<u16>>()).expect("Should be valid UCS2 data");
		Self {
			length: length,
			text: text,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
let mut encoded = vec![0u16; data.length.value as usize];
let _ = ucs2::encode(&data.text, &mut encoded).unwrap();
MCShort::write(stream, data.length);
stream
    .write_all(
        encoded
            .iter()
            .flat_map(|&u| u.to_be_bytes())
            .collect::<Vec<u8>>()
            .as_slice()
    )
    .unwrap();
	}
}

#[derive(Clone)]
pub(crate) struct MCBool {
	pub(crate) value: bool,
}

impl MCBool {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let value = stream.read_u8().expect("Should be able to read u8 from stream") == 1;
		Self {
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
stream.write_u8(data.value as u8).expect("Should be able to write bool (u8) to stream");
	}
}

#[derive(Clone)]
pub(crate) struct MCMetadata {
	pub(crate) metadata_type: MCUByte,
	pub(crate) value: MCUByte,
}

impl MCMetadata {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
//init
let metadata_type = MCUByte { value: 0 };
let value = MCUByte { value: 0 }; //TODO: implement storing value properly
loop {
    let metadata_type = MCUByte::read(stream);
    if metadata_type.value == 127 {
        break;
    } else {
        match metadata_type.value >> 5 {
            0 => { MCByte::read(stream); },
            1 => { MCShort::read(stream); },
            2 => { MCInt::read(stream); },
            3 => { MCFloat::read(stream); },
            4 => { MCString16::read(stream); },
            5 => { MCItem::read(stream); },
            6 => {
                for _ in 0..3 {
                    MCInt::read(stream);
                }
            }
            _ => { panic!("Illegal metadata value"); }, //TODO: replace with proper error handling
        }
    }
}
		Self {
			metadata_type: metadata_type,
			value: value,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
let _ = 1; //PLACEHOLDER
	}
}

#[derive(Clone)]
pub(crate) struct MCMapChunk {
	pub(crate) length: MCInt,
	pub(crate) compressed_data: Vec<MCUByte>,
}

impl MCMapChunk {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let length = MCInt::read(stream);
let mut compressed_data: Vec<MCUByte> = Vec::new();
for _ in 0..length.value {
    compressed_data.push(MCUByte::read(stream));
}
		Self {
			length: length,
			compressed_data: compressed_data,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
let _ = 1; //PLACEHOLDER
	}
}

#[derive(Clone)]
pub(crate) struct MCInventoryPayload {
	pub(crate) count: MCShort,
	pub(crate) payload: Vec<MCItem>,
}

impl MCInventoryPayload {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let count = MCShort::read(stream);
let mut payload = Vec::new();
for _ in 0..count.value {
    payload.push(
        MCItem::read(stream)
    )
}
		Self {
			count: count,
			payload: payload,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
let _ = 1; //PLACEHOLDER
	}
}

#[derive(Clone)]
pub(crate) struct MCBlockUpdateArray {
	pub(crate) length: MCShort,
	pub(crate) coordinate_array: Vec<MCShort>,
	pub(crate) type_array: Vec<MCByte>,
	pub(crate) metadata_array: Vec<MCUByte>,
}

impl MCBlockUpdateArray {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let length = MCShort::read(stream);
let mut coordinate_array: Vec<MCShort> = Vec::new();
let mut type_array: Vec<MCByte> = Vec::new();
//let mut metadata_array: Vec<MCMetadata> = Vec::new();
let mut metadata_array: Vec<MCUByte> = Vec::new();
for _ in 0..(length.value as usize) {
    coordinate_array.push(MCShort::read(stream));
}
for _ in 0..(length.value as usize) {
    type_array.push(MCByte::read(stream));
}
for _ in 0..(length.value as usize) {
    metadata_array.push(MCUByte::read(stream));
}
//for _ in 0..(length.value as usize) {
//    metadata_array.push(MCMetadata::read(stream));
//}
		Self {
			length: length,
			coordinate_array: coordinate_array,
			type_array: type_array,
			metadata_array: metadata_array,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
MCShort::write(stream, data.length.clone() );
for i in 0..data.length.value {
    MCShort::write(stream, data.coordinate_array[i as usize].clone());
}
for i in 0..data.length.value {
    MCByte::write(stream, data.type_array[i as usize].clone());
}
for i in 0..data.length.value {
    MCUByte::write(stream, data.metadata_array[i as usize].clone());
}
//for i in 0..data.length.value {
//    MCMetadata::write(stream, data.metadata_array[i as usize].clone());
//}
	}
}

#[derive(Clone)]
pub(crate) struct MCBlockCoordinate {
	pub(crate) x: MCByte,
	pub(crate) y: MCByte,
	pub(crate) z: MCByte,
}

impl MCBlockCoordinate {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
		let x = MCByte::read(stream);
		let y = MCByte::read(stream);
		let z = MCByte::read(stream);
		Self {
			x: x,
			y: y,
			z: z,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
		MCByte::write(stream, data.x);
		MCByte::write(stream, data.y);
		MCByte::write(stream, data.z);
	}
}

#[derive(Clone)]
pub(crate) struct MCExplosionUpdate {
	pub(crate) length: MCInt,
	pub(crate) records: Vec<MCBlockCoordinate>,
}

impl MCExplosionUpdate {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let length = MCInt::read(stream);
let mut records: Vec<MCBlockCoordinate> = Vec::new();
for _ in 0..length.value {
    records.push(MCBlockCoordinate::read(stream))
}
		Self {
			length: length,
			records: records,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
let _ = 1; //PLACEHOLDER
	}
}

#[derive(Clone)]
pub(crate) struct MCItem {
	pub(crate) item_id: MCShort,
	pub(crate) count: MCByte,
	pub(crate) uses: MCShort,
}

impl MCItem {
	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {
let item_id = MCShort::read(stream);

//the server will only send the count and uses if the id is not -1 (-1 means empty slot)
let count = if item_id.value != -1 {
    MCByte::read(stream)
} else {
    MCByte {
        value: 0,
    }
};

let uses = if item_id.value != -1 {
    MCShort::read(stream)
} else {
    MCShort {
        value: 0,
    }
};
		Self {
			item_id: item_id,
			count: count,
			uses: uses,
		}
	}

	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {
let _ = 1; //PLACEHOLDER
	}
}

