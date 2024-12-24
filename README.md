# down2earth
## Inspiration
I came across fhteagle's EarthNow script (https://github.com/fhteagle/EarthNow) while looking for backgrounds for my new Framework laptop.  
I decided to ~~overengineer~~ rewrite it in Rust to make it more platform-agnostic and because I like rewriting stuff in Rust. 
Currently the only missing feature is network type detection, as I haven't found a cross-platform way of doing that yet.  Be careful using this program on limited networks!

The part of the program that downloads and stitches together the cloud maps is based on a Python script by Kevin Keegan which can be found at https://gist.github.com/krkeegan/64e96290eb6569790d230085016501da

## About
down2earth uses Xplanet (https://xplanet.sourceforge.net/) to set your desktop background to a map of the earth,
complete with updating cloud maps.

The cloud images are downloaded from the Space Science and Engineering Center (SSEC) at the university of Wisconsin-Madison (http://re.ssec.wisc.edu).
Their terms can be found at https://www.ssec.wisc.edu/realearth/terms-of-use/.  
Source: SSEC RealEarth, UW-Madison

## Installation
The easiest way to use this program is to clone the repository so that it can also be used as the working directory.

### Dependencies
 - git
 - xplanet
 - cargo

### Building the Project
1. Make sure all dependencies are installed.
2. Clone the directory: `git clone https://github.com/Jenson-42/down2earth` 
3. CD into the repository: `cd down2earth`
4. Build the Rust project: `cargo build --release`

### Usage
I use crontab on Linux Mint to run this program every fifteen minutes. 
This usage guide will assume you want to do the same. 
To run standalone, run `cargo run --release -- --help` in the project directory. 
As of yet this is untested on Windows or Mac, although the wallpaper and battery crates say they do work on both platforms.

Unless you use an API key, your IP address will be limited to 500 megapixels of cloud images per day before they start getting watermarked. 
I'd recommend signing up for a key as it's free and raises that threshold to 1000 megapixels.

1. Edit your crontab file with the command `crontab -e`.
2. Paste in the following line, substituting the values in angle brackets: `00,15,30,45 * * * * RUST_LOG=info <PATH_TO_PROJECT>/target/release/down2earth --width <WALLPAPER_WIDTH> --height <WALLPAPER_HEIGHT> --dir <PATH_TO_PROJECT> --api-key <API_KEY> >/dev/null 2><PATH_TO_PROJECT>/down2earth.log`

This will run the program every 15 minutes, discarding standard output (should be empty anyway) and writing the log output to "down2earth.log"

## License
To comply with the script this program is based upon, EarthNow, this program shall also be distributed under the CC-BY-SA 4.0 (https://creativecommons.org/licenses/by-sa/4.0/) license.  No warranty, guarantee, support or entitlement should be assumed from this.
The base map images I believe are from the NASA Blue Marble collection ( https://visibleearth.nasa.gov/collection/1484/blue-marble ), which should be released in the public domain. 
If you are the copyright holder of any of the image files used in this method, please contact me with a DMCA request and I will be happy to take them down.
