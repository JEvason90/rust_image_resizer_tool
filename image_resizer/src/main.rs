use image;
use image::GenericImageView;
use std::error::Error;
use glob::glob;

fn main() {
    let mut done = false; // mut done: bool

    while !done {
        println!("Shrinking the files... \n");
        resize_images_in_directory();
        done = true;
    }

    println!("Complete");
}

fn resize_images_in_directory() ->  Result<(), Box<dyn Error>> {
    // Consider figuring out how to place in directory
    // // Args arrangement
    // let mut args = std::env::args().skip(1);
    // assert_eq!(args.len(), 1, "Arguments must be: file_location width height");

    // // Reading args
    // let directory = args.next().unwrap();

    //Iterate files in directory

    for entry in glob("./test_images/input/*.jpg")? {
        
        let file = format!(".\\{}", entry?.display());

        let split_dir = file.split('\\');
        
        let vec: Vec<&str> = split_dir.collect();

        let file_split = vec[3].split('.');

        let file_vec: Vec<&str> =  file_split.collect();

        let file_name = file_vec[0];

        // Do the job
        let img = image::open(&file)?;

        let (width, height) = img.dimensions();

        let small_img = img.resize(width/10, height/10, image::imageops::FilterType::Lanczos3);

        let save_file_name = format!(".\\test_images\\output\\{}.png", file_name);

        small_img.save(save_file_name)?;
    }

    //All was ok
    Ok(())
}