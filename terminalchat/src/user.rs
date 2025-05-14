use uuid::Uuid;
use crate::database::Database;

#[derive(Clone)]
pub struct User<'a> {
    id: Uuid,
    username: &'a str,
    password: &'a str,
}

pub fn register(username: &str, password: &str) {
    let new_user = User {
        id: Uuid::new_v4(),
        username: username,
        password: password
    };

    let mut db = Database::new();
    db.insert(new_user);
    println!("Registered new user successfully");
}

pub fn login(username: &str, password: &str){
    println!("username {:?}, password {:?}", username, password);
}

    


