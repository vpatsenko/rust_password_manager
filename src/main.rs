mod config;
mod storage;

use storage::storage::Storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //TODO: configuration
    let repo = Storage::Repo::new_storage(String::from("repo.txt"));

    let name = String::from("name");
    let login = String::from("login");
    let password = String::from("password");

    let res = repo.insert_entry(name, login, password);

    match res {
        Ok(res) => println!("{:?}", res),
        Err(res) => println!("{:?}", res),
    }
}
