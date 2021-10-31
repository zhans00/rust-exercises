use std::fmt;

#[derive(PartialEq)]
pub struct Matrix {
    x: (i32, i32),
    y : (i32, i32),
}

pub fn Matrix(r: (i32, i32), c: (i32, i32)) -> Matrix {
    return Matrix {
        x : r,
        y : c,
    };

}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Use `self.number` to refer to each positional data point.
        write!(f, "Matrix(({}, {}), ({}, {}))", self.x.0, self.x.1, self.y.0, self.y.1)
    }
}


pub fn transpose(m: Matrix) -> Matrix {
    let ans = Matrix {
        x : (m.x.0, m.y.0),
        y : (m.x.1, m.y.1),
    };

    return ans;
}
