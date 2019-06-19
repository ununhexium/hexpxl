extern crate image;

use image::{Pixel, Pixels, RgbImage, ImageBuffer, DynamicImage, GenericImageView, RgbaImage};
use std::f32::consts::PI;
use std::cmp::min;

fn main() {
    let img = image::open("/tmp/screenshot.png").unwrap();
    println!("dimensions {:?}", img.dimensions());

    let pixelized = hex_pixelize(&img, 10.0);

    pixelized.save("out.png").unwrap();
}

fn square_pixelize(img: &DynamicImage, radius: u32) -> RgbaImage {
    let (w, h) = img.dimensions();
    let mut pixelized: RgbaImage = ImageBuffer::new(w, h);

    for (x, y, pixel) in pixelized.enumerate_pixels_mut() {
        *pixel = img.get_pixel(x / radius * radius, y / radius * radius);
    }
    return pixelized;
}

type ImagePoint = (u32, u32);

fn hex_pixelize(img: &DynamicImage, outer_radius: f32) -> RgbaImage {
    let (w, h) = img.dimensions();
    let mut pixelized: RgbaImage = ImageBuffer::new(w, h);

    let r = (outer_radius * (PI / 6.0).cos()) as u32;
    let g = (3.0 * outer_radius / 2.0) as u32;

    for (x, y, pixel) in pixelized.enumerate_pixels_mut() {
        /*

        Illustration in doc/schema.xcf (gimp file)

        On an regular hexagonal grid,
        with an hexagon centered on the origin,
        with 2 of its edges parallel to the Y axis,
        with an outer circle radius R,
        with an inner circle radius r = R cos(PI/6),

        the hexagons to the left and to the right (on the X axis) of the centered hexagon
        have their centers at x = 0, x = 2r, x = 4r etc.
        These positions are referred to as x_0, x_2, x_4 etc.

        The y coordinate is y_0 = 0

        Above and below the line of the hexagons on the X axis, hexagons are shifted by 1r.
        Their centers are at 1r, 3r, 5r etc.
        Those positions are referred to as x_1, x_3, x_5 etc.

        Considering the line above the row on the X axis, the y coordinate is y_1 = 3R/2
        Let the gap g = 3R/2

        Given a point P on the plane. That point's closest hex center will be located on (Hx,Hy)


        How to find Hx and Hy?

        On the X axis, the point will be located between 2 x coordinates, x_low and x_high, with abs(high-low) = 1
        On the Y axis, the point will be located between 2 y coordinates, y_low and y_high, with abs(high-low) = 1

        The closest hex center will be at either of (x_low,y_low), (x_low,y_high), (x_high,y_low) or (x_high,y_high)

        We can notice that given the coordinate system we use, there will never be any hex center on indices with different parities.
        The closest center is therefore either on coordinates which indices have the same parity.

        This reduces the number of points to check to only 2.

        */

        let x_low = x / r;
        let x_high = x / r + 1;

        let y_low = y / g;
        let y_high = y / g + 1;

        let same_parity = ((x_low, y_low), (x_high, y_high));
        let different_parity = ((x_low, y_high), (x_high, y_low));

        let (a, b) = match (x_low % 2 == 0, y_low % 2 == 0) {
            (true, true) => same_parity,
            (false, false) => same_parity,
            (true, false) => different_parity,
            (false, true) => different_parity,
        };

        let (x_index, y_index) = closest(
            (x, y),
            [a, b].to_vec().into_iter().map(|it| (it.0 * r, it.1 * g)).collect()
        );

        println!("x idx = {} - y idx = {}", x_index, y_index);

        *pixel = img.get_pixel(min(x_index, w-1), min(y_index, h-1));
    }
    return pixelized;
}


fn closest((px, py): (u32, u32), o: Vec<ImagePoint>) -> (u32, u32) {
    return o.into_iter().min_by_key(|it| sqr((*it).0 as i32 - px as i32) + sqr((*it).1 as i32 - py as i32)).unwrap();
}

fn sqr(i: i32) -> u32 {
    return (i * i) as u32;
}