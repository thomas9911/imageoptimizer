use std::fs::read_dir;
use std::io::Write;
use std::process::Command;

use assert_fs::prelude::*;
use std::path::Path;

const DATA_FOLDER: &'static str = "tests/data";
const CONVERT_FOLDER: &'static str = "tests/converted";

#[test]
fn convert_data_folder() {
    let output = Command::new("cargo")
        .args(&["build", "--release"])
        .output()
        .unwrap();

    std::io::stdout().write_all(&output.stdout).unwrap();
    std::io::stderr().write_all(&output.stderr).unwrap();

    for file in read_dir(DATA_FOLDER).unwrap() {
        let file = file.unwrap();
        let filename = file.file_name().into_string().unwrap();
        for out in to_files(file.path()) {
            let output = command(
                &format!("{}/{}", DATA_FOLDER, filename),
                &format!("{}/{}", CONVERT_FOLDER, out),
            )
            .unwrap();

            if !output.status.success() {
                panic!("{}", String::from_utf8_lossy(&output.stderr))
            }

            std::io::stderr()
                .write_all(&format!("done {}\n", out).as_bytes())
                .unwrap()
        }
    }
}

#[test]
fn convert_from_header_png() {
    run_convert_format_header("png", imgopt_lib::Format::Png)
}

#[test]
fn convert_from_header_webp() {
    run_convert_format_header("webp", imgopt_lib::Format::Webp)
}

#[test]
fn convert_from_header_jpeg() {
    run_convert_format_header("jpeg", imgopt_lib::Format::Jpeg)
}

fn run_convert_format_header(format: &str, img_format: imgopt_lib::Format) {
    let temp = assert_fs::TempDir::new().unwrap();
    let input_file = temp.child("in.txt");
    input_file
        .write_file(Path::new(&format!(
            "{}/{}",
            DATA_FOLDER, "pexels-pixabay-373543.jpg"
        )))
        .unwrap();
    let output_file = temp.child("out.txt");

    let out = command_use_header(
        input_file.to_str().unwrap(),
        output_file.to_str().unwrap(),
        format,
    )
    .unwrap();

    let format = check_format(output_file.to_str().unwrap());

    temp.close().unwrap();

    if !out.status.success() {
        panic!("{}", String::from_utf8_lossy(&out.stderr))
    }

    assert_eq!(format, img_format)
}

fn to_files(path: std::path::PathBuf) -> Vec<String> {
    use std::ffi::OsString;
    fn path_to_string(path: std::path::PathBuf) -> String {
        path.file_name().unwrap().to_str().unwrap().to_string()
    }

    if let Some(x) = path.extension() {
        if [OsString::from("jpg"), OsString::from("png")].contains(&x.to_os_string()) {
            return vec![
                path_to_string(path.with_extension("png")),
                path_to_string(path.with_extension("jpg")),
                path_to_string(path.with_extension("webp")),
            ];
        }
    }

    vec![path_to_string(path)]
}

fn command(input: &str, output: &str) -> Result<std::process::Output, std::io::Error> {
    Command::new(path()).args(&[input, output]).output()
}

fn command_use_header(
    input: &str,
    output: &str,
    format: &str,
) -> Result<std::process::Output, std::io::Error> {
    Command::new(path())
        .args(&["--use-header", "--output-format", format, input, output])
        .output()
}

fn check_format(path: &str) -> imgopt_lib::Format {
    use std::io::Read;

    let mut buffer = [0; 64];
    {
        let mut handle = std::fs::File::open(path).unwrap().take(64);
        handle.read(&mut buffer).unwrap();
    }
    imgopt_lib::Format::from_magic_bytes(&buffer).unwrap()
}

#[cfg(target_os = "windows")]
fn path() -> &'static str {
    "target/release/imgopt.exe"
}

#[cfg(not(target_os = "windows"))]
fn path() -> &'static str {
    "target/release/imgopt"
}
