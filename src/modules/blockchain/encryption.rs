
pub use crate::utils;

use crate::modules::blockchain::encryption_helpers;

// use rustc_serialize::json;
// use std::collections::HashMap;

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

pub fn decrypt_operation(raw: Vec<u8>) -> Vec<u8> {

    let (key, iv) = encryption_elements();

    let dec_bytes = encryption_helpers::decrypt(&raw[..], key.as_bytes(), iv.as_bytes()).ok().unwrap();

    // let str_dec: String = String::from_utf8_lossy(&dec_bytes).to_string();
    // let result: HashMap<String, String> = json::decode(&str_dec).expect("");

    // return json::decode(&dec_operation).expect("");

    return dec_bytes;
}

pub fn encrypt_operation(message: &str) -> Vec<u8> {

    let (key, iv) = encryption_elements();

    return encryption_helpers::encrypt(message.as_bytes(), key.as_bytes(), iv.as_bytes()).ok().unwrap();
}
