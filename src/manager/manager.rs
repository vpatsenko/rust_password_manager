pub mod Manager {

    use std::string;

    use crate::storage::storage::Repository;
    use aes::cipher::generic_array::ArrayLength;
    use aes::cipher::typenum::private::PrivateLogarithm2;
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

        pub fn encrypt(
            &self,
            master_password: String,
            name: String,
            login: String,
            password: String,
        ) -> Result<Vec<u8>, String> {
            // TODO: uncomment line after this one
            let string_to_encrypt = Vec::from(format!(
                "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            ));
            // let string_to_encrypt = Vec::from(format!("{} {} {}\n", name, login, password));

            let blocks = self::Manager::split_bytes_into_blocks_with_padding(string_to_encrypt);

            println!("Trying to encrypt blocks\n");

            self::Manager::encypt_blocks(blocks, "pass".to_string());

            Ok(vec![0u8; 10])
        }

        fn split_bytes_into_blocks_with_padding(bytes: Vec<u8>) -> Vec<Vec<u8>> {
            let iter = (bytes.len() + (16 - 1)) / 16;

            let mut byte_arrays_container: Vec<Vec<u8>> = vec![];

            for i in 1..=iter {
                if i == iter {
                    let mut b: Vec<u8> = vec![0u8; 16];

                    let mut k = 0;
                    for n in &bytes[(16 * (i - 1))..] {
                        b[k] = n.to_owned();
                        k += 1;
                    }

                    byte_arrays_container.push(b);
                    break;
                }
                byte_arrays_container.push(bytes[(16 * (i - 1))..(i * 16)].to_owned());
            }

            println!("iter : {}", iter);
            println!("len: {}", byte_arrays_container[0].len());

            return byte_arrays_container;
        }

        fn to_bytes(v: Vec<u8>) -> [u8; 16] {
            v.try_into().unwrap()
        }

        fn encypt_blocks(blocks: Vec<Vec<u8>>, master_password: String) -> Vec<u8> {
            let hashed_password = md5::compute(master_password);
            let key = GenericArray::from(hashed_password.0);

            let cipher = Aes128::new(&key);

            for block in blocks {
                println!("block size: {}", block.len());
                // let mut block_generic = GenericArray::from_slice(block.as_slice()).to_owned();
                let mut block_generic = GenericArray::from_slice(block.as_slice()).to_owned();
                // let mut block_generic = GenericArray::from([0u8; 16]);

                println!("Encrypting the next block\n");

                println!("block before encryption: {:?}", block_generic);

                cipher.encrypt_block(&mut block_generic);
                println!("encrypted block: {:?}", block_generic);

                cipher.decrypt_block(&mut block_generic);
                println!("decrypted block: {:?}", block_generic);

                println!();
            }

            vec![]
        }
    }
}
