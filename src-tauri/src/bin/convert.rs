use std::fs::File;
use std::io::{BufReader, Write};
use std::path::Path;
use mxminimax_lib::val_types::{Node};

/// Reads a JSON file from disk and parses it into your custom Rust struct using Serde
fn json_to_struct<P: AsRef<Path>>(path: P) -> Result<Node, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data: Node = serde_json::from_reader(reader)?;
    Ok(data)
}

/// Serializes the struct using wincode and writes the raw binary data to a .bin file
fn struct_to_wincode_file<P: AsRef<Path>>(data: &Node, path: P) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Serialize directly into a vector of bytes using wincode API
    let encoded_bytes: Vec<u8> = wincode::serialize(data)?;
    
    // 2. Write the raw bytes to the specified binary file path
    let mut file = File::create(path)?;
    file.write_all(&encoded_bytes)?;
    
    Ok(())
}

fn main() {
    let json_input_path = "C:/Users/Dell/Documents/projects/minimizerTree.json";
    let binary_output_path = "C:/Users/Dell/Documents/projects/minimizerTree.bin";

    // Step 1: JSON -> Rust Struct
    match json_to_struct(json_input_path) {
        Ok(my_struct) => {
            println!("Parsed JSON into Rust Struct successfully!");

            // Step 2: Rust Struct -> .bin File via wincode
            match struct_to_wincode_file(&my_struct, binary_output_path) {
                Ok(_) => println!("Successfully saved binary data using wincode to '{}'", binary_output_path),
                Err(e) => eprintln!("Wincode serialization error: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Error reading/parsing JSON file: {}", e);
        }
    }
}
