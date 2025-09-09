use rand::rngs::OsRng;
use rand::TryRngCore;
use num_bigint::BigUint;
use num_traits::{Zero, One};

fn random_biguint(max: &BigUint) -> BigUint {
    let bits = max.bits();
    let bytes = ((bits + 7) / 8) as usize;

    let mut rng = OsRng;

    loop {
        let mut array = vec![0u8; bytes];
        rng.try_fill_bytes(&mut array).expect("Failed to get random bytes");

        let r = BigUint::from_bytes_be(&array);
        if &r <= max {
            return r;
        }
    }
}

fn generate_shares(coeff: &[BigUint], n: u8, p: &BigUint) -> Vec<Share> {
    let mut shares = Vec::new();

    for i in 1..=n {
        let x = BigUint::from(i);
        let mut y = BigUint::zero();

        for c in coeff.iter().rev() {
            y = (y * &x + c) % p;
        }

        shares.push(Share { x: format!("{:x}", x), y: format!("{:x}", y) });
    }

    shares
}

#[derive(Debug)]
struct Share {
    x: String,
    y: String,
}

fn split(secret: &str, n: u8, k: u8, p: &BigUint) -> Vec<Share> {
    if !(1 < k && k <= n) {
        panic!("Invalid threshold");
    }

    let s = BigUint::parse_bytes(secret.trim_start_matches("0x").as_bytes(), 16).unwrap() % p;
    let mut coeff = vec![s];

    for _ in 1..k {
        let r = random_biguint(&(p - BigUint::one()));
        coeff.push(r);
    }

    generate_shares(&coeff, n, p)
}

#[cfg(test)]
mod tests {
    use super::*;
    use num_bigint::BigUint;

    #[test]
    fn it_works() {
        let p = BigUint::parse_bytes(
            b"115792089237316195423570985008687907853269984665640564039457584007908834671663",
            10
        ).unwrap();

        let r = split("0x9f3c0d2e1a4b5c6d7e8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b9c0d", 5, 3, &p);
        println!("Random bigint: {:?}", r);
    }
}
