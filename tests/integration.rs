use aws_imagebuilder_component::Component;
use std::fs::{read_dir, read_to_string};

#[test]
fn deserialize_and_reserialize() {
    for entry in read_dir("./test").expect("Failed to read the tests folder") {
        let entry = entry.expect("failed to load the entry");
        let yaml = read_to_string(entry.path())
            .unwrap_or_else(|e| panic!("Failed to read {:?}: {e}", entry.file_name()));
        let component: Component = serde_yaml::from_str(&yaml)
            .unwrap_or_else(|e| panic!("Failed to deserialize {:?}: {e}", entry.file_name()));

        let serialized = serde_yaml::to_string(&component)
            .unwrap_or_else(|e| panic!("Failed to serialize {:?}: {e}", entry.file_name()));

        println!("\n{serialized}\n");

        if serialized.contains("null") {
            panic!("Should not serialize 'None'")
        }
    }
}
