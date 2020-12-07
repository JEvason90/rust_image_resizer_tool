use image;
use image::GenericImageView;
use std::error::Error;
use glob::glob;

fn main() {
    println!("shrinking files");
    resize_images_in_directory();
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

        println!("{}",file);
        println!("{}", file_name);

        // Do the job
        let img = image::open(&file)?;

        let (width, height) = img.dimensions();

        let small_img = img.resize(width, height, image::imageops::FilterType::Lanczos3);

        let save_file_name = format!(".\\test_images\\output\\{}.png", file_name);
        
        println!("{}", save_file_name);

        small_img.save(save_file_name)?;
    }

    //All was ok
    Ok(())
}