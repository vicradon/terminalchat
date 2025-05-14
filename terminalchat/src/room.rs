use crate::user::User;

pub struct Room<'a> {
    pub id: String,
    pub name: String,
    pub users:  Vec<User<'a>>,
}
