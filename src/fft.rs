use num::Complex;
use std::f64::consts::PI;

/// atc fft_c
/// ```
/// use useful_function::fft::multiply; 
/// fn main() {
///     let n = 4;
///     let p = vec![(1,1), (2, 2), (3, 4), (4,8)];
///     let mut a = vec![];
///     let mut b = vec![];
///     a.push(0);
///     b.push(0);
/// 
///     for i in 0..n {
///         a.push(p[i].0);
///         b.push(p[i].1);
///     }
///     let ans = multiply(a, b);
/// 
///     for i in 0..2*n {
///         println!("{}", ans[i+1]);
///     }
/// }
/// ```
/// 

/// g, hは所有権が移動するので注意!
/// g, hは多項式のvector表現
pub fn multiply(g: Vec<usize>, h: Vec<usize>) -> Vec<isize> {
    let n = pow_2_at_least(g.len() + h.len() + 1);

    let mut a = vec![Complex::new(0.0, 0.0); n];
    let mut b = vec![Complex::new(0.0, 0.0); n];

    for i in 0..g.len() {
        a[i] = Complex::new(g[i] as f64, 0.0);
    }
    for i in 0..h.len() {
        b[i] = Complex::new(h[i] as f64, 0.0);
    }

    let gg = dft(a, n);
    let hh = dft(b, n);

    let mut ff = vec![];
    for i in 0..n {
        ff.push(gg[i] * hh[i]);
    }
    let ff = inverse_dft(ff, n);
    let mut res = vec![];
    for &c in &ff {
        res.push((c.re / n as f64).round() as isize);
    }
    res
}

fn dft(mut f: Vec<Complex<f64>>, n: usize) -> Vec<Complex<f64>> {
    if n == 1 {
        return f;
    }
    let mut f0 = vec![];
    let mut f1 = vec![];
    for i in 0..n / 2 {
        f0.push(f[2 * i + 0]);
        f1.push(f[2 * i + 1]);
    }
    f0 = dft(f0, n / 2);
    f1 = dft(f1, n / 2);
    let zeta = Complex::new((2.0 * PI / n as f64).cos(), (2.0 * PI / n as f64).sin());
    let mut pow_zeta = Complex::new(1.0, 0.0);

    for i in 0..n {
        f[i] = f0[i % (n / 2)] + pow_zeta * f1[i % (n / 2)];
        pow_zeta *= zeta;
    }
    f
}

fn inverse_dft(f: Vec<Complex<f64>>, n: usize) -> Vec<Complex<f64>> {
    if n == 1 {
        return f;
    }
    let mut f0 = vec![];
    let mut f1 = vec![];
    for i in 0..n / 2 {
        f0.push(f[2 * i + 0]);
        f1.push(f[2 * i + 1]);
    }
    f0 = inverse_dft(f0, n / 2);
    f1 = inverse_dft(f1, n / 2);
    let zeta = Complex::new((2.0 * PI / n as f64).cos(), (2.0 * PI / n as f64).sin());
    let zeta = zeta.inv();
    let mut pow_zeta = Complex::new(1.0, 0.0);

    let mut res = vec![];
    for i in 0..n {
        res.push(f0[i % (n / 2)] + pow_zeta * f1[i % (n / 2)]);
        pow_zeta *= zeta;
    }
    res
}


fn pow_2_at_least(n: usize) -> usize {
    let mut sz = 1;
    while sz <= n {
        sz *= 2;
    }
    sz
}
