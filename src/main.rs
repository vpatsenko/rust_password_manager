use md5;
use std::{borrow::Borrow, fmt::Debug, vec};

mod manager;
mod storage;

use manager::manager::Manager;
use storage::storage::Repository;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);

    // if args.len() != 2 {
    //     println!("No master password provided");
    //     return;
    // }

    // let r = Repository::Repo::new_storage("repo.txt".to_string());
    // let m = Manager::Manager::new_manager(&r);

    // println!("{:?}", m);

    let password: &String = &args[1];

    // let name = "name".to_string();
    // let login = "login".to_string();
    let password = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string();
    // let password = "some very very long password".to_string();

    let master_password = "master_password".to_string();

    use aes::cipher::{
        generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit,
    };
    use aes::Aes128;

    // let string_to_encrypt = format!("{}\t{}\t{}\n", name, login, password)
    let string_to_encrypt = format!("{}", password).as_bytes().to_owned();
    println!("byted string to encrypt: {:?}", string_to_encrypt);
    println!("first 16 bytes to encrypt: {:?}", &string_to_encrypt[0..15]);
    println!("second 16 bytes to encrypt: {:?}", &string_to_encrypt[15..]);

    let blocks = split_bytes_into_blocks_with_padding(string_to_encrypt);

    for block in blocks {
        println!("blocks : {:?}", block);
    }

    // let hashed_password = md5::compute(master_password);

    // let key = GenericArray::from(hashed_password.0);
    // let mut block = GenericArray::from([42u8; 16]);

    // let cipher = Aes128::new(&key);
    // let block_copy = block.clone();

    // // Encrypt block in-place
    // cipher.encrypt_block(&mut block);
    // println!("encrypted block: {:?}", block);

    // // And decrypt it back
    // cipher.decrypt_block(&mut block);
    // println!("decrypted block: {:?}", block);
    // assert_eq!(block, block_copy);

    // // implementation supports parrallel block processing
    // // number of blocks processed in parallel depends in general
    // // on hardware capabilities
    // let mut blocks = [block; 100];
    // cipher.encrypt_blocks(&mut blocks);

    // for block in blocks.iter_mut() {
    //     cipher.decrypt_block(block);
    //     assert_eq!(block, &block_copy);
    // }

    // cipher.decrypt_blocks(&mut blocks);

    // for block in blocks.iter_mut() {
    //     cipher.encrypt_block(block);
    //     assert_eq!(block, &block_copy);
    // }
}

fn split_bytes_into_blocks_with_padding(bytes: Vec<u8>) -> Vec<Vec<u8>> {
    let iter = (bytes.len() + (16 - 1)) / 16;

    let mut byte_arrays_container: Vec<Vec<u8>> = vec![];

    for i in 1..=iter {
        if i == iter {
            let mut b: Vec<u8> = vec![0u8; 15];

            let mut k = 0;
            for n in &bytes[(15 * (i - 1))..] {
                b[k] = n.to_owned();
                k += 1;
            }

            byte_arrays_container.push(b);
            break;
        }
        byte_arrays_container.push(bytes[(15 * (i - 1))..(i * 15)].to_owned());
    }

    return byte_arrays_container;
}
