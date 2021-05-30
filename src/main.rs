pub mod reactors;
pub mod structures;

use crate::reactors::load_reactors;
use std::process::exit;

// macro_rules! input {
//     ($prompt:expr) => {{
//         use std::io::prelude::*;
//         print!("{}", $prompt);
//         std::io::stdout().flush().unwrap();
//         let mut temp = String::new();
//         std::io::stdin().read_line(&mut temp).unwrap();
//         temp.trim().to_owned()
//     }};
// }

fn main() {
    let used_structures = match load_reactors() {
        Ok(v) => v,
        Err(err) => {
            eprintln!("{}", err);
            exit(-1);
        }
    };

    println!("=> Imported structures {:?}", used_structures)

    // let parsed = parse(&file, path.as_str());

    // let structure = match parsed {
    //     Ok(result) => result,
    //     Err(e) => {
    //         eprintln!("{}", e);
    //         exit(-1);
    //     }
    // };

    // let serialized = bincode::serialize(&structure).unwrap();

    // let compiled_file_name = format!("out/{}.bin", &path);

    // let mut output_file = fs::File::create(compiled_file_name).unwrap();

    // output_file.write_all(serialized.as_slice()).unwrap();
}
