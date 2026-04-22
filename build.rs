use std::{env, fs, process::Output};
use yaml_rust::{Yaml, YamlLoader};

fn main() {
	let version =
		env::var("GAME_VERSION").expect("GAME_VERSION must be set as environment variable");

	println!("cargo:rerun-if-env-changed=GAME_VERSION");

	let version_spec_path = format!("Version Config/{version}.yaml");

	let version_spec = YamlLoader::load_from_str(&fs::read_to_string(&version_spec_path)
		.expect(&format!("Should be able to find protocol specification. Please check that there is a file named {version}.yaml in network_protocol_specifications")))
		.expect("Should be able to convert string of yaml to yaml object")
		[0].to_owned();

	generate_data_types(version_spec["Types"].clone());
	generate_packets(version_spec["Packets"].clone());
	action_translation_generator(version_spec["Actions"].clone());
}

fn generate_data_types(spec: Yaml) {
	let mut output_code = String::new();
	
	//imports
	output_code += "use std::io::{Read, Write};\n";
	output_code += "use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};\n";
	output_code += "use ucs2;\n";

	for mc_type in spec {
		//extract data
		let _mc_name = mc_type["MC name"]
			.as_str()
			.expect("All data types should contain a minecraft name")
			.replace(" ", "");
		let rust_equivalent = mc_type["Rust Equivalent"]
			.as_str()
			.expect("All data types should contain a rust equivalent");
		//if the data type is an elementary data type, or requires complex logic to read and write, allows for a manual definition to be specified
		let _special = mc_type["special"]
			.as_bool()
			.unwrap_or(false);
		let contained_data = extract_contained_data(&mc_type);

		//struct definition
		output_code += "#[derive(Clone)]\n";
		output_code += &format!("pub(crate) struct {rust_equivalent} {{\n");
		for (data_name, data_type) in contained_data.iter() {
			output_code += &format!(
				"	pub(crate) {}: {},\n",
				data_name
					.as_str()
					.expect("Should be able to convert data name from yaml to str")
					.replace(" ", "_")
					.to_lowercase(),
				data_type
					.as_str()
					.expect("Should be able to convert data type from yaml to str")
			);
		}
		output_code += "}\n\n";

		//impl block opening
		output_code += &format!("impl {rust_equivalent} {{\n");

		//read definition
		output_code += "	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {\n";
		//we must read the data before constructing a self to return, because if not we would not be able to acess the size of any vecs
		output_code += &read_generator(&mc_type);
		//we have all data, so make self to return
		output_code += "		Self {\n";
		for (data_name, _data_type) in contained_data.iter() {
			output_code += &format!(
				"			{0}: {0},\n",
				data_name
					.as_str()
					.expect("Should be able to convert data name from yaml to str")
					.replace(" ", "_")
					.to_lowercase()
			);
		}
		output_code += "		}\n";
		output_code += "	}\n";

		//insert empty line between read and write
		output_code += "\n";

		//write definition
		output_code += "	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {\n";
		output_code += &write_generator(&mc_type);
		//close write
		output_code += "	}\n";

		//close impl
		output_code += "}\n\n";
	}

	//write to data types
	fs::write("src/data_types.rs", output_code).unwrap();
}

fn generate_packets(spec: Yaml) {
	let mut output_code = String::new();

	//imports
	output_code += "use std::io::{Read, Write};\n";
	output_code += "use crate::data_types::*;\n";
	output_code += "use byteorder::WriteBytesExt;\n\n";

	//insert enum of packets
	output_code += &packet_enum_generator(spec.clone());

	//insert packet definitions
	for packet in spec {
		//extracting data
		let name = packet["name"]
			.as_str()
			.expect("Should be able to convert packet name from yaml to str")
			.replace(" ", "");
		let id = packet["id"]
			.as_i64()
			.expect("Should be able to convert packet name from yaml to i64")
			.to_string()
			.replace(" ", "_")
			.to_lowercase();
		//TODO: clean this up
		let payload = packet["Data"]
			.clone()
			.into_vec()
			.unwrap_or_default()
			.into_iter()
			.map(|data_item| {
				let (field, data_type) = data_item.as_hash().unwrap().iter().next().unwrap();
				return (field.clone(), data_type.clone());
			})
			.collect::<Vec<(Yaml, Yaml)>>();

		//struct definition
		output_code += &format!("pub(crate) struct {name} {{\n");
		for (field, data_type) in payload.iter() {
			output_code += &format!(
				"	pub(crate) {}: {},\n",
				field
					.as_str()
					.expect("Should be able to convert yaml field value to &str")
					.replace(" ", "_")
					.to_lowercase(),
				mc_type_to_rust(
					Yaml::as_str(data_type)
						.expect("Should be able to convert yaml type value to &str")
				)
			);
		}
		output_code += "}\n\n";

		//impl block opening
		output_code += &format!("impl {} {{\n", name);
		
		//set packet id as const
		output_code += &format!("	const ID: u8 = {id};\n");

		//Define read
		output_code += "	pub(crate) fn read<R: Read>(stream: &mut R) -> Self {\n";
		output_code += "		Self {\n";
		for (field, data_type) in payload.iter() {
			output_code += &format!(
				"			{}: {}::read(stream),\n",
				field
					.as_str()
					.expect("Should be able to convert yaml key name to rust &str")
					.replace(" ", "_")
					.to_lowercase(),
				mc_type_to_rust(
					Yaml::as_str(data_type)
						.expect("Should be able to convert yaml type value to &str")
				)
			)
		}
		output_code += "		}\n";
		output_code += "	}\n\n";

		//Define write
		output_code += "	pub(crate) fn write<W: Write>(stream: &mut W, data: Self) {\n";
		//write packet id
		output_code += "		stream.write_u8(Self::ID).expect(\"Should be able to write u8, packet ID to stream\");\n";
		for (field, data_type) in payload.iter() {
			output_code += &format!(
				"		{}::write(stream, data.{});\n",
				mc_type_to_rust(
					Yaml::as_str(data_type)
						.expect("Should be able to convert yaml type value to &str")
				),
				field
					.as_str()
					.expect("Should be able to convert yaml key name to rust &str")
					.replace(" ", "_")
					.to_lowercase()
			);
		}
		output_code += "	}\n";
		output_code += "}\n\n";
	}

	//write to packets
	fs::write("src/packets.rs", output_code).unwrap();
}

