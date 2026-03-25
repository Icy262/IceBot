use std::{env::{self, var}, fs};
use yaml_rust::{Yaml, YamlLoader};
use ucs2::{encode, decode};

fn main() {
	let protocol_version =
		env::var("PROTOCOL_VERSION").expect("PROTOCOL_VERSION must be set as environment variable");

	println!("cargo:rerun-if-env-changed=PROTOCOL_VERSION");

	let protocol_specification_path =
		format!("network_protocol_specifications/{protocol_version}.yaml");

	let protocol_specification = fs::read_to_string(&protocol_specification_path)
		.expect(format!("Should be able to find protocol specification. Please check that there is a file named {}.yaml in protocol_specifications", protocol_version).as_str());

	//Defining packets.rs
	let mut output_code = String::new();

	//module declaration
	output_code += "pub(crate) mod packets {\n";
	output_code += "use std::io::{Read, Write};\n";
	output_code += "use crate::data_types::data_types::*;\n";

	let packets = YamlLoader::load_from_str(protocol_specification.as_str())
		.expect("protocol specification should be in valid yaml format")
		[0]["Packets"]
		.to_owned();

	for packet in packets {
		//extracting data
		let name = packet["name"].as_str().expect("Should be able to convert packet name from yaml to str").replace(" ", "");
		let id = packet["id"].as_i64().expect("Should be able to convert packet name from yaml to i64").to_string().replace(" ", "_").to_lowercase();
		let payload = packet["Data"].clone().into_vec().unwrap_or_default().into_iter().map(|data_item| {let (field, data_type) = data_item.as_hash().unwrap().iter().next().unwrap(); return (field.clone(), data_type.clone())}).collect::<Vec<(Yaml, Yaml)>>();

		//struct
		output_code += &format!("	pub(crate) struct {} {{\n", name);
		for (field, data_type) in payload.iter() {
			output_code += &format!("		pub(crate) {}: {},\n", field.as_str().expect("Should be able to convert yaml field value to &str").replace(" ", "_").to_lowercase(), mc_type_to_rust(Yaml::as_str(data_type).expect("Should be able to convert yaml type value to &str")));
		}
		output_code += "	}\n\n";

		//impl block
		output_code += &format!("	impl {} {{\n", name);
		//set const id
		output_code += &format!("		const ID: u8 = {};\n", id);

		//Define read
		output_code += "		pub(crate) fn read<R: Read>(stream: &mut R) -> Self {\n";
		output_code += "			Self {\n";
		for (field, data_type) in payload.iter() {
			output_code += &format!("				{}: {}::read(stream),\n", field.as_str().expect("Should be able to convert yaml key name to rust &str").replace(" ", "_").to_lowercase(), mc_type_to_rust(Yaml::as_str(data_type).expect("Should be able to convert yaml type value to &str")))
		}
		output_code += "			}\n";
		output_code += "		}\n\n";
		
		//Define write
		output_code += "		pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {\n";
		for (field, data_type) in payload.iter() {
			output_code += &format!("			{}::write(stream, data.{});\n", mc_type_to_rust(Yaml::as_str(data_type).expect("Should be able to convert yaml type value to &str")), field.as_str().expect("Should be able to convert yaml key name to rust &str").replace(" ", "_").to_lowercase());
		}
		output_code += "		}\n";
		output_code += "	}\n\n";
	}

	//module close
	output_code += "}\n";

	//write to packets
	fs::write("src/packets.rs", output_code).unwrap();

	//Define data_types.rs
	let mut output_code = String::new();
	output_code += "pub(crate) mod data_types {\n";
	output_code += "	use std::io::{Read, Write};\n";
	output_code += "	use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};\n";

	let mc_types = YamlLoader::load_from_str(protocol_specification.as_str())
		.expect("data type specification should be in valid yaml format")
		[0]["Types"]
		.to_owned();
	for mc_type in mc_types {
		//extracting data
		let name = mc_type["name"].as_str().expect("All data types should contain a minecraft name").replace(" ", "");
		let rust_equivalent = mc_type["Rust Equivalent"].as_str().expect("All data types should contain a rust equivalent").replace(" ", "");
		let data_type_metadata = mc_type["Data"].clone().into_vec().unwrap_or_default().into_iter().map(|data_item| {let (field, data_type) = data_item.as_hash().unwrap().iter().next().unwrap(); return (field.clone(), data_type.clone())}).collect::<Vec<(Yaml, Yaml)>>();

		//define structs
		output_code += &format!("	pub(crate) struct {} {{\n", rust_equivalent);
		for (data_name, data_type) in data_type_metadata.iter() {
			output_code += &format!("		{}: {},\n", data_name.as_str().expect("Should be able to convert data name from yaml to str").replace(" ", "_").to_lowercase(), data_type.as_str().expect("Should be able to convert data type from yaml to str"));
		}

		output_code += "	}\n\n";

		// define impl block
		output_code += &format!("	impl {} {{\n", rust_equivalent);

		//define read
		if mc_type["arg"].is_badvalue() { //if it does not specifiy and argument, put the default read, else, add the argument
			output_code += "		pub(crate) fn read<R: Read>(stream: &mut R) -> Self {\n";
		} else {
			output_code += &format!("		pub(crate) fn read<R: Read>(stream: &mut R, {}) -> Self {{\n", mc_type["arg"].as_str().expect("Should be able to convert yaml value to str"));
		}
		//we must read the data before constructing a self to return, because if not we would not be able to acess the size of any vecs
		for (data_name, data_type) in data_type_metadata.iter() {
			output_code += &format!("			let {} = {};\n", data_name.as_str().expect("Should be able to convert data name from yaml to str").replace(" ", "_").to_lowercase(), read_generator(data_type.as_str().expect("Should be able to convert data type from yaml to str")));
		}
		//we have all data, so make self to return
		output_code += "			Self {\n";
		for (data_name, _data_type) in data_type_metadata.iter() {
			output_code += &format!("				{}: {},\n", data_name.as_str().expect("Should be able to convert data name from yaml to str").replace(" ", "_").to_lowercase(), data_name.as_str().expect("Should be able to convert data name from yaml to str").replace(" ", "_").to_lowercase());
		}
		output_code += "			}\n";
		output_code += "		}\n";

		//define write
		output_code += "		pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {\n";
		for (data_name, data_type) in data_type_metadata.iter() {
			output_code += &format!("				{};\n", write_generator(data_type.as_str().expect("Should be able to convert data type from yaml to str"), &data_name.as_str().expect("Should be able to convert data name from yamml to str").replace(" ", "_").to_lowercase())); //NOP, to be implemented later
		}
		output_code += "		}\n";

		output_code += "	}\n\n";
	}
	
	//module close
	output_code += "}";
	
	//write to data types
	fs::write("src/data_types.rs", output_code).unwrap();
}

