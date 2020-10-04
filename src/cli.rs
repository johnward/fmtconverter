/*
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license - Copyright (c) 2020 John Ward
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
*/

extern crate clap;
use clap::{App, Arg};

pub fn get_filename_and_cells() -> (bool, String) {
    let mut is_deleted = true;

    let matches = App::new("Convert m4a files ro mp3s v0.1")
        .version("0.1")
        .author("John Ward <john@johnward.net>")
        .about("Utility to convert m4a files to mp3s")
        .arg(
            Arg::with_name("delete")
                .short("d")
                .long("delete")
                .help("deletes the old value"),
        )
        .arg(
            Arg::with_name("INPUT_DIRECTORY")
                .help("Sets input directory")
                .required(true)
                .index(1),
        )
        .get_matches();

    // Vary the output based on how many times the user used the "verbose" flag
    // (i.e. 'myprog -v -v -v' or 'myprog -vvv' vs 'myprog -v'
    match matches.occurrences_of("delete") {
        0 => println!("No verbose info"),
        1 | _ => {
            is_deleted = true;
            println!("Some verbose info");
        }
    }

    let dir_name = String::from(matches.value_of("INPUT_DIRECTORY").unwrap_or("."));

    (is_deleted, dir_name)
}
