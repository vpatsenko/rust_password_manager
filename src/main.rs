mod config;
mod storage;

use storage::storage::Storage;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    //TODO: configuration
    let repo = Storage::Repo::new_storage(String::from("repo.txt"));

    // let name = String::from("name");
    // let login = String::from("login");
    // let password = String::from("password");

    // let res = repo.insert_entry(name, login, password);

    let res = repo.read_by_entry_name("name6".to_string());
    match res {
        Ok(r) => println!("{:?}", r),
        Err(e) => println!("{:?}", e),
    }
}
