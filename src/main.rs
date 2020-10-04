use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::io;
use std::path::{Path, PathBuf};
use std::process::Command;

fn process_file<'a>(file: &'a DirEntry) {
    println!("Processing");

    // let ext = file.path().extension().and_then(OsStr::to_str);
    // let filename = file.path().file_stem().and_then(OsStr::to_str);
    // let orig_filename = file.path().file_name().and_then(OsStr::to_str);

    //let newfile = Path::from(&orig_filename);

    let mut newfile = file.path().clone();
    newfile.set_extension("mp3");

    let output = Command::new("ffmpeg")
        .arg("-i")
        .arg(file.path().file_name().and_then(OsStr::to_str).unwrap())
        .arg("-codec:v copy")
        .arg("-codec:a libmp3lame")
        .arg("-q:a 2")
        .arg(newfile.file_name().and_then(OsStr::to_str).unwrap()) // TO CHANGE
        .output()
        .expect("Failed to compile main.exe");

    println!("status: {}", output.status);
    //io::stdout().write_all(&output.stdout).unwrap();
    //io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

    // for f in *.m4a; do ffmpeg -i "$f" -codec:v copy -codec:a libmp3lame -q:a 2 newfiles/"${f%.m4a}.mp3"; done
}

fn main() -> io::Result<()> {
    //let args: Vec<String> = env::args().collect();

    //let path_string = &args[1];
    //let filename = &args[2];

    let path_string = String::from("/home/john/Music/VanMusicTest");

    println!("Searching im {}", path_string);

    let path = PathBuf::from(path_string);

    parse_files(&path, &process_file)?;

    Ok(())
}

fn parse_files<'b, 'a>(dir: &'b Path, convert_file: &dyn Fn(&'a DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                parse_files(&path, convert_file)?;
            } else {
                process_file(&entry);
            }
        }
    }

    Ok(())
}
