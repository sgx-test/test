use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use num_bigint::{ToBigInt, RandBigInt};

pub fn serialize_rand_pk_verify_pad() {
    println!("1");
    use num_bigint::{ToBigInt, RandBigInt};

    let mut rng = rand::thread_rng();
    let a = rng.gen_bigint(1000);

    let low = -10000.to_bigint().unwrap();
    let high = 10000.to_bigint().unwrap();
    let b = rng.gen_bigint_range(&low, &high);

    // Probably an even larger number.
    println!("{}", a * b);
}

