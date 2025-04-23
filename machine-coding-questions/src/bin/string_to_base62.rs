use num_bigint::BigUint;
use num_traits::{Zero, ToPrimitive};

const BASE62_ALPHABET: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn to_base62(mut num: BigUint) -> String {
    if num.is_zero() {
        return "0".to_string();
    }

    let base = BASE62_ALPHABET.len() as u32;
    let mut result = Vec::new();

    while !num.is_zero() {
        let rem = (&num % base).to_usize().unwrap();
        result.push(BASE62_ALPHABET[rem] as char);
        num /= base;
    }

    result.iter().rev().collect()
}

fn string_to_base62(input: &str) -> String {
    let bytes = input.as_bytes();
    let num = BigUint::from_bytes_be(bytes);
    println!("BigUint: {}", num);
    to_base62(num)
}

fn main() {
    println!("{}", string_to_base62("hello")); // Output: Base62-encoded string
}
