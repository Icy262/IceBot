use crate::block::Block;
use crate::world::WorldUpdate;
use crate::packets::Packets;
use crate::block::Coordinates;
use crate::data_types::MCUByte;
use crate::data_types::MCMetadata;
use std::io::Read;
use crate::world::Region;
use flate2::read::ZlibDecoder;

pub(crate) fn process_packet(packet: Packets) -> WorldUpdate {
	return match packet {
Packets::MapChunk(packet) => {
    //unwrap the compressed data out of MCUBytes and inflate it using zlib
    let unwraped_compressed = packet.map_chunk.compressed_data.iter().map(|mcbyte| mcbyte.value).collect::<Vec<u8>>();
    let mut decoder = ZlibDecoder::new(unwraped_compressed.as_slice());
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed);

    //get the dimensions of the region and the total number of blocks contained in it
    let size_x = packet.size_x.value as usize + 1;
    let size_y = packet.size_y.value as usize + 1;
    let size_z = packet.size_z.value as usize + 1;
    let num_blocks = size_x * size_y * size_z;

    //format the data into vecs of block types, metadata, light levels, and sky light
    let block_types: &[u8] = &decompressed[0..num_blocks];
    let block_metadata: &[u8] = &decompressed[num_blocks..(3*num_blocks/2) as usize];
    
    //unused
    //TODO: implement block and sky light levels
    let block_light = &decompressed[(3*num_blocks/2) as usize .. (2 * num_blocks) as usize];
    let sky_light = &decompressed[(2 * num_blocks) as usize .. (5*num_blocks/2) as usize];

    //generate the palette and local id array
    //TODO: implement metadata
    let mut palette: Vec<String> = Vec::new();
    let mut local_ids = vec![vec![vec![0u8; size_z]; size_y]; size_x];
    for current_x in 0..size_x {
        for current_y in 0..size_y {
            for current_z in 0..size_z {
                //get index of block type in array
                let index = current_y + (current_z * size_y) + (current_x * size_y * size_z); //see https://minecraft.wiki/w/Minecraft_Wiki:Projects/wiki.vg_merge/Protocol?oldid=2769763#Multi_Block_Change_(0x34) for reason on why this formula works

                //get block type
                let block_type = block_types[index].to_string();

                //convert to local block id
                let mut local_block_type = palette.iter().position(|block| *block == block_type);

                //if Some, then it was in the palette and we have the local id, if None, we should push the block type to the palette and use the index of the final element as the local id
                if local_block_type.is_none() {
                    palette.push(block_type);
                    local_block_type = Some(palette.len() - 1)
                }

                //push the local id to the current position
                local_ids[current_x][current_y][current_z] = local_block_type.expect("Should be impossible to be None because we check if local_block_type is None to see if it is in the palette") as u8;
            }
        }
    } 

    WorldUpdate::BlockRegion(
        Region {
            start_x: packet.x.value as i32,
            start_y: packet.y.value as i32,
            start_z: packet.z.value as i32,
            size_x: size_x as i32,
            size_y: size_y as i32,
            size_z: size_z as i32,
            palette: palette,
            local_ids: local_ids,
        }
    )
},
Packets::MultiBlockChange(packet) => {
    let mut blocks: Vec<Block> = Vec::new();
    for i in 0..packet.block_update_array.length.value as usize {
        blocks.push(
            Block {
                block_type: packet.block_update_array.type_array[i].value.to_string(),
                //metadata: packet.block_update_array.metadata_array[i].clone(),
                metadata: MCMetadata {
                    metadata_type: MCUByte { value: 0 }, //always a block metadata
                    value: MCUByte { value: packet.block_update_array.metadata_array[i].value },
                },
                position: Coordinates {
                    //bit shift and mask to isolate the important bits. top 4 bits are x, next 4 are z, final 8 are y
                    x: (packet.block_update_array.coordinate_array[i].value >> 12) as i32,
                    y: (packet.block_update_array.coordinate_array[i].value << 8) as i32,
                    z: ((packet.block_update_array.coordinate_array[i].value >> 8) & 0b1111) as i32,
                },
            }
        )
    }

    WorldUpdate::MultiBlock(blocks)
},
Packets::BlockChange(packet) => {
    WorldUpdate::SingleBlock(
        Block {
            block_type: packet.block_type.value.to_string(),
            metadata: MCMetadata {
                metadata_type: MCUByte {
                    value: 0
                },
                value: MCUByte {
                    value: 0
                },
            },
            position: Coordinates {
                x: packet.x.value as i32,
                y: packet.y.value as i32,
                z: packet.x.value as i32
            }
        }
    )
},
		_ => WorldUpdate::NoEffect,
	};
}

