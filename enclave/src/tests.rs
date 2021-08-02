use std::prelude::v1::*;
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

use num_bigint::{BigInt, Sign, ToBigInt};

pub fn serialize_rand_pk_verify_pad() {
    println!("1");

    let a = BigInt::from_bytes_be(Sign::Plus, b"AA");
    println!("2");
    let b = BigInt::from_bytes_be(Sign::Plus, b"BB");

    println!("{}", a * b);
}

