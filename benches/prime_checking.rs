#![feature(test)]
use goldbach_conjecture::dynamic_checking;

extern crate test;

use test::Bencher;

#[bench]
fn bench_dyanmic(b: &mut Bencher) {
    b.iter(|| dynamic_checking::can_be_expressed_as_sum_of_primes(1000000000));
}