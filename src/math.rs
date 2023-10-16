use bnum::{types::U128, prelude::As};
use rand::{CryptoRng, RngCore, Rng};

pub fn is_prime(n: u64) -> bool {
    if n == 2 || n == 3 {
        return true;
    }

    if n <= 1 || n % 2 == 0 || n % 3 == 0 {
        return false;
    }

    let mut i: u64 = 5;

    while i * i <= n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        
        i += 6;
    }

    true
}

pub fn gen_p_q<T: RngCore + CryptoRng>(mut rng: T) -> (u64, u64){    
    let mut p: u64 = 6;

    while !is_prime(p) {
       p = rng.gen_range(2u64.pow(4u32)..2u64.pow(16u32));
    }
    
    let mut q: u64 = 6;
    while !is_prime(q) {
       q = rng.gen_range(2u64.pow(4u32)..2u64.pow(16u32));

    }

    (p, q)
}

pub fn lcm(a: u64, b: u64) -> u64{
    a*b/gcd(a, b)
}

pub fn gcd(a: u64, b: u64) -> u64{
    if b != 0{
        return gcd(b, a % b);
    }

    a
}

pub fn sam(a: u64, m: u64, n: u64) -> u64 {
    if a == 0 || a == 1 {
        return a;
    }

    let mut x = a;

    let mut mbits = format!("{:b}", m).chars().collect::<Vec<_>>();
    mbits.remove(0);

    for i in mbits{
        match i{
            '0' => {
                x = samzero(x, n)
            },
            '1' => {
                x = samone(x, a, n);
            },
            _ => (),
        }
    }

    x
}

pub fn samzero(x: u64, n: u64) -> u64 {
    let (mut res, overload) = (x % n).overflowing_pow(2);

    if overload {
        let bigx = U128::from_digit(x);
        let bign = U128::from_digit(n);

        res = (bigx.pow(2) % bign).as_();
    }

    return res % n;
}

pub fn samone(x: u64, a: u64, n: u64) -> u64{
    let x = samzero(x, n);

    let (mut res, overload) = (x % n).overflowing_mul(a % n);

    if overload {
        let bigxn = U128::from_digit(x % n);
        let bigan = U128::from_digit(a % n);

        res = (bigxn * bigan).as_();
    }

    return res % n;
}

// Taken from modinverse crate
pub fn egcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        (b, 0, 1)
    }
    else {
        let (g, x, y) = egcd(b % a, a);
        (g, y - (b / a) * x, x)
    }
}

// Taken from modinverse crate
pub fn modinverse2(a: i64, m: i64) -> Option<i64> {
    let (g, x, _) = egcd(a, m);
    if g != 1 {
        None
    }
    else {
        Some((x % m + m) % m)
    }
}