extern crate rustfft;
use rustfft::num_complex::Complex;
use rustfft::num_traits::Zero;
use rustfft::FFTplanner;

fn mult_vec(fft_x: Vec<Complex<f64>>, fft_y: Vec<Complex<f64>>) -> Vec<Complex<f64>> {
    let mut result: Vec<Complex<f64>> = Vec::with_capacity(fft_x.len());
    for (i, x) in fft_x.iter().enumerate() {
        let y = fft_y[i];
        let re = x.re * y.re - x.im * y.im;
        let im = x.re * y.im + x.im * y.re;
        result.push(Complex::new(re, im));
    }
    result
}

fn init_vec(x: &str, len: usize) -> Vec<Complex<f64>> {
    let mut fx: Vec<Complex<f64>> = vec![Complex::zero(); len];
    for (i, c) in x.chars().rev().enumerate() {
        fx[i] = Complex::new(c.to_digit(10).unwrap() as f64, 0.);
    }
    fx
}

fn collect_re(fft_inv_output: Vec<Complex<f64>>) -> Vec<u32> {
    let inv_len = fft_inv_output.len();
    let mut z = Vec::<u32>::with_capacity(inv_len);
    for i in fft_inv_output {
        z.push((i.re.round() as u32 / inv_len as u32) as u32);
    }
    z
}

fn mult(x: &str, y: &str) -> String {
    let xy_len = x.len() + y.len();
    let fft = FFTplanner::new(false).plan_fft(xy_len);
    let mut fx = init_vec(x, xy_len);
    let mut fy = init_vec(y, xy_len);
    let mut fft_x_output: Vec<Complex<f64>> = vec![Complex::zero(); xy_len];
    let mut fft_y_output: Vec<Complex<f64>> = vec![Complex::zero(); xy_len];
    fft.process(&mut fx, &mut fft_x_output);
    fft.process(&mut fy, &mut fft_y_output);
    let mut fx_fy = mult_vec(fft_x_output, fft_y_output);
    let mut fft_inv_output: Vec<Complex<f64>> = vec![Complex::zero(); xy_len];
    let fft_inv = FFTplanner::new(true).plan_fft(xy_len);
    fft_inv.process(&mut fx_fy, &mut fft_inv_output);
    let mut z = collect_re(fft_inv_output);
    for i in 0..z.len() - 1 {
        z[i + 1] += z[i] / 10;
        z[i] = z[i] % 10
    }
    let mut result = String::from("");
    for s in z.iter().rev() {
        result.push_str(&s.to_string());
    }
    result
}

fn main() {
    let result = mult("987324", "23487");
    assert_eq!("23189278788", &result);
}
