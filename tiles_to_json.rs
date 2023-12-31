///
/// # tiles_to_json.rs
/// This file will be a binary, it will take a folder as an argument and will output a json file.
///
/// Tom Planche <github.com/tomPlanche>

// Imports  ==============================================================================  Imports
#[path = "file_utils.rs"]
mod file_utils;

use json::JsonValue;

use std::{
    env::{
        current_dir,
        args,
    },
    fs::{
        File
    },
    io::Write,
    path::Path,
};

use crate::file_utils::{iterate_over_folder, Mode};
// Variables  =========================================================================== Variables

// Functions  =========================================================================== Functions
fn main() {
    // Folder caller - the folder from which the program was called
    let caller = current_dir().unwrap();

    // Folder to read from
    let folder_path = match args().nth(1) {
        Some(folder_path) => folder_path,
        None => panic!("Please provide a folder path"),
    };

    let file_destination = match args().nth(2) {
        Some(file_destination) => {
            if !file_destination.ends_with(".json") {
                panic!("The file destination must end with .json");
            }

            file_destination
        },
        None => "output/tiles_struct.json".to_string(),
    };

    if !Path::new(&folder_path).exists() {
        panic!("The folder {:?} does not exist", folder_path);
    }

    let mut final_json: JsonValue = json::object! {};

    // Iterate over the folder.
    // If the element is a file, add it to the final json.
    // If the element is a folder, add it to the final json and iterate over it.
    let final_json = iterate_over_folder(
        final_json,
        Path::new(&folder_path),
        Mode::Tiles,
        None
    );

    // create/recreate the output file
    let mut file = File::create(format!("{}/{}", caller.display(), file_destination)).unwrap();

    // write the final json to the file
    file.write_all(final_json.dump().as_bytes()).unwrap();
}

/*
 * End of file /tiles_to_json.rs
 */
