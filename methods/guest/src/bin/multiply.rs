#![no_main]
#![no_std]
#![feature(core_intrinsics)]
use risc0_zkvm_guest::env;
extern crate zxcvbn;
use zxcvbn::zxcvbn;
use core::intrinsics::abort;

risc0_zkvm_guest::entry!(main);
#[no_mangle]
extern "C" fn __rust_start_panic() {
    abort()
}
pub fn main() {
    let password: &str = env::read();
    let score: u8 = env::read();
    let estimate = zxcvbn("correcthorsebatfjrifoterystaple", &[]);
    //let result: bool = estimate.score() as u8 == score;
    env::commit(&true);
}
