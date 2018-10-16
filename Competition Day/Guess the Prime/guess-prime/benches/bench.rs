#![feature(test)]

extern crate guess_prime;
extern crate test;

use guess_prime::*;
use test::Bencher;

#[bench]
fn thousand_primes(b: &mut Bencher) {
    b.iter(|| atkin::sieve(1_000));
}
