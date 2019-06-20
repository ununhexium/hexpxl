extern crate image;

#[macro_use]
extern crate clap;


use image::{ImageBuffer, DynamicImage, GenericImageView, RgbaImage};
use std::f32::consts::PI;
use clap::{App, Arg};


arg_enum! {
    #[derive(Debug)]
    enum PixelMode {
        sqr,
        hex,
    }
}

fn main() {
    let matches = App::new("hexpxl, a non-square pixelisation tool")
        .version("0.1") // TODO: find how to sync this with cargo.toml
        .author("Christophe '116' Loiseau <116@lab0.net>")
        .about("Pixelises an image using a hexagonal pattern")
        .arg(
            Arg::from_usage("<source> 'Input image path'")
                .required(true)
        )
        .arg(
            Arg::from_usage("<destination> 'Output image path'")
                .required(true)
        )
        .arg(
            Arg::from_usage("<size> 'The size of the pixels, in pixel :P'")
                .default_value("20")
        )
        .arg(
            Arg::from_usage("<mode> 'The PixelMode to use'")
                .help("The pixelisation mode")
                .short("m")
                .long("mode")
                .default_value("hex")
        )
        .get_matches();


    let src = matches.value_of("source").unwrap();
    let dst= matches.value_of("destination").unwrap();
    let size = value_t!(matches, "size", u32).unwrap_or_else(|e| e.exit());
    let mode = value_t!(matches.value_of("mode"), PixelMode).unwrap_or_else(|e| e.exit());

    pixelise(mode, src, dst, size)
}

fn pixelise(mode: PixelMode, src: &str, dst: &str, size: u32) {
    let img = image::open(src).unwrap();

    let pixelised = match mode {
        PixelMode::sqr => square_pixelisation(&img, size),
        PixelMode::hex => hexagon_pixelisation(&img, size),
    };

    pixelised.save(dst).unwrap();
}

fn square_pixelisation(img: &DynamicImage, radius: u32) -> RgbaImage {
    let (w, h) = img.dimensions();
    let mut pixelised: RgbaImage = ImageBuffer::new(w, h);

    for (x, y, pixel) in pixelised.enumerate_pixels_mut() {
        *pixel = img.get_pixel(x / radius * radius, y / radius * radius);
    }
    return pixelised;
}

///
/// Pixelises an image using a hexagonal pattern
///
/// Illustration in doc/schema.xcf (gimp file)
///
/// On an regular hexagonal grid,
/// with an hexagon centered on the origin,
/// with 2 of its edges parallel to the Y axis,
/// with an outer circle radius R,
/// with an inner circle radius r = R cos(PI/6),
///
/// the hexagons to the left and to the right (on the X axis) of the centered hexagon
/// have their centers at x = 0, x = 2r, x = 4r etc.
/// These positions are referred to as x_0, x_2, x_4 etc.
///
/// The y coordinate is y_0 = 0
///
/// Above and below the line of the hexagons on the X axis, hexagons are shifted by 1r.
/// Their centers are at 1r, 3r, 5r etc.
/// Those positions are referred to as x_1, x_3, x_5 etc.
///
/// Considering the line above the row on the X axis, the y coordinate is y_1 = 3R/2
/// Let the gap g = 3R/2
///
/// Given a point P on the plane. That point's closest hex center will be located on (Hx,Hy)
///
///
/// How to find Hx and Hy?
///
/// On the X axis, the point will be located between 2 x coordinates, x_low and x_high, with abs(high-low) = 1
/// On the Y axis, the point will be located between 2 y coordinates, y_low and y_high, with abs(high-low) = 1
///
/// The closest hex center will be at either of (x_low,y_low), (x_low,y_high), (x_high,y_low) or (x_high,y_high)
///
/// We can notice that given the coordinate system we use, there will never be any hex center on indices with different parities.
/// The closest center is therefore either on coordinates which indices have the same parity.
///
/// This reduces the number of points to check to only 2.
///
/// # Arguments
///
/// * `img` - The input image to pixelise
/// * `outer_radius` - The radius of the hexagon's outer circle
///


fn hexagon_pixelisation(img: &DynamicImage, outer_radius: u32) -> RgbaImage {
    let (width, height) = img.dimensions();
    let mut pixelised: RgbaImage = ImageBuffer::new(width, height);

    let inner_radius = (outer_radius as f32 * (PI / 6.0).cos()) as u32;
    let gap = (3.0 * outer_radius as f32 / 2.0) as u32;

    for (x, y, pixel) in pixelised.enumerate_pixels_mut() {
        let x_low = x / inner_radius;
        let x_high = x / inner_radius + 1;

        let y_low = y / gap;
        let y_high = y / gap + 1;

        let (a, b) = match (x_low % 2 == 0) == (y_low % 2 == 0) {
            true => ((x_low, y_low), (x_high, y_high)),
            false => ((x_low, y_high), (x_high, y_low)),
        };

        // find the closest hexagon center point
        let (hx1, hy1) = (a.0 * inner_radius, a.1 * gap);
        let (hx2, hy2) = (b.0 * inner_radius, b.1 * gap);
        let d1 = sqr(hx1 - x) + sqr(hy1 - y);
        let d2 = sqr(hx2 - x) + sqr(hy2 - y);
        let (x_index, y_index) = if d1 < d2 {
            (hx1, hy1)
        } else {
            (hx2, hy2)
        };

        *pixel = img.get_pixel(x_index.min(width - 1), y_index.min(height - 1));
    }
    return pixelised;
}

fn sqr(i: u32) -> u32 {
    return i * i;
}

