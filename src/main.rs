// mod config;
// mod storage;

// use storage::storage::Storage;

// fn main() {
//     let args: Vec<String> = std::env::args().collect();

//     //TODO: configuration
//     let repo = Storage::Repo::new_storage(String::from("repo.txt"));

//     let name = String::from("name10");
//     let login = String::from("login10");
//     let password = String::from("password10");

//     let res = repo.insert_entry(name, login, password);

//     // let res = repo.read_by_entry_name("name6".to_string());
//     // match res {
//     //     Ok(r) => println!("{:?}", r),
//     //     Err(e) => println!("{:?}", e),
//     // }
// }

// use crypto::aead::{AeadDecryptor, AeadEncryptor};
// use crypto::aes_gcm::AesGcm;
// use rand::Rng;
// use rustc_serialize::hex::FromHex;

// use core::str;
// use crypto::aes::{self, KeySize};
// use crypto::digest::Digest;
// use crypto::hmac::Hmac;
// use crypto::mac::Mac;
// use crypto::sha2::Sha256;
// use crypto::symmetriccipher::SynchronousStreamCipher;

// use rustc_serialize::base64::{ToBase64, STANDARD};
// use rustc_serialize::hex::ToHex;

// use std::iter::repeat;

use std::fmt::Debug;

// use rand;
use md5;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    println!("{:?}", args);

    if args.len() != 2 {
        println!("No master password provided");
        return;
    }

    let password: &String = &args[1];
    let hashed_password = md5::compute(password);
    println!("hashed password: {:?}", hashed_password.0);

    //    let str_hashed_pasword: String = hashed_password.

    use aes::cipher::{
        generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit,
    };
    use aes::Aes128;

    let key = GenericArray::from(hashed_password.0);
    // let key = GenericArray::from([0u8; 16]);
    println!("key: {:?}", key);
    let mut block = GenericArray::from([42u8; 16]);

    // Initialize cipher
    let cipher = Aes128::new(&key);

    let block_copy = block.clone();

    // Encrypt block in-place
    cipher.encrypt_block(&mut block);
    println!("encrypted block: {:?}", block);

    // And decrypt it back
    cipher.decrypt_block(&mut block);
    println!("decrypted block: {:?}", block);
    assert_eq!(block, block_copy);

    // implementation supports parrallel block processing
    // number of blocks processed in parallel depends in general
    // on hardware capabilities
    let mut blocks = [block; 100];
    cipher.encrypt_blocks(&mut blocks);

    for block in blocks.iter_mut() {
        cipher.decrypt_block(block);
        assert_eq!(block, &block_copy);
    }

    cipher.decrypt_blocks(&mut blocks);

    for block in blocks.iter_mut() {
        cipher.encrypt_block(block);
        assert_eq!(block, &block_copy);
    }
}
