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
    use std::{fs, io::Write};

    #[derive(Debug)]
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
            let mut f = fs::OpenOptions::new()
                .write(true)
                .append(true)
                .open(&self.file_name)?;

            f.write_all(format!("{}\t{}\t{}\n", name, login, password).as_bytes())?;
            Ok(())
        }

        pub fn read_by_entry_name(&self, name_to_find: String) -> Result<Entity, String> {
            let read_bytes = fs::read(&self.file_name);
            let content_bytes = match read_bytes {
                Ok(rb) => rb,
                Err(e) => return Err(e.to_string()),
            };

            let content_str = match std::str::from_utf8(&content_bytes) {
                Ok(c) => c,
                Err(e) => return Err(e.to_string()),
            };

            let mut name: String;
            let mut login: String;
            let mut password: String;

            for line in content_str.split("\n") {
                let tabbed_line: Vec<&str> = line.split("\t").collect();

                name = tabbed_line[0].to_string();
                login = tabbed_line[1].to_string();
                password = tabbed_line[2].to_string();

                if name == name_to_find {
                    return Ok(Entity {
                        name: name,
                        login: login,
                        password: password,
                    });
                }
            }

            Err("nothing was found with name: ".to_owned() + &name_to_find)
        }
    }
}
