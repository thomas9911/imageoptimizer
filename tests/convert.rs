use std::fs::read_dir;
use std::io::Write;
use std::process::Command;

const DATA_FOLDER: &'static str = "data";
const CONVERT_FOLDER: &'static str = "converted";

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
            command(
                &format!("{}/{}", DATA_FOLDER, filename),
                &format!("{}/{}", CONVERT_FOLDER, out),
            )
            .unwrap();
            std::io::stderr()
                .write_all(&format!("done {}\n", out).as_bytes())
                .unwrap()
        }
    }
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

#[cfg(target_os = "windows")]
fn path() -> &'static str {
    "target/release/imgopt.exe"
}

#[cfg(not(target_os = "windows"))]
fn path() -> &'static str {
    "target/release/imgopt"
}
