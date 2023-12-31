/// # reader.rs
/// This module will compare the GPX files in the 'gpx_files' folder and will output a JSON file
/// with a map of the common coordinates between the files.
///
/// The final map will look like this:
/// ```json
/// {
///   "file_1": {
///     "file_2": [(index_file_1, index_file_2), ...],
///     "file_3": [(index_file_1, index_file_3), ...],
///     ...
///   },
///   "file_2": {
///     ...
///   },
///   ...
/// }
/// ```
///
/// ## Author
/// * Tom Planche - <github.com/tomPlanche>

// IMPORTS ===================================================================================================  IMPORTS
#[path = "gpx_utils.rs"]
mod gpx_utils;

#[path = "file_utils.rs"]
mod file_utils;

#[path = "utils.rs"]
mod utils;

use std::path::PathBuf;
use file_utils::get_final_json_path;
use ansi_term::Colour::{Green, Red};


use crate::file_utils::{load_from_json, file_name_to_path_buf, read_gpx_file};
use crate::gpx_utils::Coord;
use crate::utils::{FileCoordsHM};
// END IMPORTS ==========================================================================================   END IMPORTS


// FUNCTIONS ================================================================================================ FUNCTIONS
///
/// # check_if_file_exists
/// Checks if the output file exists.
///
/// ## Returns
/// * `bool` - True if the file exists, false otherwise
fn check_if_file_exists() -> bool {
    get_final_json_path().exists()
}

///
/// # read_from_file
/// Read from the output file and returns the hashmap.
///
/// ## Returns
/// * `FileCoordsHM` - The hashmap read from the file
fn read_from_file() -> FileCoordsHM {
    if !check_if_file_exists() {
        println!("{}", Red.paint("The output file does not exist"));
        println!("{}", Green.paint("Run the comparator first"));

        return FileCoordsHM::new();
    }

    let path_buff: PathBuf = get_final_json_path();

    load_from_json(&path_buff)
}

///
/// # find_common_coords
/// Find the common coordinates between the two files.
///
/// ## Arguments
/// * `file_1` - The first file
/// * `file_2` - The second file
///
/// ## Returns
/// * `Vec<(usize, usize)>` - The vector of common coordinates
fn find_common_coords_indexes(
    file_1: &str,
    file_2: &str
) -> Vec<(usize, usize)> {
    let mut common_coords: Vec<(usize, usize)> = Vec::new();
    let file_coords_map: FileCoordsHM = read_from_file();

    let (final_file_1, final_file_2): (String, String) = if file_coords_map.contains_key(file_2) {
        (file_2.to_string(), file_1.to_string())
    } else {
        (file_1.to_string(), file_2.to_string())
    };

    if file_coords_map.contains_key(&final_file_1) && file_coords_map[&final_file_1].contains_key(&final_file_2) {
        common_coords = file_coords_map[&final_file_1][&final_file_2].clone();
    }

    common_coords
}

///
/// # indexes_to_coords
/// Convert the indexes to coordinates.
///
/// ## Arguments
/// * `file` - The file to read the coordinates from
/// * `indexes` - The indexes to convert
///
/// ## Returns
/// * `Vec<(Coord, Coord)>` - The vector of coordinates
fn indexes_to_coords(file_1: &str, file_2: &str, indexes: &Vec<(usize, usize)>) -> Vec<(Coord, Coord)> {
    let path_buff_from_file_1: PathBuf = file_name_to_path_buf(file_1);
    let path_buff_from_file_2: PathBuf = file_name_to_path_buf(file_2);

    let coord_1: Vec<Coord> = match read_gpx_file(&path_buff_from_file_1) {
        Some(coords) => coords,
        None => panic!("Could not read the file {:?}", file_1),
    };

    let coord_2: Vec<Coord> = match read_gpx_file(&path_buff_from_file_2) {
        Some(coords) => coords,
        None => panic!("Could not read the file {:?}", file_2),
    };

    let mut coords_pairs: Vec<(Coord, Coord)> = Vec::new();

    for (index_1, index_2) in indexes {
        coords_pairs.push((
            coord_1[*index_1],
            coord_2[*index_2]
        ));
    }

    coords_pairs
}

fn main() {
    println!("{:?}", find_common_coords_indexes("puertoviejofenars.gpx", "EmbalseCuezoPradera.gpx"));
    print!("{:?}", indexes_to_coords("puertoviejofenars.gpx", "EmbalseCuezoPradera.gpx", &find_common_coords_indexes("puertoviejofenars.gpx", "EmbalseCuezoPradera.gpx")));
}

// END FUNCTIONS =======================================================================================  END FUNCTIONS

//
// * End of file file_utils.rs
//
