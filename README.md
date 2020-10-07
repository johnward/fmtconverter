# Tool to convert M4A files to MP3 files
This tool was written to convert M4A files to MP3s. The user provides the base input directory and algorithm will recursivly traverse the directory structure converting all *.m4a files to *.mp3. ffmpeg is used for conversion.

## Dependancies
This application calls ffmpeg to do the conversion, so this needs to be installed on the platform:

$ sudo apt-get update

$ sudo apt-get install ffmpeg

## Usage

    convertmusic [FLAGS] <INPUT_DIRECTORY>

FLAGS:
    -d, --delete     deletes the old value
    -h, --help       Prints help information
    -V, --version    Prints version information

ARGS:
    <INPUT_DIRECTORY>    Sets input directory


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
