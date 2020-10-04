/*
## License

Licensed under either of

 * Apache License, Version 2.0
   ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license
   ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.
*/

use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

mod cli;

fn main() -> io::Result<()> {
    let (is_delete, dir_name) = cli::get_filename_and_cells();

    //let path_string = String::from("/home/john/Music/VanMusicTest");

    println!("Searching im {}", dir_name);

    let path = PathBuf::from(dir_name);

    parse_files(&path, is_delete, &convert_file_and_delete)?;

    Ok(())
}

fn convert_file_and_delete(file: &DirEntry, is_deleted: bool) {
    println!("Attempting to convert {}", file.path().to_str().unwrap());

    let mut newfile = file.path().clone();
    newfile.set_extension("mp3");

    // let filen = file.path().to_str().unwrap(); //.file_name().and_then(OsStr::to_str).unwrap();
    // let new_filen = newfile.to_str().unwrap(); //.file_name().and_then(OsStr::to_str).unwrap();
    // println!("Converting {} to {}", filen, new_filen);

    if file
        .path()
        .file_name()
        .and_then(OsStr::to_str)
        .unwrap()
        .as_bytes()[0]
        == b'.'
    {
        println!(
            "Ignoring file (hidden file): {}",
            file.path().to_str().unwrap()
        );
        return;
    }

    if file.path().extension().and_then(OsStr::to_str).unwrap() == "m4a"
        && newfile.extension().and_then(OsStr::to_str).unwrap() == "mp3"
    {
        println!("Converting {}", file.path().to_str().unwrap());

        let output = Command::new("ffmpeg")
            .arg("-i")
            .arg(file.path().to_str().unwrap())
            .arg("-codec:v")
            .arg("copy")
            .arg("-codec:a")
            .arg("libmp3lame")
            .arg("-q:a")
            .arg("2")
            .arg(newfile.to_str().unwrap()) // TO CHANGE
            .output()
            .expect("Failed to do convert");

        println!("status: {}", output.status);
        io::stdout().write_all(&output.stdout).unwrap();
        io::stderr().write_all(&output.stderr).unwrap();

        assert!(output.status.success());

        if is_deleted {
            println!("Deleting {}", file.path().to_str().unwrap());
            let output = Command::new("rm")
                .arg(file.path().to_str().unwrap())
                .output()
                .expect("Failed to delete");

            println!("status: {}", output.status);
            io::stdout().write_all(&output.stdout).unwrap();
            io::stderr().write_all(&output.stderr).unwrap();

            assert!(output.status.success());
        }
    } else {
        println!("Ignoring file: {}", file.path().to_str().unwrap());
    }

    // for f in *.m4a; do ffmpeg -i "$f" -codec:v copy -codec:a libmp3lame -q:a 2 newfiles/"${f%.m4a}.mp3"; done
}

fn parse_files(
    dir: &Path,
    is_deleted: bool,
    convert_file: &dyn Fn(&DirEntry, bool),
) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                parse_files(&path, is_deleted, convert_file)?;
            } else {
                convert_file(&entry, is_deleted);
            }
        }
    }

    Ok(())
}
