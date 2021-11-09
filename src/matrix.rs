const SIZE: usize = 4;

pub struct Matrix {
    numbers: [f64; SIZE * SIZE],
}

type Row = [f64; SIZE];

impl Matrix {
    pub fn from_rows(row1: Row, row2: Row, row3: Row, row4: Row) -> Self {
        Self {
            numbers: [
                row1[0], row1[1], row1[2], row1[3], row2[0], row2[1], row2[2], row2[3], row3[0],
                row3[1], row3[2], row3[3], row4[0], row4[1], row4[2], row4[3],
            ],
        }
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.numbers[row * SIZE + col]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn constructing_and_inspecting_a_4x4_matrix() {
        let m = Matrix::from_rows(
            [1., 2., 3., 4.],
            [5.5, 6.5, 7.5, 8.5],
            [9., 10., 11., 12.],
            [13.5, 14.5, 15.5, 16.5],
        );

        assert_eq!(m.get(0, 0), 1.);
        assert_eq!(m.get(0, 3), 4.);
        assert_eq!(m.get(1, 0), 5.5);
        assert_eq!(m.get(1, 2), 7.5);
        assert_eq!(m.get(2, 2), 11.);
        assert_eq!(m.get(3, 0), 13.5);
        assert_eq!(m.get(3, 2), 15.5);
    }
}
