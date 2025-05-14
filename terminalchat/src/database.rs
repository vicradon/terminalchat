use crate::user::User;
use crate::room::Room;
use crate::message::Message;
use crate::session::Session;
use std::fs;

pub struct Database<'a> {
    users: Vec<User<'a>>,
    rooms: Vec<Room<'a>>,
    messages: Vec<Message>,
    sessions: Vec<Session>,
}

const database_file:&str = "./db.json";

impl<'b> Database<'_> {
    pub fn insert(&mut self, object: User<'b>) {
        self.users.push(object);
    }

    pub fn get(_key:&str){

    }

    pub fn new() -> Self {
        Self {
            users: Vec::new(),
            rooms: Vec::new(),
            messages: Vec::new(),
            sessions: Vec::new(),
        }
    }
}
