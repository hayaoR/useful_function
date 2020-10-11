use std::mem;

pub fn modinv(mut a: isize, m: isize) -> isize {
    let mut b: isize = m;
    let mut u: isize = 1;
    let mut v = 0;

    while b != 0 {
        let t = a / b;
        a -= t * b;
        mem::swap(&mut a, &mut b);
        u -= t * v;
        mem::swap(&mut u, &mut v);
    }

    u %= m;

    if u < 0 {
        u += m;
    }
    u
}

pub fn modpow(mut a: isize, mut n: isize, m: isize) -> isize {
    let mut res = 1;
    while n > 0 {
        if n & 1 != 0 {
            res = res * a % m;
        }
        a = a * a % m;
        n >>= 1;
    }
    res
}

#[test]
fn test_modpow() {
    assert_eq!(modpow(3, 45, 1000000007), 644897553);
}