fn read_generator(data_type: &str) -> String {
	match data_type {
		"u8" => "stream.read_u8().expect(\"Should be able to read u8 from stream\")",
		"i8" => "stream.read_i8().expect(\"Should be able to read i8 from stream\")",
		"i16" => "stream.read_i16::<BigEndian>().expect(\"Should be able to read i16 from stream\")",
		"i32" => "stream.read_i32::<BigEndian>().expect(\"Should be able to read i32 from stream\")",
		"i64" => "stream.read_i64::<BigEndian>().expect(\"Should be able to read i64 from stream\")",
		"f32" => "stream.read_f32::<BigEndian>().expect(\"Should be able to read f32 from stream\")",
		"f64" => "stream.read_f64::<BigEndian>().expect(\"Should be able to read f64 from stream\")",
		"StringUTF8" => "String::from_utf8(stream.read_exact(vec![0u8; stream.read_i16::<BigEndian>().expect(\"Should be able to read i16 length from stream\") as usize]).expect(\"Should be able to read length bytes from stream\")).expect(\"Should be able to convert array of utf8 bytes to string\")",
		"StringUCS2" => "",
		"bool" => "stream.read_u8().expect(\"Should be able to read u8 from stream\") == 1",
		"MCMetadata" => "",
		"MCMapChunk" => "",
		"MCBlockMetadataArray" => "",
		"MCBlockTypeArray" => "",
		"MCBlockCoordinateArray" => "",
		"MCInventoryPayload" => "",
		"MCBlockUpdateArray" => "",
		"MCExplosionUpdateArray" => "",
		_ => if data_type.contains("Vec<") { //vec reading will fail if length not specified before (must be "length", not something else). Additionally, the type must be able to read from the stream using the syntax stream.read_TYPE::<BigEndian>()
				return "{ let mut temp = Vec::new();\nfor _ in 0..length {\n	temp.push(stream.read_".to_owned() + data_type.get(4..data_type.len()-1).expect("Should be able to extract vec type") + "().unwrap());\n}\n temp}";
			} else {
				panic!("data_type should be one of the types in this match (was {:?})", data_type);
			}
	}.to_string()
}

