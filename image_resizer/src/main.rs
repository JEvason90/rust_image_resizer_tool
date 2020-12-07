use image;
use std::error::Error;
use std::io;
use std::fs::{self, DirEntry};
use std::path::Path;

fn main() -> Result<(), Box<dyn Error>> {
    // Args arrangement
    let mut args = std::env::args().skip(1);
    assert_eq!(args.len(), 3, "Arguments must be: file_location width height");

    // Reading args
    let file_location = args.next().unwrap();
    let width = args.next().unwrap().parse()?;
    let height = args.next().unwrap().parse()?;

    let split = file_location.split('.');

    let vec: Vec<&str> = split.collect();

    let sub_string = vec[1];

    let sub_split = sub_string.split("\\");

    let sub_vec: Vec<&str> = sub_split.collect();
    
    let file_name = sub_vec[3];



    // Do the job
    let img = image::open(&file_location)?;
    let small_img = img.resize(width, height, image::imageops::FilterType::Lanczos3);

    let save_file_name = format!("test_images\\output\\{}.png", file_name);

    small_img.save(save_file_name);

    //All was ok
    Ok(())
}