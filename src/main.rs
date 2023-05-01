use ascii_art::*;
use std::fs::File;
use std::io::Write;

fn main() {
    // Argument parsing -----
    let args: Vec<String> = env::args().collect();
    let config: Config = Config::build(&args).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });
    println!(
        "input_path: {}\noutput_path: {}\nis_reversed: {}",
        config.input_path, config.output_path, config.is_reversed
    );

    // Image processing -----
    let src_path: &str = config.input_path.as_str();
    let src_img: Mat = imgcodecs::imread(src_path, imgcodecs::IMREAD_GRAYSCALE).unwrap();

    let dest_dimensions: Dimensions = (64, 64);
    let dest_img: Mat = resize(&src_img, dest_dimensions);

    let mut dest_vec: Vec<&str> = config.output_path.split('.').collect();
    let dest_len = dest_vec.len();
    dest_vec[dest_len - 1] = "jpg";
    let binding = dest_vec.clone().join(".");
    let dest_path: &str = binding.as_str();
    let arguments: Vector<i32> = Vector::new();
    imgcodecs::imwrite(dest_path, &dest_img, &arguments).unwrap();

    // Image to Text -----
    let mut content: String = "".to_string();
    let mut content_list: &[u8];
    let mut ascii_chars = [
        '@', '&', '%', '$', '#', '/', '?', '!', '+', '=', '~', '-', '.', ' ',
    ];
    if !config.is_reversed {
        ascii_chars.reverse();
    }
    for r in 0..dest_dimensions.0 {
        content_list = dest_img.at_row::<u8>(r).unwrap();
        let content_vec: &Vec<String> = &content_list
            .iter()
            .map(|x| String::from(ascii_chars[(((x.clone() as f32)/256.0)*(ascii_chars.len() as f32)) as usize]))
            .collect();
        let b: String = content_vec.join(" ");
        content.push_str(b.as_str());
        // build for windows: "\r\n", else: '\n' (windows assume '\n'(only) is utf16)
        content.push_str("\r\n");
    }

    // File writing -----
    let out_file: File = File::create(config.output_path.as_str()).unwrap();
    write!(&out_file, "{}", content).unwrap();

    println!("{}", content);

    println!("✔ Successfully converted!\n");
}
