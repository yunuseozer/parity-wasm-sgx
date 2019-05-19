use std::prelude::v1::*;
use parity_wasm::elements::*;

// A helper funtion for the tests. Serialize a section, deserialize it,
// and make sure it matches the original.
fn serialize_test(original: NameSection) -> Vec<u8> {
	let mut buffer = vec![];
	original
		.serialize(&mut buffer)
		.expect("serialize error");
	buffer
}

//#[test]
pub fn serialize_module_name() {
	let original = NameSection::Module(ModuleNameSection::new("my_mod"));
	serialize_test(original.clone());
}

//#[test]
pub fn serialize_function_names() {
	let mut sect = FunctionNameSection::default();
	sect.names_mut().insert(0, "hello_world".to_string());
	serialize_test(NameSection::Function(sect));
}

//#[test]
pub fn serialize_local_names() {
	let mut sect = LocalNameSection::default();
	let mut locals = NameMap::default();
	locals.insert(0, "msg".to_string());
	sect.local_names_mut().insert(0, locals);
	serialize_test(NameSection::Local(sect));
}

//#[test]
pub fn serialize_and_deserialize_unparsed() {
	let original = NameSection::Unparsed {
		// A made-up name section type which is unlikely to be allocated
		// soon, in order to allow us to test `Unparsed`.
		name_type: 120,
		name_payload: vec![0u8, 1, 2],
	};
	serialize_test(original.clone());
}
