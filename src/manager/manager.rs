pub mod Manager {

    use crate::storage::storage::Repository;

    #[derive(Debug)]
    pub struct Manager<'a> {
        pub repo: &'a Repository::Repo,
    }

    impl<'a> Manager<'a> {
        pub fn new_manager(r: &'a Repository::Repo) -> Manager {
            Manager { repo: r }
        }

        pub fn encrypt_and_store(
            &self,
            master_password: String,
            name: String,
            login: String,
            password: String,
        ) -> Result<(), String> {
            // TODO: ecnrypt

            match &self.repo.insert_entry(name, login, password) {
                Ok(r) => return Ok(()),
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
        }
    }
}
