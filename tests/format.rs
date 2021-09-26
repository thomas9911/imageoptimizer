const WEBP_IMAGE: &'static [u8] = include_bytes!("support/simple.webp");
const PNG_IMAGE: &'static [u8] = include_bytes!("support/simple.png");
const JPEG_IMAGE: &'static [u8] = include_bytes!("support/simple.jpg");
const SVG_IMAGE: &'static [u8] = include_bytes!("support/simple.svg");

use imgopt_lib::{Error, Format};

#[test]
fn magic_bytes_webp() {
    let format = Format::from_magic_bytes(WEBP_IMAGE);

    assert_eq!(format, Ok::<_, Error>(Format::Webp))
}

#[test]
fn magic_bytes_png() {
    let format = Format::from_magic_bytes(PNG_IMAGE);

    assert_eq!(format, Ok::<_, Error>(Format::Png))
}

#[test]
fn magic_bytes_jpeg() {
    let format = Format::from_magic_bytes(JPEG_IMAGE);

    assert_eq!(format, Ok::<_, Error>(Format::Jpeg))
}

#[test]
fn magic_bytes_svg() {
    let format = Format::from_magic_bytes(SVG_IMAGE);

    assert_eq!(format, Ok::<_, Error>(Format::Svg))
}

#[test]
fn magic_bytes_error() {
    let format = Format::from_magic_bytes(&[]);

    assert_eq!(
        format,
        Err::<Format, _>(Error::new("format not found for bytes".into()))
    )
}
