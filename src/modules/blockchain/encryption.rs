extern crate openssl;

// use openssl::symm::{encrypt, decrypt, Cipher};
use rustc_serialize::json;

pub use crate::utils;

static FILE_SEED: &str = "/etc/passwd";

#[allow(dead_code)]
fn get_key() -> String {

    return utils::md5_file(FILE_SEED);
}

#[allow(dead_code)]
fn get_vector() -> String {

    let md5_hash: String = utils::md5_file(FILE_SEED);
    let sha256_hash: String = utils::sha256_file(FILE_SEED);

    let chunk_md5 = &md5_hash[0..8];
    let chunk_sha256 = &sha256_hash[0..8];

    let final_str: String = to_string!(chunk_md5) + chunk_sha256;

    return final_str;
}

fn data_operation(encrypt_option: bool, text_str: &str) -> String {

    // let secret_key: String = get_key();
    // let secret_iv: String = get_vector();
    //
    // // hash
    // let key: &str = &utils::sha256_string(&secret_key)[0..32];
    //
    // // iv - encrypt method AES-256-CBC expects 16 bytes - else you will get a warning
    // let iv: &str = &utils::sha256_string(&secret_iv)[0..16];
    //
    // let code_object;
    //
    // let cipher = Cipher::aes_256_cbc();
    //
    // if encrypt_option {
    //
    //     code_object = encrypt(cipher, &key.as_bytes(), Some(&iv.as_bytes()), text_str.as_bytes()).unwrap();
    //
    // } else {
    //
    //     code_object = decrypt(cipher, &key.as_bytes(), Some(&iv.as_bytes()), text_str.as_bytes()).unwrap();
    // }
    //
    // let output: String = format!("{:?}", String::from_utf8_lossy(&code_object));

    // println!("{}", output);

    if encrypt_option {
        return to_string!(text_str);
    }

    return to_string!(text_str);
}

pub fn decrypt_operation(str_content: &str) -> String {

    let dec_operation = data_operation(false, str_content);

    return json::decode(&dec_operation).expect("");
}

pub fn encrypt_operation(str_content: &str) -> String {

    return data_operation(true, str_content);
}
