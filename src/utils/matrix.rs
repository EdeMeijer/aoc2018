//! Simple 2d Matrix helper that stores data in a contiguous vector and is accessed with (row, col)
//! tuples.

use std::fmt::Debug;
use std::fmt::Error;
use std::fmt::Formatter;
use std::ops::Index;
use std::ops::IndexMut;
use std::fmt::Write;
use std::fmt::Display;

#[derive(Clone, Eq, PartialEq)]
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

impl<T> Matrix<T> {
    fn index_of(&self, s: (usize, usize)) -> usize {
        assert!(s.0 < self.height);
        assert!(s.1 < self.width);
        s.0 * self.width + s.1
    }

    pub fn rows(&self) -> impl Iterator<Item=Vec<&T>> {
        let width= self.width;
        (0..self.height)
            .map(move |y| {
                (0..width)
                    .map(|x| &self[(y, x)])
                    .collect()
            })
    }

    pub fn format(
        &self,
        pre: &str,
        line_pre: &str,
        sep: &str,
        line_post: &str,
        line_sep: &str,
        post: &str,
        formatter: fn(&T) -> String
    ) -> Result<String, Error> {
        let mut f = String::new();
        f.write_str(pre)?;
        for y in 0..self.height {
            f.write_str(line_pre)?;
            for x in 0..self.width {
                f.write_str(&formatter(&self[(y, x)]))?;
                if x < self.width - 1 {
                    f.write_str(sep)?;
                }
            }
            f.write_str(line_post)?;
            if y < self.height - 1 {
                f.write_str(line_sep)?;
            }
        }
        f.write_str(post)?;
        Ok(f)
    }
}

impl<T> Matrix<T> where T: Display {
    pub fn format_dense(&self) -> Result<String, Error> {
        self.format("", "", "", "", "\n", "", |x| format!("{}", x))
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, s: (usize, usize)) -> &T {
        let i = self.index_of(s);
        &self.buf[i]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, s: (usize, usize)) -> &mut T {
        let i = self.index_of(s);
        &mut self.buf[i]
    }
}

/// Custom Debug trait that prints the 2d matrix over multiple lines
impl<T> Debug for Matrix<T> where T: Debug {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let s = self.format("[", "[", ", ", "]", "\n ", "]", |x| format!("{:?}", x))?;
        f.write_str(&s)
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
