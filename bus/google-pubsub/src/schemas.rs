use serde::Serialize;
// use apache_avro::Schema;
// use apache_avro::Writer;
// use relative_path::RelativePath;
// use std::env::current_dir;
// use std::fs;

// fn load_topic_schema(topic: &str) -> Result<Schema, String> {
//     let root = current_dir().expect("No dir found");
//     let file_path = format!("bus/google-pubsub/src/avro/{}.json", topic);
//     let relative_path = RelativePath::new(&file_path);
//     let file_full_path = relative_path.to_logical_path(&root);
//     let panic_msg = format!("Schema file not found at: {:?}", file_full_path.to_str());
//     let contents = fs::read_to_string(file_full_path).expect(&panic_msg);
//     let schema = Schema::parse_str(contents.as_str()).expect("Bad schema file");
//     Ok(schema)
// }

// pub fn serialize_using_avro<T: Serialize>(topic: &str, data: T) -> Result<Vec<u8>, String> {
//     let schema = load_topic_schema(topic)?;
//     let mut writer = Writer::new(&schema, Vec::new());
//     writer.append_ser(data).unwrap();
//     let encoded = writer.into_inner().expect("Failed to encode");
//     Ok(encoded)
// }

#[derive(Debug, Serialize)]
pub struct Demo {
    pub event: String,
    pub value: i32,
}
