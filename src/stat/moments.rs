#![doc="Statistical moments
"]

// std imports
use std::num::Float;

// local imports
use entry::Zero;
use number::Number;
use matrix::matrix::Matrix;
use matrix::traits::{Shape, MatrixBuffer, Strided, 
 StridedNumberMatrix,
 StridedFloatMatrix};
 use matrix::eo::eo_traits::{ERO, ECO};


/// Computes sum over columns and returns a row vector
pub fn sum_cw<T:Number>(m : & StridedNumberMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let mut result = Matrix::new(1, cols);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for c in range(0, cols) {
        let mut sum : T = Zero::zero(); 
        for r in range(0, rows){
            sum = sum + unsafe{*ptr.offset(offset + r as int)};
        }
        offset += stride;
        result.set(0, c, sum);
    }
    result
}

/// Computes sum over rows and returns a column vector
pub fn sum_rw<T:Number>(m : & StridedNumberMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let mut result = Matrix::new(rows, 1);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for r in range(0, rows) {
        let mut sum : T = Zero::zero();
        let mut src_offset  = offset; 
        for _ in range(0, cols){
            sum = sum + unsafe{*ptr.offset(src_offset)};
            src_offset += stride;
        }
        offset += 1;
        result.set(r, 0, sum);
    }
    result
}

/// Computes sum over columns and returns a row vector
pub fn sum_sqr_cw<T:Number>(m : & StridedNumberMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let mut result = Matrix::new(1, cols);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for c in range(0, cols) {
        let mut sum : T = Zero::zero(); 
        for r in range(0, rows){
            let v  = unsafe{*ptr.offset(offset + r as int)};
            sum = sum + v * v;
        }
        offset += stride;
        result.set(0, c, sum);
    }
    result
}

/// Computes sum over rows and returns a column vector
pub fn sum_sqr_rw<T:Number>(m : & StridedNumberMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let mut result = Matrix::new(rows, 1);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for r in range(0, rows) {
        let mut sum : T = Zero::zero();
        let mut src_offset  = offset; 
        for _ in range(0, cols){
            let v = unsafe{*ptr.offset(src_offset)};
            sum = sum + v * v;
            src_offset += stride;
        }
        offset += 1;
        result.set(r, 0, sum);
    }
    result
}


/// Computes mean over columns and returns a row vector
pub fn mean_cw<T:Number+Float+FromPrimitive>(m : & StridedFloatMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let rows_t : T = FromPrimitive::from_uint(rows).unwrap();
    let mut result = Matrix::new(1, cols);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for c in range(0, cols) {
        let mut sum : T = Zero::zero(); 
        for r in range(0, rows){
            sum = sum + unsafe{*ptr.offset(offset + r as int)};
        }
        offset += stride;
        result.set(0, c, sum / rows_t);
    }
    result
}

/// Computes mean over rows and returns a column vector
pub fn mean_rw<T:Number+Float+FromPrimitive>(m : & StridedFloatMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let cols_t : T = FromPrimitive::from_uint(cols).unwrap();
    let mut result = Matrix::new(rows, 1);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for r in range(0, rows) {
        let mut sum : T = Zero::zero();
        let mut src_offset  = offset; 
        for _ in range(0, cols){
            sum = sum + unsafe{*ptr.offset(src_offset)};
            src_offset += stride;
        }
        offset += 1;
        result.set(r, 0, sum / cols_t);
    }
    result
}


/// Computes mean square over columns and returns a row vector
pub fn mean_sqr_cw<T:Number+Float+FromPrimitive>(m : & StridedFloatMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let mut result = Matrix::new(1, cols);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for c in range(0, cols) {
        let mut sum : T = Zero::zero(); 
        for r in range(0, rows){
            let v  = unsafe{*ptr.offset(offset + r as int)};
            sum = sum + v * v;
        }
        offset += stride;
        result.set(0, c, sum);
    }
    let rows_t : T = FromPrimitive::from_uint(m.num_rows()).unwrap();
    result.ero_scale(0, rows_t.powi(-1));
    result
}

/// Computes mean square over rows and returns a column vector
pub fn mean_sqr_rw<T:Number+Float+FromPrimitive>(m : & StridedFloatMatrix<T>) -> Matrix<T> {
    let cols = m.num_cols();
    let rows = m.num_rows();
    let mut result = Matrix::new(rows, 1);
    let ptr = m.as_ptr();
    let stride = m.stride() as int;
    let mut offset = m.start_offset();
    for r in range(0, rows) {
        let mut sum : T = Zero::zero();
        let mut src_offset  = offset; 
        for _ in range(0, cols){
            let v = unsafe{*ptr.offset(src_offset)};
            // sum = sum + sum + v * v;
            sum = v.mul_add(v, sum);
            src_offset += stride;
        }
        offset += 1;
        result.set(r, 0, sum);
    }
    let cols_t : T = FromPrimitive::from_uint(m.num_cols()).unwrap();
    result.eco_scale(0, cols_t.powi(-1));
    result
}


/******************************************************
 *
 *   Unit tests
 *
 *******************************************************/
#[cfg(test)]
mod test{

    use super::*;
    use api::*;


    #[test]
    fn test_moment_sum_cw_1(){
        let m = matrix_rw_i32(3, 3, &[
            1, 2, 3,
            4, 5, 6,
            7, 8, 9]);
        let s = sum_cw(&m);
        assert_eq!(s, matrix_cw_i32(1,3, &[12, 15, 18]));
    }


