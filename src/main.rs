use clap::{load_yaml, App};
use image::io::Reader as ImageReader;
use image::{ImageFormat, Rgba};
use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::Path;
use walkdir::WalkDir;

//TODO: Properly handle multiple input images with the same file name.

fn main() {
    let yaml = load_yaml!("..\\cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let exe_path = env::current_exe().unwrap();
    let exe_path = if fs::symlink_metadata(&exe_path)
        .unwrap()
        .file_type()
        .is_symlink()
    {
        fs::read_link(&exe_path).unwrap()
    } else {
        exe_path
    };

    let output_path = exe_path.parent().unwrap().join("output");
    let output_path = Path::new(
        matches
            .value_of("output")
            .unwrap_or_else(|| output_path.to_str().unwrap()),
    );

    if !output_path.exists() {
        fs::create_dir(&output_path).unwrap();
    }

    let mut paths = Vec::new();

    let mut invalid_paths = Vec::new();

    for path in matches.values_of("PATH").unwrap() {
        let path = Path::new(path);
        if !path.exists() {
            invalid_paths.push(path.to_owned());
        } else if path.is_dir() {
            for entry in WalkDir::new(path) {
                let entry = entry.unwrap();
                if entry.path().is_file() {
                    paths.push(entry.path().to_owned());
                }
            }
        } else if path.is_file() {
            paths.push(path.to_owned());
        }
    }

    if !invalid_paths.is_empty() {
        println!(
            "Skipped {} arguments (paths do not exist):",
            invalid_paths.len()
        );
        for path in &invalid_paths {
            println!("{}", path.display());
        }
        println!();
    }

    let mut excluded_paths = Vec::new();
    let mut included_paths = Vec::new();

    for path in paths {
        let short_path =
            Path::new(path.parent().unwrap().file_name().unwrap()).join(path.file_name().unwrap());

        if let Ok(img) = ImageReader::open(&path)
            .unwrap()
            .with_guessed_format()
            .unwrap()
            .decode()
        {
            let mut img = img.into_rgba8();
            for (_, _, pixel) in img.enumerate_pixels_mut() {
                if *pixel == Rgba([255, 255, 255, 255]) {
                    *pixel = Rgba([255, 255, 255, 0]);
                }
            }

            img.save_with_format(
                output_path
                    .join(path.file_name().unwrap())
                    .with_extension("png"),
                ImageFormat::Png,
            )
            .unwrap();

            included_paths.push(short_path);
        } else {
            excluded_paths.push(short_path);
        }
    }

    if !excluded_paths.is_empty() {
        println!(
            "Skipped {} files (invalid or unsupported image format):",
            excluded_paths.len()
        );
        for path in &excluded_paths {
            println!("{}", path.display());
        }
        println!();
    }

    if !included_paths.is_empty() {
        println!("Processed {} files:", included_paths.len());
        for path in &included_paths {
            println!("{}", path.display());
        }
    } else {
        println!(
            "Processed 0 files\n\n{}\n\nHELP: Drag and drop images onto petz_ih.exe",
            matches.usage()
        );
    }

    if !matches.is_present("no-pause") {
        print!("\nPress enter to exit.");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut String::new()).unwrap();
    }
}
