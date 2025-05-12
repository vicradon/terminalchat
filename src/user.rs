use uuid::Uuid;

pub struct User<'a> {
    id: Uuid,
    username: &'a str,
    password: &'a str,
}

pub fn register(username: &str, password: &str) {
    let _new_user = User {
        id: Uuid::new_v4(),
        username: username,
        password: password
    };
    println!("Registered new user successfully");
}

pub fn login(username: &str, password: &str){
    println!("username {:?}, password {:?}", username, password);
}

    


