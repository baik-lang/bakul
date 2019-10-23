extern crate flate2;
extern crate tar;

use std::fs::File;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use flate2::Compression;
use tar::Archive;
// use std::env;

fn unzip_tar_gz(input_path: &String, output_path: &String) -> Result<(), std::io::Error> {
    let     file    = File     ::open(&input_path)?;
    let     decoder = GzDecoder::new(file);
    let mut archive = Archive  ::new(decoder);
    archive.unpack(&output_path)?;
    Ok(())
}

fn zip_tar_gz(input_path: &String, output_path: &String) -> Result<(), std::io::Error> {
    let     file    = File::create(&output_path)?;
    let     encoder = GzEncoder::new(file, Compression::best());
    let mut tar     = tar::Builder::new(encoder);
    tar.append_dir_all(&output_path, &input_path)?;
    Ok(())
}