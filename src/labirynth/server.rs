use crate::labirynth::Point;

pub struct Server {
    rows: usize,
    cols: usize,
    data: Vec<Vec<i32>>,
}

impl Server {
    pub fn new(data: Vec<Vec<i32>>) -> Server {
        Server{
            rows: data.len(),
            cols: data[0].len(),
            data
        }
    }

    // Returns size (rows, cols) of the labirynth.
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    /// Returns all points possible for move from passed point.
    pub fn possible_steps(&self, point: Point) -> Vec<Point> {
        let mut data: Vec<Point> = Vec::new();

        if self.valid_point(&point) {
            // top
            if point.y > 0 {
                if self.data[point.y - 1][point.x] == 1 {
                    let p = Point::new(point.x, point.y + 1);
                    data.push(self.update_exit_flag(p));
                }
            }
            // right
            if point.x < (self.cols - 1) {
                if self.data[point.y][point.x] == 1 {
                    let p = Point::new(point.x + 1, point.y);
                    data.push(self.update_exit_flag(p));
                }
            }
            // bottom
            if point.y < (self.rows - 1) {
                if self.data[point.y + 1][point.x] == 1 {
                    let p = Point::new(point.x, point.y + 1);
                    data.push(self.update_exit_flag(p));
                }
            }
            // left
            if point.x > 0 {
                if self.data[point.y][point.x - 1] == 1 {
                    let p = Point::new(point.x - 1, point.y);
                    data.push(self.update_exit_flag(p));
                }
            }
        }
        data
    }

    /// Checks if passed point is inside labirynth.
    pub fn valid_point(&self, p: &Point) -> bool {
        if p.x < self.cols {
            if p.y < self.rows {
                return self.data[p.y][p.x] == 1;
            }
        }
        false
    }

    fn update_exit_flag(&self, mut p: Point) -> Point {
        p.exit = p.x == (self.cols - 1);
        p
    }
}
