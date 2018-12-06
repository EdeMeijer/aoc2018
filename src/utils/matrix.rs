//! Simple 2d Matrix helper that stores data in a contiguous vector and is accessed with (row, col)
//! tuples.

use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
pub struct Matrix<T> {
    buf: Vec<T>,
    pub height: usize,
    pub width: usize,
}

impl<T> Matrix<T> where T: Copy {
    /// Create a new matrix of a custom shape and fill it with an initial value
    pub fn new(height: usize, width: usize, initial: T) -> Matrix<T> {
        Matrix {
            buf: vec![initial; width * height],
            height,
            width,
        }
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, s: (usize, usize)) -> &T {
        assert!(s.0 < self.height);
        assert!(s.1 < self.width);
        &self.buf[s.0 * self.width + s.1]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, s: (usize, usize)) -> &mut T {
        assert!(s.0 < self.height);
        assert!(s.1 < self.width);
        &mut self.buf[s.0 * self.width + s.1]
    }
}

/// Custom Debug trait that prints the 2d matrix over multiple lines
impl<T> Debug for Matrix<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[")?;
        for y in 0..self.height {
            write!(f, "[")?;
            for x in 0..self.width {
                write!(f, "{:?}", self[(y, x)])?;
                if x < self.width - 1 {
                    write!(f, ", ")?;
                }
            }
            write!(f, "]")?;
            if y < self.height - 1 {
                write!(f, "\n ")?;
            }
        }
        write!(f, "]")
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_matrix() {
        let mut m = Matrix::new(3, 4, 0.0);

        m[(0, 0)] = 1.0;
        m[(1, 0)] = 2.0;
        m[(2, 3)] = 3.0;

        assert_eq!(m[(0, 0)], 1.0);
        assert_eq!(m[(0, 1)], 0.0);
        assert_eq!(m[(1, 0)], 2.0);
        assert_eq!(m[(2, 3)], 3.0);

        assert_eq!(
            format!("{:?}", m),
            "[[1.0, 0.0, 0.0, 0.0]
 [2.0, 0.0, 0.0, 0.0]
 [0.0, 0.0, 0.0, 3.0]]"
        )
    }
}
