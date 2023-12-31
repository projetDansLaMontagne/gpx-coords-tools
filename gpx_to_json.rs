/// # gpx_to_json.rs
///
///
/// /// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
#[path = "gpx_utils.rs"]
mod gpx_utils;

#[path = "file_utils.rs"]
mod file_utils;

use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use file_utils::{file_name_to_path_buf, read_gpx_file};
use crate::file_utils::look_4_files;
use crate::gpx_utils::{Coord, Point};

// END IMPORTS ==========================================================================================   END IMPORTS

// VARIABLES ================================================================================================ VARIABLE
// Type(s)

// Other(s)
// END VARIABLES ======================================================================================= END VARIABLES

// CODE ========================================================================================================= CODE
///
/// # gpx_to_json
/// Convert a GPX file to a JSON file.
/// The JSON file will contain a list of Points.
///
/// ## Arguments
/// * `file_name` - The name of the file to read from.
/// * `file_destination` - The name of the file to write to.
///
/// ## Returns
/// * `bool` - True if the file was successfully saved, false otherwise.
fn gpx_to_json(file_name: String, file_destination: String) -> bool {
    let path_buff_from_file: PathBuf = file_name_to_path_buf(&file_name);

    let coords: Vec<Coord> = match read_gpx_file(&path_buff_from_file) {
        Some(coords) => coords,
        None => panic!("Could not read the file {:?}", file_name),
    };

    let nb_points: usize = coords.len();

    let points: Vec<Point> = coords
        .iter()
        .enumerate()
        .map(|(i, coord)| Point {
            coords: *coord,
            name: Some(format!("{}/{}", i, nb_points)),
            description: None,
            elevation: None,
        })
        .collect();

    let mut file = File::create(&file_destination).unwrap();

    match file.write_all(serde_json::to_string(&points).unwrap().as_bytes()) {
        Ok(_) => {
            println!("Successfully saved to: {}", file_destination);
            true
        }
        Err(error) => {
            println!("Error: {}", error);
            false
        }
    }
}

fn main() {
    let gpx_files: Vec<PathBuf> = look_4_files();

    for file in gpx_files {
        let file_name: String = match file.file_name() {
            Some(file_name) => file_name.to_str().unwrap().to_string(),
            None => panic!("Could not read the file name of {:?}", file),
        };

        // remove the extension
        let file_name_destination: String = file_name.split('.').collect::<Vec<&str>>()[0].to_string();

        let file_destination: String = format!("./output/{}.json", file_name_destination);

        gpx_to_json(file_name, file_destination);
    }
}
// END CODE =======================================================================================  END COMPONENT

//
// * End of file /gpx_to_json.rs
//