fn write_generator(data_type: &str, variable_name: &str) -> String {
	match data_type {
		"u8" => format!("stream.write_u8({}).expect(\"Should be able to write u8 to stream\")", variable_name),
		"i8" => format!("stream.write_i8({}).expect(\"Should be able to write i8 to stream\")", variable_name),
		"i16" => format!("stream.write_i16({}).expect(\"Should be able to write i16 to stream\")", variable_name),
		"i32" => format!("stream.write_i32({}).expect(\"Should be able to write i32 to stream\")", variable_name),
		"i64" => format!("stream.write_i64({}).expect(\"Should be able to write i64 to stream\")", variable_name),
		"f32" => format!("stream.write_f32({}).expect(\"Should be able to write f32 to stream\")", variable_name),
		"f64" => format!("stream.write_f64({}).expect(\"Should be able to write f64 to stream\")", variable_name),
		"StringUTF8" => format!("{{\n	stream.write_i16::<BigEndian>({}.len() as i16).expect(\"Should be able to write length i16 to stream\");\n	stream.write_all({}).expect(\"Should be able to write all bytes of utf8 to stream\");\n}}", variable_name, variable_name),
		"StringUCS2" => format!(""),
		"bool" => format!(""),
		"MCMetadata" => format!(""),
		"MCMapChunk" => format!(""),
		"MCBlockMetadataArray" => format!(""),
		"MCBlockTypeArray" => format!(""),
		"MCBlockCoordinateArray" => format!(""),
		"MCInventoryPayload" => format!(""),
		"MCBlockUpdateArray" => format!(""),
		"MCEplosionUpdateArray" => format!(""),
		_ => if data_type.contains("Vec<") {
				return "{}".to_owned(); //TODO: impliment
			} else {
				panic!("data_type should be one of the types in this match (was {:?})", data_type)
			}
	}.to_string()
}

//TODO: remove and replace with the definitions in the yaml
fn mc_type_to_rust(mc_type: &str) -> &str {
	match mc_type {
		"byte" => "MCByte",
		"short" => "MCShort",
		"int" => "MCInt",
		"long" => "MCLong",
		"float" => "MCFloat",
		"double" => "MCDouble",
		"string8" => "MCString8",
		"string16" => "MCString16",
		"bool" => "MCBool",
		"metadata" => "MCMetadata",
		"MapChunk" => "MCMapChunk",
		"BlockMetadataArray" => "MCBlockMetadataArray",
		"BlockTypeArray" => "MCBlockTypeArray",
		"BlockCoordinateArray" => "MCBlockCoordinateArray",
		"InventoryPayload" => "MCInventoryPayload",
		"BlockUpdateArray" => "MCBlockUpdateArray",
		"ExplosionUpdateArray" => "MCExplosionUpdate",
		_ => panic!("{} {}", "yaml should not specify data types outside those defined in mc_type_to_rust", mc_type),
	}
}
