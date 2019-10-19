extern crate base64;

pub use crate::utils;

use crate::modules::blockchain::encryption_helpers;
use rustc_serialize::json;
use std::collections::HashMap;

static FILE_SEED: &str = "/etc/passwd";

fn get_key() -> String {

    return utils::md5_file(FILE_SEED);
}

fn get_vector() -> String {

    let md5_hash: String = utils::md5_file(FILE_SEED);
    let sha256_hash: String = utils::sha256_file(FILE_SEED);

    let chunk_md5 = &md5_hash[0..8];
    let chunk_sha256 = &sha256_hash[0..8];

    let final_str: String = to_string!(chunk_md5) + chunk_sha256;

    return final_str;
}

fn encryption_elements() -> (String, String) {

    let secret_key: String = get_key();
    let secret_iv: String = get_vector();

    // hash
    let key: &str = &utils::sha256_string(&secret_key)[0..32];

    // iv - encrypt method AES-256-CBC expects 16 bytes
    let iv: &str = &utils::sha256_string(&secret_iv)[0..16];

    return (to_string!(key), to_string!(iv));
}

pub fn base64_encode(bytes_array: &Vec<u8>)-> String {

    return base64::encode(&bytes_array);
}

pub fn base64_decode(string_encoded: &str)-> Vec<u8> {

    return base64::decode(string_encoded).ok().unwrap();
}

pub fn decrypt_operation(str_encrypted: &str) -> Vec<u8> {

    let raw: Vec<u8> = base64_decode(&str_encrypted);

    let (key, iv) = encryption_elements();

    let dec_bytes = encryption_helpers::decrypt(&raw[..], key.as_bytes(), iv.as_bytes()).ok().unwrap();

    return dec_bytes;
}

pub fn decrypt_operation_str(str_encrypted: &str) -> String {

    let dec_bytes = decrypt_operation(str_encrypted);

    let str_dec: String = String::from_utf8_lossy(&dec_bytes).to_string();

    return str_dec;
}

pub fn decrypt_operation_object(str_encrypted: &str) -> HashMap<String, String> {

    let dec_bytes = decrypt_operation(str_encrypted);

    let str_dec: String = String::from_utf8_lossy(&dec_bytes).to_string();

    let result: HashMap<String, String> = json::decode(&str_dec).expect("");

    return result;
}

pub fn encrypt_operation(message: &str) -> String {

    let (key, iv) = encryption_elements();

    let bytes: Vec<u8> = encryption_helpers::encrypt(message.as_bytes(), key.as_bytes(), iv.as_bytes()).ok().unwrap();

    return base64_encode(&bytes);
}
