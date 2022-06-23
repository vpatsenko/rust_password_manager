pub mod Manager {

    use std::string;

    use crate::storage::storage::Repository;
    use aes::cipher::generic_array::ArrayLength;
    use aes::cipher::{
        generic_array::GenericArray, BlockCipher, BlockDecrypt, BlockEncrypt, KeyInit,
    };
    use aes::Aes128;

    #[derive(Debug)]
    pub struct Manager<'a> {
        pub repo: &'a Repository::Repo,
    }

    impl<'a> Manager<'a> {
        pub fn new_manager(r: &'a Repository::Repo) -> Manager {
            Manager { repo: r }
        }

        pub fn store_entry(
            &self,
            master_password: String,
            name: String,
            login: String,
            password: String,
        ) -> Result<(), String> {
            // TODO: ecnrypt

            match &self.repo.insert_entry(name, login, password) {
                Ok(r) => return Ok(r.to_owned()),
                Err(e) => return Err(e.to_string()),
            };
        }

        fn encrypt(
            &self,
            master_password: String,
            name: String,
            login: String,
            password: String,
        ) -> Result<Vec<u8>, String> {
            Ok(vec![0u8; 10])

            // let string_to_encrypt = format!("{}\t{}\t{}\n", name, login, password).as_bytes();

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
            byte_arrays_container
        }

        fn encypt_blocks(blocks: Vec<Vec<u8>>, master_password: String) -> Vec<u8> {
            let hashed_password = md5::compute(master_password);

            let key = GenericArray::from(hashed_password.0);
            let mut block = GenericArray::from([42u8; 16]);

            let cipher = Aes128::new(&key);
            let block_copy = block.clone();

            for block in blocks {
                // Encrypt block in-place

                let mut block_generic = GenericArray::from(block.try_into().unwrap());

                cipher.encrypt_block(&mut block_generic);
                println!("encrypted block: {:?}", block_generic);

                // And decrypt it back
                cipher.decrypt_block(&mut block_generic);
                println!("decrypted block: {:?}", block_generic);
                // assert_eq!(block, block_copy);
            }

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

            vec![]
        }

        fn to_block_generic(v: Vec<u8>) -> GenericArray<u8, ArrayLength<u8>> {}
    }
}
