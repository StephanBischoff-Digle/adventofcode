use std::{
    default::Default,
    ops::{Add, Mul},
};

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Matrix<T: Copy, const M: usize, const N: usize> {
    m: [[T; N]; M],
}

impl<T, const M: usize, const N: usize> Matrix<T, M, N>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Default,
{
    pub fn set(&mut self, m: usize, n: usize, v: T) {
        assert!(m < M && n < N);
        self.m[m][n] = v;
    }

    pub fn get(&self, m: usize, n: usize) -> T {
        assert!(m < M && n < N);
        self.m[m][n]
    }
}

// For Square Matrix MxM
impl<T, const M: usize> Matrix<T, M, M>
where
    T: Copy + Mul<Output = T> + Add<Output = T> + Default,
{
    pub fn pow(self, n: usize) -> Self {
        let mut acc = self;

        // TODO: implement this for O(log n) instead of O(n)
        for _ in 1..n {
            acc = acc * self;
        }

        acc
    }
}

impl<T, const M: usize, const N: usize> Default for Matrix<T, M, N>
where
    T: Default + Copy,
{
    fn default() -> Self {
        Self {
            m: [[T::default(); N]; M],
        }
    }
}

impl<T, const M: usize, const N: usize, const P: usize> Mul<Matrix<T, N, P>> for Matrix<T, M, N>
where
    T: Mul<Output = T> + Add<Output = T> + Copy + Default,
{
    type Output = Matrix<T, M, { P }>;

    fn mul(self, rhs: Matrix<T, N, { P }>) -> Self::Output {
        let mut o = Matrix::<T, M, P>::default();

        for i in 0..M {
            for j in 0..P {
                let mut acc = T::default();
                for k in 0..N {
                    acc = acc + self.get(i, k) * rhs.get(k, j);
                }
                o.set(i, j, acc);
            }
        }

        o
    }
}

#[test]
fn unit_matrix() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 1);
    a.set(1, 1, 1);

    assert_eq!(a * a, a);
}

#[test]
fn unit_matrix_unit_vector() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 1);
    a.set(1, 1, 1);

    let mut v: Matrix<i32, 2, 1> = Matrix::default();
    v.set(0, 0, 1);
    v.set(1, 0, 1);

    assert_eq!(a * v, v);
}

#[test]
fn unit_matrix_vector() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 1);
    a.set(1, 1, 1);

    let mut v: Matrix<i32, 2, 1> = Matrix::default();
    v.set(0, 0, 2);
    v.set(1, 0, 3);

    assert_eq!(a * v, v);
}

#[test]
fn matrix_unit_vector() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 2);
    a.set(1, 1, 3);

    let mut v: Matrix<i32, 2, 1> = Matrix::default();
    v.set(0, 0, 1);
    v.set(1, 0, 1);

    let mut o: Matrix<i32, 2, 1> = Matrix::default();
    o.set(0, 0, 2);
    o.set(1, 0, 3);

    assert_eq!(a * v, o);
}

#[test]
fn matrix_vector() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 2);
    a.set(1, 1, 3);

    let mut v: Matrix<i32, 2, 1> = Matrix::default();
    v.set(0, 0, 5);
    v.set(1, 0, 4);

    let mut o: Matrix<i32, 2, 1> = Matrix::default();
    o.set(0, 0, 10);
    o.set(1, 0, 12);

    assert_eq!(a * v, o);
}

#[test]
fn matrix_square() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 2);
    a.set(1, 1, 3);

    let mut o: Matrix<i32, 2, 2> = Matrix::default();
    o.set(0, 0, 4);
    o.set(1, 1, 9);

    assert_eq!(a.pow(2), o);
}

#[test]
fn matrix_cubed() {
    let mut a: Matrix<i32, 2, 2> = Matrix::default();
    a.set(0, 0, 2);
    a.set(1, 1, 3);

    let mut o: Matrix<i32, 2, 2> = Matrix::default();
    o.set(0, 0, 8);
    o.set(1, 1, 27);

    assert_eq!(a.pow(3), o);
}
