use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

const MAX_COLOR_VALUE: i32 = 255;

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        let pixels = vec![Color::new(0., 0., 0.); width * height];

        Self {
            width,
            height,
            pixels,
        }
    }

    pub fn write_pixel(&mut self, x: usize, y: usize, color: Color) {
        self.pixels[x + y * self.width] = color;
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> Color {
        self.pixels[x + y * self.width]
    }

    pub fn to_ppm(&self) -> String {
        let ppm_header = format!("P3\n{} {}\n{}", self.width, self.height, MAX_COLOR_VALUE);

        let ppm_body: String = self
            .pixels
            .chunks(self.width)
            .map(process_row)
            .collect::<Vec<_>>()
            .join("\n");

        ppm_header + "\n" + &ppm_body
    }
}

fn process_row(row: &[Color]) -> String {
    row.iter()
        .map(|pixel| {
            let scaled_pixel = *pixel * (MAX_COLOR_VALUE as f64);

            let red = scaled_pixel.red.clamp(0., MAX_COLOR_VALUE as f64).round() as i16;
            let green = scaled_pixel.green.clamp(0., MAX_COLOR_VALUE as f64).round() as i16;
            let blue = scaled_pixel.blue.clamp(0., MAX_COLOR_VALUE as f64).round() as i16;

            format!("{} {} {}", red, green, blue)
        })
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn creating_a_canvas() {
        let c = Canvas::new(10, 20);

        assert_eq!(c.width, 10);
        assert_eq!(c.height, 20);
        assert!(c
            .pixels
            .iter()
            .all(|pixel| *pixel == Color::new(0., 0., 0.)))
    }

    #[test]
    fn writing_pixels_to_a_canvas() {
        let mut c = Canvas::new(10, 20);
        let red = Color::new(1., 0., 0.);

        c.write_pixel(2, 3, red);

        assert_eq!(c.pixel_at(2, 3), red);
    }

    #[test]
    fn contructing_the_ppm_header() {
        let c = Canvas::new(5, 3);
        let ppm: String = c.to_ppm();

        let first_3_lines = get_lines(&ppm, 0, 2);

        let expected_header = "P3\n5 3\n255\n";

        assert_eq!(first_3_lines, expected_header)
    }

    #[test]
    fn constructing_the_ppm_pixel_data() {
        let mut c = Canvas::new(5, 3);

        let c1 = Color::new(1.5, 0., 0.);
        let c2 = Color::new(0., 0.5, 0.);
        let c3 = Color::new(-0.5, 0., 1.);

        c.write_pixel(0, 0, c1);
        c.write_pixel(2, 1, c2);
        c.write_pixel(4, 2, c3);

        let ppm: String = c.to_ppm();

        let ppm_body = get_lines(&ppm, 3, 5);
        let expected_body = "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 128 0 0 0 0 0 0 0\n0 0 0 0 0 0 0 0 0 0 0 0 0 0 255\n";

        assert_eq!(ppm_body, expected_body);
    }

    /// Returns the lines in the range [start, end] (inclusive!!!)
    fn get_lines(s: &str, start: usize, end: usize) -> String {
        s.lines()
            .skip(start)
            .take(end - start + 1)
            .fold(String::new(), |a, b| a + b + "\n")
    }
}
