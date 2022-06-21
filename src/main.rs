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

// use rand;
// use std::env;

fn main() {
    // let mut gen = rand::thread_rng().gen_range(0..10);
    // rand::rngs::OsRng::
    // let mut key: Vec<u8> = repeat(0u8).take(16).collect();
    // gen.fill_bytes(&mut key[..]);
    // let mut nonce: Vec<u8> = repeat(0u8).take(16).collect();
    // gen.fill_bytes(&mut nonce[..]);
    // println!("Key: {}", key.to_base64(STANDARD));
    // println!("Nonce: {}", nonce.to_base64(STANDARD));
    // let mut cipher = aes::ctr(KeySize::KeySize128, &key, &nonce);
    // let secret = "I like Nickelback";
    // let mut output: Vec<u8> = repeat(0u8).take(secret.len()).collect();
    // cipher.process(secret.as_bytes(), &mut output[..]);
    // println!("Ciphertext: {}", output.to_base64(STANDARD));

    use aes::cipher::{
        generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit,
    };
    use aes::Aes128;

    let key = GenericArray::from([0u8; 16]);
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