fn action_translation_generator(spec: Yaml) {
	let mut output_code = String::new();
	
	//import all packets and data types because we don't know what we might need
	output_code += "use crate::packets::*;\n";
	output_code += "use crate::data_types::*;\n\n";
	
	for action in spec {
		//extracting data
		let name = action["name"]
			.as_str()
			.expect("Should be able to convert action name from yaml to str");
		//TODO: clean this up
		let conversion = action["packet conversion"]
			.as_str()
			.expect("Should be able to convert packet conversion from yaml to str");

		//import the action we are implementing for
		output_code += &format!("use crate::actions::{name};\n");

		//impl block opening
		output_code += &format!("impl {} {{\n", name);
		
		//insert packet conversion
		output_code += &format!("	fn to_packets(action: {name}) -> Vec<Packets> {{\n");
		output_code += conversion;

		//function block closing
		output_code += "	}\n";

		//impl block closing
		output_code += "}\n\n"
	}

	//write to packets
	fs::write("src/action_translator.rs", output_code).unwrap();
}

fn packet_enum_generator(spec: Yaml) -> String {
	let mut output_code = String::new();

	//enum opening
		output_code += "pub(crate) enum Packets {\n";

	for packet in spec {
		//extracting name
		let name = packet["name"]
			.as_str()
			.expect("Should be able to convert packet name from yaml to str")
			.replace(" ", "");

		//insert packet
		output_code += &format!("	{name}({name}),\n");
	}

	//enum closing
	output_code += "}\n";

	return output_code;
}

fn read_generator(mc_type: &Yaml) -> String {
	if mc_type["special"].as_bool().unwrap_or(false) {
		return mc_type["special read"]
			.as_str()
			.expect("If special flag set to true, special read code must be defined")
			.to_owned();
	} else {
		let contained_data = extract_contained_data(mc_type);

		let mut output_code = String::new();

		for (data_name, data_type) in contained_data.iter() {
			output_code += &format!(
				"		let {} = {}::read(stream);\n",
				data_name
					.as_str()
					.expect("Should be able to convert data name from yaml to str")
					.replace(" ", "_")
					.to_lowercase(),
				data_type
					.as_str()
					.expect("Should be able to convert value data type from yaml to str")
			);
		}

		return output_code;
	}
}

fn write_generator(mc_type: &Yaml) -> String {
	if mc_type["special"].as_bool().unwrap_or(false) {
		return mc_type["special write"]
			.as_str()
			.expect("If special flag set to true, special write code must be defined")
			.to_owned()
			// .replace("{variable_name}", mc_type[]);
	} else {
		let mut output_code = String::new();

		let contained_data = extract_contained_data(&mc_type);
		
		for (variable_name, data_type) in contained_data.iter() {
			output_code += &format!(
				"		{}::write(stream, data.{});\n",
				data_type
					.as_str()
					.expect("Should be able to convert data type from yaml to str"),
				variable_name
					.as_str()
					.expect("Should be able to convert variable name from yaml to str")
					.to_lowercase()
					.replace(" ", "_")
			);
		}

		return output_code;
	}
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
		"Item" => "MCItem",
		_ => panic!("{} {}", "yaml should not specify data types outside those defined in mc_type_to_rust", mc_type),
	}
}

fn extract_contained_data(mc_type: &Yaml) -> Vec<(Yaml, Yaml)> {
	//TODO: clean this up
	return mc_type["Data"]
		.to_owned()
		.into_vec()
		.unwrap_or_default()
		.into_iter()
		.map(|data_item| {
			let (field, data_type) = data_item.as_hash().unwrap().iter().next().unwrap();
			return (field.clone(), data_type.clone());
		})
		.collect::<Vec<(Yaml, Yaml)>>();
}
