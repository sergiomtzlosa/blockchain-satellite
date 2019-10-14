
pub use crate::utils;

pub fn get_key() -> String {

    return utils::md5_file("/etc/passwd");
}

pub fn get_vector() -> String {

    let md5_hash: String = utils::md5_file("/etc/passwd");
    let sha256_hash: String = utils::sha256_file("/etc/passwd");

    let chunk_md5 = &md5_hash[0..8];
    let chunk_sha256 = &sha256_hash[0..8];

    let final_str: String = to_string!(chunk_md5) + chunk_sha256;

    return final_str;
}

pub fn data_operation(encrypt: bool, string: &str) -> String {

    return to_string!("");
}

pub fn decrypt(str_content: &str) -> String {

    return to_string!("");
}

pub fn encrypt(str_content: &str) -> String {

    return to_string!("");
}
