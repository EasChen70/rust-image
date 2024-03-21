use csc411_image::{Read, Rgb, RgbImage, Write};
use array2::Array2;
use array2::IterRowMajor;
use array2::IterColumnMajor;
use std::convert::TryInto;
use std::process;
use std::time::Instant;


use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
//using this to parse through the arguments - taking in a file name
//transpose, flip, rotation, row-major and col-major
struct Args {
    // File 
    file: Option<String>,
    #[clap(short = 't', long="transpose")]
    transpose: bool,
    #[clap(short = 'f', long="flip")]
    flip: Option<String>,
    // Rotate
    #[clap(short = 'r', long="rotate")]
    rotate: Option<u32>,
    // Row major
    #[clap(long = "row-major")]
    row_m: bool,
    // Column major
    #[clap(long = "col-major")]
    col_m: bool,
}

//rotate the image 90 degrees 
//parameters: a 2d vec of rgb, width, height, col_major:bool
//retruns a 2d vec representing the rotated image 
fn rotate_90(array: Vec<Vec<Rgb>>, width: u32, height: u32, col_major: bool) -> Vec<Vec<Rgb>>{
    //destination array2
    let default_rgb = Rgb { red: 0, green: 0, blue: 0 };
    let mut destination = Array2::new(height as usize,width as usize, default_rgb).from_row_major();
    if col_major == true{
        let now = Instant::now();
        //column major iteration 
        for (row, col, &ref pixel) in array.iter_column_major(){
           let new_col = height as usize - row - 1;
            destination[col][new_col] = pixel.clone();
        }
        //prints the time 
        let elapsed = now.elapsed();
        eprintln!("{:.2?}", elapsed);

    }
    //row major iteration 
    else {
        let now = Instant::now();
        for (row, col, &ref pixel) in array.iter_row_major(){
            let new_col = height as usize - row - 1;
            destination[col][new_col] = pixel.clone();
        }
        let elapsed = now.elapsed();
        eprintln!("{:.2?}", elapsed);

    }
    destination
}

//rotates the orginal image 180 degrees 
//parameters: 2d vec of rgb, width, height, col_major: bool
//returns a 2d vec of rotated image 
fn rotate_180(source: Vec<Vec<Rgb>>, width: u32, height: u32, col_major: bool) -> Vec<Vec<Rgb>>{
    let default_rgb = Rgb { red: 0, green: 0, blue: 0 };
    //destination 2d array
    let mut destination = Array2::new(width as usize, height as usize, default_rgb).from_row_major();
    if col_major == true{
        let now = Instant::now();
        //col major iteration
        for (row, col, &ref pixel) in source.iter_column_major(){
            let new_row = width as usize - col - 1;
            let new_col = height as usize - row - 1;
            destination[new_col][new_row] = pixel.clone();
        }
        let elapsed = now.elapsed();
        eprintln!("{:.2?}", elapsed);
    }
    else{
        let now = Instant::now();
        //row major iteration 
        for (row, col, &ref pixel) in source.iter_row_major(){
            let new_row = width as usize - col - 1;
            let new_col = height as usize - row - 1;
            destination[new_col][new_row] = pixel.clone();
        }
        let elapsed = now.elapsed();
        eprintln!("{:.2?}", elapsed);
    }
    destination
}

fn main() {
    //parse through the argumentss 
    let args = Args::parse();
    //making sure that we a valid input file 
    let file_path = match &args.file {
        Some(file) => file.as_str(),
        None => {
            eprintln!("Error: No input file provided.");
            process::exit(1);
        }
    };
    //reject transpose and flip
    if args.transpose{
        eprintln!("Error:Reject Transpose");
        process::exit(1);
    }
    if let Some(_value) = args.flip{
        eprintln!("Error:Reject Flip");
        process::exit(1)
    }
    //making sure we have a valid image file 
    let img = match RgbImage::read(Some(file_path)) {
        Ok(img) => img,
        Err(err) => {
            eprintln!("Error reading image file: {}", err);
            process::exit(1);
        }
    };
    let width = img.width;
    let height = img.height;
    //getting our Rgb image struct and creating our 2d array 
    let array = Array2::with_data(img.width.try_into().unwrap(),img.height.try_into().unwrap(), img.pixels);
    let ppmtrans = array.from_row_major();
    //checks which rotation was passed in
    if let Some(rotate_value) = args.rotate {
        if rotate_value == 90 {
            //calles rotate_90 to rotate orignal image 90 degrees 
            let trans = rotate_90(ppmtrans,width,height,args.col_m);
            //create a 1d vector to make Rgb image struct
            let mut answer = vec![];
            for x in trans.iter_row_major(){
                answer.push(x.2.clone());
            }
            //create a RgbImage struc
            let destin = RgbImage{
                pixels: answer,
                width: img.height,
                height: width, 
                denominator: img.denominator
            };
            //write the image to standard output
           let _=  RgbImage::write(&destin,Some("example.ppm"));
        }
        else if rotate_value == 180{
            //calls rotate_180 to rotate the orignal image 180 degrees 
            let trans = rotate_180(ppmtrans, width, height, args.col_m);
            //create a 1d vector using our 2d array to make Rgb image struct
            let mut answer = vec![];
            for x in trans.iter_row_major(){
            answer.push(x.2.clone());
        }
        //create a Rgb image struct
        let destin = RgbImage{
            pixels: answer,
            width: width,
            height: img.height, 
            denominator: img.denominator
        };
        //write the image to standard output 
        let _ = RgbImage::write(&destin,Some("example.ppm")); //None for stdout, but Some("text.ppm") for file 
        }
        //no other value besides 90 and 180 degree
        else {
            std::process::exit(1)
        }
    } 
    
}
