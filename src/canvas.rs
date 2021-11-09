use crate::color::Color;

pub struct Canvas {
    width: usize,
    height: usize,
    pixels: Vec<Color>,
}

const MAX_COLOR_VALUE: i32 = 255;
const MAX_PPM_LINE_LENGTH: usize = 70;

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

        ppm_header + "\n" + &ppm_body + "\n"
    }
}

fn process_row(row: &[Color]) -> String {
    row.iter()
        .fold((0, String::new()), |accum, color| {
            process_pixel(accum, *color)
        })
        .1
}

/// Split rows of characters if the lines are longer than MAX_PPM_LINE_LENGTH
fn process_pixel(
    (mut char_count, mut result_string): (usize, String),
    pixel: Color,
) -> (usize, String) {
    let scaled_pixel = pixel * (MAX_COLOR_VALUE as f64);

    let red = format_scaled_color(scaled_pixel.red);
    let green = format_scaled_color(scaled_pixel.green);
    let blue = format_scaled_color(scaled_pixel.blue);

    for component in [red, green, blue].iter() {
        if char_count + component.len() + 1 > MAX_PPM_LINE_LENGTH {
            result_string += "\n";
            char_count = 0;
        } else if char_count != 0 {
            result_string += " ";
            char_count += 1;
        }
        result_string += &component;
        char_count += component.len();
    }

    (char_count, result_string)
}

fn format_scaled_color(color_component: f64) -> String {
    format!(
        "{}",
        color_component.clamp(0., MAX_COLOR_VALUE as f64).round() as i16
    )
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

    #[test]
    fn splitting_long_lines_in_ppm_files() {
        let mut c = Canvas::new(10, 2);

        c.pixels.fill(Color::new(1., 0.8, 0.6));

        let ppm = c.to_ppm();

        let ppm_body = get_lines(&ppm, 3, 6);
        let expected_body = "255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
255 204 153 255 204 153 255 204 153 255 204 153 255 204 153 255 204
153 255 204 153 255 204 153 255 204 153 255 204 153
";

        assert_eq!(ppm_body, expected_body);
    }

    #[test]
    fn ppm_files_are_terminated_by_a_newline_character() {
        let c = Canvas::new(5, 3);
        let ppm = c.to_ppm();

        assert_eq!(ppm.chars().last().unwrap(), '\n');
    }

    /// Returns the lines in the range [start, end] (inclusive!!!)
    fn get_lines(s: &str, start: usize, end: usize) -> String {
        s.lines()
            .skip(start)
            .take(end - start + 1)
            .fold(String::new(), |a, b| a + b + "\n")
    }
}
