use resvg::{tiny_skia, usvg};
use std::{env, fs, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=asset/icon/insight.svg");

    let output_directory = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ico_output_path = output_directory.join("insight.ico");
    let rgba_output_path = output_directory.join("icon.rgba");
    let svg_path = PathBuf::from("asset/icon/insight.svg");

    let svg_data = fs::read(&svg_path).expect("Failed to read SVG asset");
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_data(&svg_data, &options).expect("Failed to parse SVG");

    let width = 256;
    let height = 256;

    let transform = tiny_skia::Transform::from_scale(
        width as f32 / tree.size().width(),
        height as f32 / tree.size().height(),
    );

    let mut pixel_map = tiny_skia::Pixmap::new(width, height).unwrap();
    resvg::render(&tree, transform, &mut pixel_map.as_mut());

    let mut raw_data = pixel_map.data().to_vec();
    for pixel in raw_data.chunks_exact_mut(4) {
        let alpha = f32::from(pixel[3]) / 255.0;
        if alpha > 0.0 && alpha < 1.0 {
            pixel[0] = (f32::from(pixel[0]) / alpha).clamp(0.0, 255.0) as u8;
            pixel[1] = (f32::from(pixel[1]) / alpha).clamp(0.0, 255.0) as u8;
            pixel[2] = (f32::from(pixel[2]) / alpha).clamp(0.0, 255.0) as u8;
        }
    }

    fs::write(&rgba_output_path, &raw_data).expect("Failed to write icon.rgba");

    let image = image::RgbaImage::from_raw(width, height, raw_data)
        .expect("Failed to create RgbaImage from raw data");

    image
        .save_with_format(&ico_output_path, image::ImageFormat::Ico)
        .expect("Failed to save .ico file");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        if ico_output_path.exists() {
            let mut resource = winresource::WindowsResource::new();
            resource.set_icon(ico_output_path.to_str().unwrap());
            resource
                .compile()
                .expect("Failed to compile Windows resource");
        } else {
            println!(
                "cargo:warning=Icon path not found at {}",
                ico_output_path.display()
            );
        }
    }
}