    #[test]
    fn test_moment_sum_cw_2(){
        let m = matrix_rw_i32(3, 3, &[
            1, 2, 3,
            4, 5, 6,
            7, 8, 9]);
        let v = m.view(1, 1, 2, 2);
        let s = sum_cw(&v);
        assert_eq!(s, matrix_cw_i32(1,2, &[13, 15]));
    }

    #[test]
    fn test_moment_sum_rw_1(){
        let m = matrix_rw_i32(3, 3, &[
            1, 2, 3,
            4, 5, 6,
            7, 8, 9]);
        let s = sum_rw(&m);
        assert_eq!(s, matrix_rw_i32(3,1, &[6, 15, 24]));
    }

    #[test]
    fn test_moment_sum_rw_2(){
        let m = matrix_rw_i32(3, 3, &[
            1, 2, 3,
            4, 5, 6,
            7, 8, 9]);
        let v = m.view(1, 1, 2, 2);
        let s = sum_rw(&v);
        assert_eq!(s, matrix_rw_i32(2,1, &[11, 17]));
    }

    #[test]
    fn test_moment_sum_sqr_cw_1(){
        let m = matrix_rw_i32(3, 3, &[
            1, 1, 2,
            2, 2, 1,
            3, 2, 2]);
        let s = sum_sqr_cw(&m);
        assert_eq!(s, matrix_cw_i32(1,3, &[14, 9, 9]));
    }

    #[test]
    fn test_moment_sum_sqr_cw_2(){
        let m = matrix_rw_i32(3, 3, &[
            1, 1, 2,
            2, 2, 1,
            3, 2, 2]);
        let v = m.view(1, 1, 2, 2);
        let s = sum_sqr_cw(&v);
        assert_eq!(s, matrix_cw_i32(1,2, &[8, 5]));
    }

    #[test]
    fn test_moment_sum_sqr_rw_1(){
        let m = matrix_rw_i32(3, 3, &[
            1, 1, 2,
            2, 2, 1,
            3, 2, 2]);
        let s = sum_sqr_rw(&m);
        assert_eq!(s, matrix_cw_i32(3,1, &[6, 9, 17]));
    }

    #[test]
    fn test_moment_sum_sqr_rw_2(){
        let m = matrix_rw_i32(3, 3, &[
            1, 1, 2,
            2, 2, 1,
            3, 2, 2]);
        let v = m.view(1, 1, 2, 2);
        let s = sum_sqr_rw(&v);
        assert_eq!(s, matrix_cw_i32(2,1, &[5, 8]));
    }

    #[test]
    fn test_moment_mean_cw_1(){
        let m = matrix_rw_f32(3, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.]);
        let s = mean_cw(&m);
        assert_eq!(s, matrix_cw_f32(1,3, &[12./3., 15./3., 18./3.]));
    }


    #[test]
    fn test_moment_mean_cw_2(){
        let m = matrix_rw_f32(3, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.]);
        let v = m.view(1, 1, 2, 2);
        let s = mean_cw(&v);
        assert_eq!(s, matrix_cw_f32(1,2, &[13./2., 15./2.]));
    }

    #[test]
    fn test_moment_mean_rw_1(){
        let m = matrix_rw_f32(3, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.]);
        let s = mean_rw(&m);
        assert_eq!(s, matrix_rw_f32(3,1, &[6./3., 15./3., 24./3.]));
    }

    #[test]
    fn test_moment_mean_rw_2(){
        let m = matrix_rw_f32(3, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.]);
        let v = m.view(1, 1, 2, 2);
        let s = mean_rw(&v);
        assert_eq!(s, matrix_rw_f32(2,1, &[11./2., 17./2.]));
    }

    #[test]
    fn test_moment_mean_sqr_cw_1(){
        let m = matrix_rw_f32(3, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.]);
        let s = mean_sqr_cw(&m);
        assert_eq!(s, matrix_cw_f32(1,3, &[22., 31., 42.]));
    }

    #[test]
    fn test_moment_mean_sqr_cw_2(){
        let m = matrix_rw_f32(3, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            7., 8., 9.]);
        let v = m.view(1, 1, 2, 2);
        let s = mean_sqr_cw(&v);
        assert_eq!(s, matrix_cw_f32(1,2, &[44.5, 58.5]));
    }

    #[test]
    fn test_moment_mean_sqr_rw_1(){
        let m = matrix_rw_f32(4, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            4., 5., 6.,
            7., 8., 9.]);
        let s = mean_sqr_rw(&m);
        let d = s - matrix_cw_f32(4,1, &[14./3., 
            77. / 3., 
            77. / 3., 
            194.0 / 3.0
            ]);
        println!("{:e}", d.max_abs_scalar_value());
        assert!(d.max_abs_scalar_value() < 1e-5);
        // for 64-bit floating point, we can be more accurate.
        //assert!(d.max_abs_scalar_value() < 1e-13);
    }

    #[test]
    fn test_moment_mean_sqr_rw_2(){
        let m = matrix_rw_f32(4, 3, &[
            1., 2., 3.,
            4., 5., 6.,
            4., 5., 6.,
            7., 8., 9.]);
        let v = m.view(1, 1, 2, 2);
        let s = mean_sqr_rw(&v);
        assert_eq!(s, matrix_cw_f32(2,1, &[30.5, 30.5]));
    }

}