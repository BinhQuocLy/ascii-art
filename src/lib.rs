pub use opencv::prelude::MatTraitConstManual;
pub use opencv::{
    core::{self, Mat, Size, Vector},
    imgcodecs, imgproc,
    prelude::*,
};
pub use std::env;
pub use std::error::Error;
pub use std::fs;
pub use std::process;

pub struct Config {
    pub input_path: String,
    pub output_path: String,
    pub is_reversed: bool
}
impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 5 || args.len() > 6 {
            return Err("Argument count is not valid.");
        }

        let mut input_path = String::from("");
        let mut output_path = String::from("");
        let mut is_reversed = false;

        let special_arg = ["-i", "-o", "-r"];
        for (i, val) in args.iter().enumerate() {
            if special_arg.contains(&val.as_str()) {
                match val.as_str() {
                    "-i" => {
                        input_path = args.get(i + 1).clone().unwrap().to_string();
                    }
                    "-o" => {
                        output_path = args.get(i + 1).clone().unwrap().to_string();
                    }
                    "-r" => {
                        is_reversed = true;
                    }
                    _ => ()
                }
            }
        }
        if input_path.len() == 0 || output_path.len() == 0 {
            return Err("Wrong syntax.");
        }

        Ok(Config {
            input_path,
            output_path,
            is_reversed
        })
    }
}

pub type Dimensions = (i32, i32);

pub fn resize(src_img: &Mat, dimensions: Dimensions) -> Mat {
    const INTER_AREA: i32 = 3;
    let mut dest_img: Mat = src_img.clone();
    let size: Size = core::Size_ {
        width: dimensions.0,
        height: dimensions.1,
    };
    imgproc::resize(src_img, &mut dest_img, size, 0.0, 0.0, INTER_AREA).unwrap();
    dest_img
}

pub fn _hori_flip(img: &Mat) -> Mat {
    let mut destination_arr = Mat::default();
    core::flip(&img, &mut destination_arr, 1).unwrap();
    destination_arr
}
