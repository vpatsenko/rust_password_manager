// TODO: substitute file by postgres
// pub mod Storage {
// 	use postgres::{Client, Error, NoTls};

// 	// const query_entry: String = String::from("query");
// 	// const insert_entry: String = String::from("insert");

// 	pub struct Repo {
// 		cl: Client,
// 	}

// 	impl Repo {
// 		pub fn new_storage() -> Result<Repo, Error> {
// 			let conn: postgres::Client =
// 				Client::connect("postgresql://postgres@localhost:5432", NoTls)?;
// 			Ok(Repo { cl: conn })
// 		}

// 		//TODO
// 		pub fn insert_row(&self) -> Result<(), Error> {
// 			Ok(())
// 		}

// 		//TODO: query row
// 		pub fn read_by_entry_name(&mut self, entry_name: &String) -> Result<(), Error> {
// 			let rows = self.cl.query("query", &[entry_name])?;

// 			for row in rows {
// 				println!("{:?}", row);
// 			}

// 			Ok(())
// 		}
// 	}
// }

pub mod Storage {
    use std::fs;

    pub struct Entity {
        pub name: String,
        pub login: String,
        pub password: String,
    }

    pub struct Repo {
        file_name: String,
    }

    impl Repo {
        pub fn new_storage(file_name: String) -> Repo {
            Repo { file_name }
        }

        pub fn insert_entry(
            &self,
            name: String,
            login: String,
            password: String,
        ) -> std::io::Result<()> {
            fs::write(
                &self.file_name,
                format!("{}\t{}\t{}\n", name, login, password),
            )?;
            Ok(())
        }

        pub fn read_by_entry_name(&self, name: String) -> std::io::Result<Entity> {
            let read_bytes = fs::read(self.file_name)?;
            let content = std::str::from_utf8(&read_bytes)?;

            Ok(Entity)
        }
    }
}
