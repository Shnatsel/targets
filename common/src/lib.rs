extern crate brotli;
extern crate chrono;
extern crate crypto_hashes;
extern crate httparse;
extern crate iso8601;
extern crate proc_macro2;
extern crate regex;
extern crate url;
extern crate bson;

// many function bodies are copied from https://github.com/rust-fuzz/targets

#[inline(always)]
pub fn fuzz_brotli(data: &[u8]) {
    use std::io::{Cursor, Read};

    let mut data_reader = Cursor::new(data);
    let mut result = Vec::with_capacity(data.len());

    let mut de = brotli::Decompressor::new(&mut data_reader, data.len());

    let _ = de.read_exact(&mut result);
}

#[inline(always)]
pub fn fuzz_bson(data: &[u8]) {
    let _ = bson::decode_document(&mut std::io::Cursor::new(data));
}

#[inline(always)]
pub fn fuzz_chrono(data: &[u8]) {
    use chrono::prelude::*;
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = DateTime::parse_from_rfc2822(data);
        let _ = DateTime::parse_from_rfc3339(data);
    }
}

#[inline(always)]
pub fn fuzz_crypto_hashes_blake2b(data: &[u8]) {
    use crypto_hashes::digest::Digest;
    let mut hasher = crypto_hashes::blake2::Blake2b::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_blake2s(data: &[u8]) {
    use crypto_hashes::digest::Digest;
    let mut hasher = crypto_hashes::blake2::Blake2s::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_gost94(data: &[u8]) {
    use crypto_hashes::digest::Digest;
    let mut hasher = crypto_hashes::gost94::Gost94Test::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_md2(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::md2::Md2::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_md4(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::md4::Md4::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_md5(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::md5::Md5::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_ripemd160(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::ripemd160::Ripemd160::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha1(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha1::Sha1::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha2_256(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha2::Sha256::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha2_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha2::Sha512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha3::Sha3_512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_keccak512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::sha3::Keccak512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_shake256(data: &[u8]) {
    use crypto_hashes::digest::{Input, ExtendableOutput};

    let mut hasher = crypto_hashes::sha3::Shake256::default();
    hasher.process(data);
    hasher.xof_result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_streebog_256(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::streebog::Streebog256::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_streebog_512(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::streebog::Streebog512::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_crypto_hashes_sha3_whirlpool(data: &[u8]) {
    use crypto_hashes::digest::Digest;

    let mut hasher = crypto_hashes::whirlpool::Whirlpool::default();
    hasher.input(data);
    hasher.result();
}

#[inline(always)]
pub fn fuzz_httparse_request(data: &[u8]) {
	let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut req = httparse::Request::new(&mut headers);
    let _ = req.parse(data);
}

#[inline(always)]
pub fn fuzz_httparse_response(data: &[u8]) {
	let mut headers = [httparse::EMPTY_HEADER; 16];
    let mut res = httparse::Response::new(&mut headers);
    let _ = res.parse(data);
}

#[inline(always)]
pub fn fuzz_iso8601(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        let _ = iso8601::date(data);
        let _ = iso8601::time(data);
        let _ = iso8601::datetime(data);
    }
}

#[inline(always)]
pub fn fuzz_proc_macro2(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        if let Ok(token_stream) = data.parse::<proc_macro2::TokenStream>() {
            for _ in token_stream { }
        }
    }
}

#[inline(always)]
pub fn fuzz_regex(data: &[u8]) {
    if let Ok(data) = std::str::from_utf8(data) {
        // split data into regular expression and actual input to search through
        use std::cmp::max;
        let len = data.chars().count();
        let split_off_point = max(len / 5, 1) as usize;
        let char_index = data.char_indices().nth(split_off_point);

        if let Some((char_index, _)) = char_index {
            let (pattern, input) = data.split_at(char_index);
            if let Ok(re) = regex::Regex::new(pattern) {
                re.is_match(input);
            }
        }
    }
}

#[inline(always)]
pub fn fuzz_url(data: &[u8]) {
    if let Ok(s) = std::str::from_utf8(data) {
        let _ = url::Url::parse(s);
    }
}
