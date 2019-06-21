extern crate image;

#[macro_use]
extern crate clap;


use image::{ImageBuffer, DynamicImage, GenericImageView, RgbaImage};
use std::f32::consts::PI;
use clap::{App, Arg};


macro_rules! sqr {
    ( $a:expr ) => {
        {
            let tmp = $a;
            tmp * tmp
        }
    }
}


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
    let dst = matches.value_of("destination").unwrap();
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
        let x_low_idx = x / inner_radius;
        let x_high_idx = x / inner_radius + 1;

        let y_low_idx = y / gap;
        let y_high_idx = y / gap + 1;

        let (corner_a_idx, corner_b_idx) =
            // do they have the same parity?
            if (x_low_idx % 2 == 0) == (y_low_idx % 2 == 0) {
                ((x_low_idx, y_low_idx), (x_high_idx, y_high_idx))
            } else {
                ((x_low_idx, y_high_idx), (x_high_idx, y_low_idx))
            };

        // first Hx / Hy
        let (corner_a_x, corner_a_y) = (corner_a_idx.0 * inner_radius, corner_a_idx.1 * gap);
        // second Hx / Hy
        let (corner_b_x, corner_b_y) = (corner_b_idx.0 * inner_radius, corner_b_idx.1 * gap);

        let d1 = sqr!(corner_a_x - x) + sqr!(corner_a_y - y);
        let d2 = sqr!(corner_b_x - x) + sqr!(corner_b_y - y);

        let (x_index, y_index) = if d1 < d2 {
            (corner_a_x, corner_a_y)
        } else {
            (corner_b_x, corner_b_y)
        };

        *pixel = img.get_pixel(x_index.min(width - 1), y_index.min(height - 1));
    }
    return pixelised;
}
