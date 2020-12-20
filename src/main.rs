extern crate clap;
extern crate tar;

use bzip2::read::BzDecoder;
use flate2::read::GzDecoder;
use clap::{Arg, App};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use tar::Archive;
use tar::EntryType;
use sha2::{Digest, Sha256};
use xz2::read::XzDecoder;

fn print_hash<'a, R: io::Read>(file: &mut tar::Entry<'a, R>) -> io::Result<()> {
    let mut sha256 = Sha256::new();
    let mut buffer = [0; 4096];
    loop {
        let count = file.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        sha256.update(&buffer[..count]);
    }

    for b in sha256.finalize() {
        print!("{:02x}", b);
    }
    Ok(())
}

fn process_tar<R: io::Read>(stream: R) -> io::Result<()> {
    let mut a = Archive::new(stream);
    for file in a.entries().unwrap() {
        let mut file = file.unwrap();
        if file.header().entry_type() != EntryType::Regular {
            continue
        }

        let file_name = file.path_bytes();
        let bytes_to_escape = file_name.iter().filter(|&v| v == &b'\n' || v == &b'\\').count();
        if bytes_to_escape > 0 {
            print!("\\");
        }

        print_hash(&mut file)?;
        print!("  ");
        for b in file.path_bytes().iter() {
            match b {
                b'\n' => print!("\\n"),
                b'\\' => print!("\\\\"),
                _     => io::stdout().write_all(&[*b])?
            }
        }
        println!();
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let matches = App::new("tarsum")
        .arg(Arg::with_name("compression")
            .short("c")
            .long("compression")
            .default_value("none")
            .possible_values(&["none", "bzip2", "gzip", "xz"]))
        .arg(Arg::with_name("INPUT")
            .help("Sets the input file to use")
            .index(1))
        .get_matches();

    let reader: Box<dyn io::Read> = match matches.value_of("INPUT") {
        Some(n) => Box::new(File::open(n).unwrap()),
        None => Box::new(io::stdin()),
    };

    let reader_with_decompress: Box<dyn io::Read> = match matches.value_of("compression").unwrap() {
        "xz"   => Box::new(XzDecoder::new(reader)),
        "bzip2" => Box::new(BzDecoder::new(reader)),
        "gzip" => Box::new(GzDecoder::new(reader)),
        _ => reader,
    };

    return process_tar(reader_with_decompress);
}
